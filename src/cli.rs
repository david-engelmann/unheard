//! UH-11, UH-17, UH-24. The score command. Unheard
//! never writes to the rules file or the tapes. The
//! parse reproduces the Python's argparse behavior
//! for this grammar byte for byte: its help texts,
//! its error shapes, and the order they fire in.
//! Help fires at the end of the token that carries
//! it; a scan error fires when its token is
//! processed and outranks a pending help; extras
//! are only reported after the required check.
//! Argument tokens are carried as text in the
//! operating system's own decoding (fsdecode), so
//! argument bytes survive as the Python's do.

use std::io::Write;
use std::path::PathBuf;

use crate::digest::digest_of;
use crate::emit::{dumps_document, Mode};
use crate::failure::Failure;
use crate::helptext::*;
use crate::inventory::parse_rules;
use crate::pyrepr::repr_text;
use crate::scoring::score;
use crate::tapes::{load_tape_file, TapeLoad};
use crate::util::{decode_utf8_sig, fsdecode_units, scalars_of, stderr_render};
use crate::value::{Text, Value};

/// One argument token: its text, and its decoded
/// scalars for the parser's character work.
struct Token {
    text: Text,
    scalars: Vec<u32>,
}

impl Token {
    fn from_units(units: Vec<u16>) -> Token {
        let scalars = scalars_of(&units);
        Token {
            text: Text::from_units(units),
            scalars,
        }
    }

    fn rendered(&self) -> String {
        stderr_render(self.text.units())
    }
}

/// Whether a token is shaped like an option: it
/// starts with a dash, is not a bare dash, and does
/// not look like a negative number (no option in
/// this grammar takes one, so argparse lets those
/// through as values).
fn option_shaped(token: &Token) -> bool {
    let scalars = &token.scalars;
    if scalars.len() < 2 || scalars[0] != '-' as u32 {
        return false;
    }
    !is_negative_number(&scalars[1..])
}

/// argparse's negative number matcher, the full
/// token pattern ^-\d+$|^-\d*\.\d+$ with \d a
/// Unicode decimal digit: a token matching it
/// whole is a value, never an option.
fn is_negative_number(rest: &[u32]) -> bool {
    let digit = |c: &u32| crate::util::decimal_digit(*c).is_some();
    if !rest.is_empty() && rest.iter().all(digit) {
        return true;
    }
    match rest.iter().position(|c| *c == '.' as u32) {
        Some(dot) => {
            let (before, after) = (&rest[..dot], &rest[dot + 1..]);
            !after.is_empty() && before.iter().all(digit) && after.iter().all(digit)
        }
        None => false,
    }
}

fn scalars_of_str(text: &str) -> Vec<u32> {
    text.chars().map(|ch| ch as u32).collect()
}

/// Split a long-option token at its first '=': the
/// name's scalars, and the glued value's text when
/// an '=' is present.
fn split_long(token: &Token) -> (&[u32], Option<Text>) {
    match token.scalars.iter().position(|c| *c == '=' as u32) {
        Some(at) => {
            let units = token.text.units();
            let unit_at = crate::util::unit_offset(units, at);
            (
                &token.scalars[..at],
                Some(Text::from_units(units[unit_at + 1..].to_vec())),
            )
        }
        None => (&token.scalars[..], None),
    }
}

/// The text of a token from a scalar index on, for
/// the value glued behind a short option letter or
/// an '=' in a short cluster.
fn text_from_scalar(token: &Token, scalar_index: usize) -> Text {
    let units = token.text.units();
    let unit_at = crate::util::unit_offset(units, scalar_index);
    Text::from_units(units[unit_at..].to_vec())
}

/// The outcome of matching a long-option name
/// against a level's option set.
enum LongMatch {
    Exact(&'static str),
    Unique(&'static str),
    Ambiguous(Vec<&'static str>),
    None,
}

fn match_long(name: &[u32], options: &[&'static str]) -> LongMatch {
    let mut candidates: Vec<&'static str> = Vec::new();
    for option in options {
        let full = scalars_of_str(option);
        if full == name {
            return LongMatch::Exact(option);
        }
        if full.len() > name.len() && full[..name.len()] == *name {
            candidates.push(option);
        }
    }
    match candidates.len() {
        0 => LongMatch::None,
        1 => LongMatch::Unique(candidates[0]),
        _ => LongMatch::Ambiguous(candidates),
    }
}

enum ParseOutcome {
    Run(Box<SubParsed>),
    Exit(i32),
}

/// Write bytes to stdout under its lock, reporting
/// only whether the write and its flush landed.
fn write_stdout(bytes: &[u8]) -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(bytes).and_then(|_| handle.flush())
}

/// Write bytes to stderr under its lock, reporting
/// only whether the write and its flush landed.
fn write_stderr(bytes: &[u8]) -> Result<(), std::io::Error> {
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    handle.write_all(bytes).and_then(|_| handle.flush())
}

/// The "error, ..." line's destination: stderr, or
/// stdout when stderr was closed at launch (the
/// Python backs a missing standard stream with the
/// surviving one, so its reports migrate there). A
/// failed write is dropped at this seam; the caller
/// keeps the exit its own rule assigns.
fn report_error(line: &str) {
    let mut bytes = line.as_bytes().to_vec();
    bytes.push(b'\n');
    if crate::util::closed_at_launch(2) {
        let _ = write_stdout(&bytes);
    } else {
        let _ = write_stderr(&bytes);
    }
}

fn parse_error(usage: &str, prog: &str, message: &str) -> ParseOutcome {
    let text = format!("{usage}\n{prog}: error: {message}\n");
    if crate::util::closed_at_launch(2) {
        // With stderr closed at launch the Python's
        // usage line migrates to stdout and its
        // error line is lost in the fallback stream;
        // the exit stays 2.
        let _ = write_stdout(format!("{usage}\n").as_bytes());
        return ParseOutcome::Exit(2);
    }
    match write_stderr(text.as_bytes()) {
        Ok(()) => ParseOutcome::Exit(2),
        // A usage that cannot be delivered collapses
        // to 1 where the Python's shutdown flush
        // failure gives 120 (the flush class, named
        // in the dossier).
        Err(_) => ParseOutcome::Exit(1),
    }
}

/// Print a help text the guarded way: a broken
/// stream is a caught write error with exit 1,
/// never a panic (the shutdown flush class, named
/// in the dossier). With stdout closed at launch
/// the help migrates to stderr and still exits 0,
/// as the Python's does.
fn print_help(text: &str) -> ParseOutcome {
    if crate::util::closed_at_launch(1) {
        return match write_stderr(text.as_bytes()) {
            Ok(()) => ParseOutcome::Exit(0),
            Err(_) => ParseOutcome::Exit(1),
        };
    }
    match write_stdout(text.as_bytes()) {
        Ok(()) => ParseOutcome::Exit(0),
        Err(error) => {
            report_error(&format!("error, {}", os_error_text(&error)));
            ParseOutcome::Exit(1)
        }
    }
}

struct SubParsed {
    positionals: Vec<Text>,
    values: Vec<(&'static str, Text)>,
}

impl SubParsed {
    fn get_text(&self, canonical: &str) -> Option<Text> {
        self.values
            .iter()
            .rev()
            .find_map(|(name, value)| (*name == canonical).then(|| value.clone()))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum OptKind {
    Help,
    Value,
}

struct OptSpec {
    canonical: &'static str,
    label: &'static str,
    kind: OptKind,
    short: Option<char>,
}

struct SubSpec {
    usage: &'static str,
    prog: &'static str,
    help: &'static str,
    positionals: &'static [&'static str],
    options: &'static [OptSpec],
    /// The required arguments in definition order:
    /// (label, is_positional).
    required: &'static [(&'static str, bool)],
}

const HELP_OPT: OptSpec = OptSpec {
    canonical: "--help",
    label: "-h/--help",
    kind: OptKind::Help,
    short: Some('h'),
};

const OUTPUT_OPT: OptSpec = OptSpec {
    canonical: "--output",
    label: "-o/--output",
    kind: OptKind::Value,
    short: Some('o'),
};

const SCORE_SPEC: SubSpec = SubSpec {
    usage: SCORE_USAGE,
    prog: "unheard score",
    help: SCORE_HELP,
    positionals: &["rules", "tapes"],
    options: &[HELP_OPT, OUTPUT_OPT],
    required: &[("rules", true), ("tapes", true)],
};

/// Take the value for an option: a glued form wins,
/// else the next token, which an option-shaped
/// token can never be. On success `index` still
/// points at the option token; the caller advances
/// past both.
fn take_value(
    spec: &SubSpec,
    tokens: &[Token],
    index: &mut usize,
    glued: Option<Text>,
    opt: &OptSpec,
) -> Result<Text, ParseOutcome> {
    if let Some(value) = glued {
        return Ok(value);
    }
    match tokens.get(*index + 1) {
        Some(next) if !option_shaped(next) => {
            *index += 1;
            Ok(next.text.clone())
        }
        _ => Err(parse_error(
            spec.usage,
            spec.prog,
            &format!("argument {}: expected one argument", opt.label),
        )),
    }
}

/// A short cluster token's outcome.
enum ClusterFlow {
    /// The token is an extra.
    Extra,
    /// The token was consumed with nothing recorded.
    Skip,
    /// Values were recorded from the cluster.
    Recorded,
    /// The parse ends here.
    Exit(ParseOutcome),
}

/// Read a short cluster. A first letter the command
/// does not know makes the whole token an extra.
/// Help fires when the token ends, unless a scan
/// error in it fired first; a value letter takes
/// the rest of the cluster as its glued value, or
/// the next token when the cluster ends there.
fn short_cluster(
    spec: &SubSpec,
    tokens: &[Token],
    index: &mut usize,
    values: &mut Vec<(&'static str, Text)>,
) -> ClusterFlow {
    let token = &tokens[*index];
    let cluster = &token.scalars[1..];
    let first = cluster[0] as u8 as char;
    let known = spec.options.iter().any(|opt| opt.short == Some(first));
    if !known {
        return ClusterFlow::Extra;
    }
    let mut help_pending = false;
    let mut recorded = false;
    let mut at = 0usize;
    while at < cluster.len() {
        let letter = cluster[at] as u8 as char;
        if letter == 'h' {
            if cluster.get(at + 1) == Some(&('=' as u32)) {
                let value = text_from_scalar(token, at + 3);
                return ClusterFlow::Exit(parse_error(
                    spec.usage,
                    spec.prog,
                    &format!(
                        "argument -h/--help: ignored explicit argument {}",
                        repr_text(&value)
                    ),
                ));
            }
            help_pending = true;
            at += 1;
            continue;
        }
        let opt = spec
            .options
            .iter()
            .find(|opt| opt.short == Some(letter) && opt.kind != OptKind::Help);
        match opt {
            Some(opt) => {
                let rest = &cluster[at + 1..];
                let glued = if rest.is_empty() {
                    None
                } else if rest[0] == '=' as u32 {
                    Some(text_from_scalar(token, at + 3))
                } else {
                    Some(text_from_scalar(token, at + 2))
                };
                match take_value(spec, tokens, index, glued, opt) {
                    Ok(value) => {
                        values.push((opt.canonical, value));
                        recorded = true;
                    }
                    Err(outcome) => return ClusterFlow::Exit(outcome),
                }
                // A value letter ends the cluster.
                at = cluster.len();
            }
            None => {
                // A letter the command does not know,
                // after known letters: where a help
                // letter already stands in the cluster
                // the help fires (argparse takes the
                // help action as the cluster is read);
                // otherwise argparse reads the whole
                // token as unrecognized.
                if help_pending {
                    return ClusterFlow::Exit(print_help(spec.help));
                }
                return ClusterFlow::Extra;
            }
        }
    }
    if help_pending {
        return ClusterFlow::Exit(print_help(spec.help));
    }
    if recorded {
        return ClusterFlow::Recorded;
    }
    ClusterFlow::Skip
}

fn parse_sub(tokens: &[Token], top_extras: &[Text]) -> ParseOutcome {
    let spec = &SCORE_SPEC;
    let candidates: Vec<&'static str> = spec.options.iter().map(|opt| opt.canonical).collect();
    let mut positionals: Vec<Text> = Vec::new();
    let mut extras: Vec<Text> = top_extras.to_vec();
    let mut values: Vec<(&'static str, Text)> = Vec::new();
    let mut separated = false;
    let mut index = 0usize;

    fn push_positional(
        token: &Text,
        spec: &SubSpec,
        positionals: &mut Vec<Text>,
        extras: &mut Vec<Text>,
    ) {
        if positionals.len() < spec.positionals.len() {
            positionals.push(token.clone());
        } else {
            extras.push(token.clone());
        }
    }

    while index < tokens.len() {
        let token = &tokens[index];
        if separated {
            push_positional(&token.text, spec, &mut positionals, &mut extras);
            index += 1;
            continue;
        }
        if token.scalars == scalars_of_str("--") {
            // The separator is consumed wherever it
            // stands; what follows it is positional,
            // and overflows to the extras on its own.
            separated = true;
            index += 1;
            continue;
        }
        if token.scalars.len() >= 2
            && token.scalars[0] == '-' as u32
            && token.scalars[1] == '-' as u32
        {
            let (name, glued) = split_long(token);
            let chosen = match match_long(name, &candidates) {
                LongMatch::Exact(option) | LongMatch::Unique(option) => option,
                LongMatch::Ambiguous(matches) => {
                    return parse_error(
                        spec.usage,
                        spec.prog,
                        &format!(
                            "ambiguous option: {} could match {}",
                            token.rendered(),
                            matches.join(", ")
                        ),
                    );
                }
                LongMatch::None => {
                    extras.push(token.text.clone());
                    index += 1;
                    continue;
                }
            };
            let opt = spec
                .options
                .iter()
                .find(|opt| opt.canonical == chosen)
                .expect("a spec option");
            if opt.kind == OptKind::Help {
                if let Some(value) = glued {
                    return parse_error(
                        spec.usage,
                        spec.prog,
                        &format!(
                            "argument -h/--help: ignored explicit argument {}",
                            repr_text(&value)
                        ),
                    );
                }
                return print_help(spec.help);
            }
            match take_value(spec, tokens, &mut index, glued, opt) {
                Ok(value) => values.push((opt.canonical, value)),
                Err(outcome) => return outcome,
            }
            index += 1;
            continue;
        }
        if option_shaped(token) {
            match short_cluster(spec, tokens, &mut index, &mut values) {
                ClusterFlow::Extra => {
                    extras.push(token.text.clone());
                    index += 1;
                    continue;
                }
                ClusterFlow::Skip | ClusterFlow::Recorded => {
                    index += 1;
                    continue;
                }
                ClusterFlow::Exit(outcome) => return outcome,
            }
        }
        push_positional(&token.text, spec, &mut positionals, &mut extras);
        index += 1;
    }

    let mut missing: Vec<&str> = Vec::new();
    for (label, is_positional) in spec.required {
        if *is_positional {
            let at = spec
                .positionals
                .iter()
                .position(|name| name == label)
                .expect("a spec positional");
            if positionals.len() <= at {
                missing.push(label);
            }
        } else if !values.iter().any(|(name, _)| name == label) {
            missing.push(label);
        }
    }
    if !missing.is_empty() {
        return parse_error(
            spec.usage,
            spec.prog,
            &format!(
                "the following arguments are required: {}",
                missing.join(", ")
            ),
        );
    }
    if !extras.is_empty() {
        let rendered: Vec<String> = extras.iter().map(|t| stderr_render(t.units())).collect();
        return parse_error(
            TOP_USAGE,
            "unheard",
            &format!("unrecognized arguments: {}", rendered.join(" ")),
        );
    }
    ParseOutcome::Run(Box::new(SubParsed {
        positionals,
        values,
    }))
}

/// Parse the top level: help, the command word, and
/// the held extras an unknown token becomes.
fn parse_top(tokens: &[Token]) -> ParseOutcome {
    let mut extras: Vec<Text> = Vec::new();
    let mut index = 0usize;
    while index < tokens.len() {
        let token = &tokens[index];
        if token.scalars.len() >= 2
            && token.scalars[0] == '-' as u32
            && token.scalars[1] == '-' as u32
            && token.scalars != scalars_of_str("--")
        {
            let (name, glued) = split_long(token);
            match match_long(name, &["--help"]) {
                LongMatch::Exact("--help") | LongMatch::Unique("--help") => {
                    if let Some(value) = glued {
                        return parse_error(
                            TOP_USAGE,
                            "unheard",
                            &format!(
                                "argument -h/--help: ignored explicit argument {}",
                                repr_text(&value)
                            ),
                        );
                    }
                    return print_help(TOP_HELP);
                }
                _ => {
                    extras.push(token.text.clone());
                    index += 1;
                    continue;
                }
            }
        }
        if token.scalars == scalars_of_str("--") || !option_shaped(token) {
            if token.scalars == scalars_of_str("--") && index == tokens.len() - 1 {
                break;
            }
            if token.text.units() == Text::from_str("score").units() {
                return parse_sub(&tokens[index + 1..], &extras);
            }
            return parse_error(
                TOP_USAGE,
                "unheard",
                &format!(
                    "argument command: invalid choice: {} (choose from 'score')",
                    repr_text(&token.text)
                ),
            );
        }
        if token.scalars[1] == 'h' as u32 {
            if token.scalars.get(2) == Some(&('=' as u32)) {
                let value = text_from_scalar(token, 3);
                return parse_error(
                    TOP_USAGE,
                    "unheard",
                    &format!(
                        "argument -h/--help: ignored explicit argument {}",
                        repr_text(&value)
                    ),
                );
            }
            return print_help(TOP_HELP);
        }
        extras.push(token.text.clone());
        index += 1;
    }
    parse_error(
        TOP_USAGE,
        "unheard",
        "the following arguments are required: command",
    )
}

/// The Python's str of a path argument: pathlib's
/// normalization, redundant separators and dot
/// components dropped, a doubled leading separator
/// kept, and an empty path read as the dot.
fn normalize_path(units: &[u16]) -> Vec<u16> {
    let scalars = scalars_of(units);
    let leading = scalars.iter().take_while(|scalar| **scalar == 0x2F).count();
    let mut parts: Vec<&[u32]> = Vec::new();
    for part in scalars.split(|scalar| *scalar == 0x2F) {
        if part.is_empty() || part == [0x2E] {
            continue;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        return if leading > 0 { vec![0x2F] } else { vec![0x2E] }
            .into_iter()
            .map(|unit| unit as u16)
            .collect();
    }
    let mut out: Vec<u32> = Vec::new();
    if leading == 2 {
        out.extend_from_slice(&[0x2F, 0x2F]);
    } else if leading > 0 {
        out.push(0x2F);
    }
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            out.push(0x2F);
        }
        out.extend_from_slice(part);
    }
    crate::util::text_from_scalars(&out).units().to_vec()
}

/// A path resolved the way pathlib's resolve
/// reads: the longest existing prefix is
/// canonicalized, the rest is appended and then
/// normalized lexically, a dot component dropping
/// and a dot-dot popping the component before it,
/// whatever kind of component that is. Resolution
/// fails outright (None) where the interpreter's
/// resolve raises: a symlink loop in any component,
/// the ELOOP the kernel reports, which the output
/// guard reads as the interpreter's crash (UH-17,
/// UH-24).
/// The kernel's symlink-loop errno on this
/// platform family.
const ELOOP: i32 = 40;

fn resolve_units(units: &[u16]) -> Option<Vec<u16>> {
    let raw = crate::util::os_path_from_units(units);
    let absolute = if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join(raw)
    };
    let mut prefix = absolute.clone();
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    let resolved = loop {
        match std::fs::canonicalize(&prefix) {
            Ok(mut resolved) => {
                for part in tail.iter().rev() {
                    resolved.push(part);
                }
                break resolved;
            }
            Err(error) if error.raw_os_error() == Some(ELOOP) => {
                return None;
            }
            Err(_) => {
                let last = prefix.components().next_back();
                match last {
                    Some(component) => {
                        tail.push(component.as_os_str().to_os_string());
                        if !prefix.pop() {
                            break prefix;
                        }
                    }
                    None => break prefix,
                }
            }
        }
    };
    let bytes = crate::util::os_bytes(resolved.as_os_str());
    let units = fsdecode_units(&bytes);
    let scalars = scalars_of(&units);
    let mut parts: Vec<&[u32]> = Vec::new();
    for part in scalars.split(|scalar| *scalar == 0x2F) {
        if part.is_empty() || part == [0x2E] {
            continue;
        }
        if part == [0x2E, 0x2E] {
            parts.pop();
            continue;
        }
        parts.push(part);
    }
    let mut out: Vec<u32> = vec![0x2F];
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            out.push(0x2F);
        }
        out.extend_from_slice(part);
    }
    Some(crate::util::text_from_scalars(&out).units().to_vec())
}

/// Whether one resolved path stands strictly above
/// another: the tapes directory among the output's
/// parents, read component by component.
fn is_ancestor(ancestor: &[u16], path: &[u16]) -> bool {
    let anc: Vec<&[u16]> = ancestor
        .split(|unit| *unit == 0x2F)
        .filter(|part| !part.is_empty())
        .collect();
    let parts: Vec<&[u16]> = path
        .split(|unit| *unit == 0x2F)
        .filter(|part| !part.is_empty())
        .collect();
    anc.len() < parts.len() && parts[..anc.len()] == anc[..]
}

static TEMP_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The mkstemp-shaped write: a sibling temp with
/// the target's name and a dot for a prefix, no
/// suffix, mode 0600, replaced onto the target,
/// and unlinked on any failure.
fn write_atomically(path: &std::path::Path, bytes: &[u8]) -> Result<(), ()> {
    let dir = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    };
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut attempt = 0u32;
    let tmp = loop {
        let candidate = dir.join(format!(
            "{}.{}{}{}",
            name,
            std::process::id(),
            TEMP_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            attempt
        ));
        #[cfg(unix)]
        let opener = {
            use std::os::unix::fs::OpenOptionsExt;
            let mut options = std::fs::OpenOptions::new();
            options.write(true).create_new(true).mode(0o600);
            options
        };
        #[cfg(not(unix))]
        let opener = {
            let mut options = std::fs::OpenOptions::new();
            options.write(true).create_new(true);
            options
        };
        match opener.open(&candidate) {
            Ok(mut file) => match file.write_all(bytes).and_then(|_| file.flush()) {
                Ok(()) => break candidate,
                Err(_) => {
                    let _ = std::fs::remove_file(&candidate);
                    return Err(());
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                attempt += 1;
                if attempt > 64 {
                    return Err(());
                }
            }
            Err(_) => return Err(()),
        }
    };
    match std::fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(_) => {
            let _ = std::fs::remove_file(&tmp);
            Err(())
        }
    }
}

/// The Python's write at the end of _score: write
/// the rendered bytes to stdout or atomically to
/// the output, and on a failure name it with the
/// output as a path.
fn emit(bytes_result: Result<Vec<u8>, crate::emit::EmitError>, out: Option<&Text>) -> i32 {
    let out_path = out.map(|text| crate::util::os_path_from_units(text.units()));
    let bytes = match bytes_result {
        Ok(bytes) => bytes,
        Err(_) => {
            return cannot_write(out);
        }
    };
    match out_path {
        None => {
            if crate::util::closed_at_launch(1) {
                return 1;
            }
            match write_stdout(&bytes) {
                Ok(()) => 0,
                Err(_) => cannot_write(None),
            }
        }
        Some(path) => match write_atomically(&path, &bytes) {
            Ok(()) => 0,
            Err(()) => cannot_write(out),
        },
    }
}

fn cannot_write(out: Option<&Text>) -> i32 {
    match out {
        // The interpreter formats the output as a
        // Path, so the name in this message is the
        // pathlib-normalized form, not the raw
        // argument.
        Some(text) => report_error(&format!(
            "error, cannot write the observations file, {}",
            stderr_render(&normalize_path(text.units()))
        )),
        None => report_error("error, cannot write the observations to stdout"),
    }
    1
}

/// A module failure at the CLI seam: a named
/// refusal after "error, ", and the crash class as
/// a bare exit 1.
fn report_failure(failure: Failure) -> i32 {
    match failure {
        Failure::Named(message) => {
            report_error(&format!("error, {}", message.render_stderr()));
            1
        }
        Failure::Crash => 1,
    }
}

fn named_error(message: &str) -> i32 {
    report_error(&format!("error, {message}"));
    1
}

fn path_error(prefix: &str, units: &[u16]) -> i32 {
    report_error(&format!("error, {prefix}, {}", stderr_render(units)));
    1
}

fn excluded_entry(file: &Text, reason: &str, defects: &[Text]) -> Value {
    Value::Object(vec![
        (Text::from_str("file"), Value::Str(file.clone())),
        (Text::from_str("reason"), Value::Str(Text::from_str(reason))),
        (
            Text::from_str("defects"),
            Value::Array(defects.iter().cloned().map(Value::Str).collect()),
        ),
    ])
}

/// The tapes directory's entries, in the order the
/// Python's sorted listing reads them: by the code
/// points of each entry's decoded name. A listing
/// that cannot be read is the crash class, the
/// interpreter's uncaught OSError.
fn read_entries(dir: &std::path::Path) -> Result<Vec<(Text, PathBuf)>, Failure> {
    let reader = match std::fs::read_dir(dir) {
        Ok(reader) => reader,
        Err(_) => return Err(Failure::Crash),
    };
    let mut entries: Vec<(Vec<u32>, Text, PathBuf)> = Vec::new();
    for entry in reader {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return Err(Failure::Crash),
        };
        let units = fsdecode_units(&crate::util::os_bytes(entry.file_name().as_os_str()));
        let scalars = scalars_of(&units);
        entries.push((scalars, Text::from_units(units), entry.path()));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(entries
        .into_iter()
        .map(|(_, text, path)| (text, path))
        .collect())
}

fn run_score(parsed: &SubParsed) -> i32 {
    let rules_arg = normalize_path(parsed.positionals[0].units());
    let tapes_arg = normalize_path(parsed.positionals[1].units());
    let output = parsed
        .get_text("--output")
        .filter(|text| !text.units().is_empty());
    let rules_path = crate::util::os_path_from_units(&rules_arg);
    let tapes_path = crate::util::os_path_from_units(&tapes_arg);
    if !rules_path.is_file() {
        return path_error("no such rules file", &rules_arg);
    }
    if !tapes_path.is_dir() {
        return path_error("no such tapes directory", &tapes_arg);
    }
    if let Some(out) = &output {
        // The guard resolves all three paths; the
        // interpreter's resolve raises on a symlink
        // loop, uncaught, so an unresolvable path
        // among them is the crash class (UH-17).
        let resolved = (
            resolve_units(out.units()),
            resolve_units(&rules_arg),
            resolve_units(&tapes_arg),
        );
        let (resolved_out, resolved_rules, resolved_tapes) = match resolved {
            (Some(out), Some(rules), Some(tapes)) => (out, rules, tapes),
            _ => return 1,
        };
        if resolved_out == resolved_rules
            || resolved_out == resolved_tapes
            || is_ancestor(&resolved_tapes, &resolved_out)
        {
            return named_error("unheard never writes to its inputs");
        }
    }
    let rules_bytes = match std::fs::read(&rules_path) {
        Ok(bytes) => bytes,
        Err(_) => return path_error("cannot read the rules file", &rules_arg),
    };
    let rules_text = match decode_utf8_sig(&rules_bytes) {
        Some(text) => Text::from_str(&text),
        None => return path_error("cannot read the rules file", &rules_arg),
    };
    let (rules, warnings) = match parse_rules(&rules_text) {
        Ok(pair) => pair,
        Err(failure) => return report_failure(failure),
    };
    let rule_texts: Vec<Text> = rules.iter().map(|rule| rule.text.clone()).collect();
    let entries = match read_entries(&tapes_path) {
        Ok(entries) => entries,
        Err(failure) => return report_failure(failure),
    };
    let mut scored_tapes: Vec<(Value, String)> = Vec::new();
    let mut excluded: Vec<Value> = Vec::new();
    for (name, path) in &entries {
        if !path.is_file() {
            continue;
        }
        match load_tape_file(path) {
            TapeLoad::Failed(reason, defects) => {
                excluded.push(excluded_entry(name, reason, &defects));
            }
            TapeLoad::Loaded(tape) => match digest_of(&tape) {
                Ok(digest) => scored_tapes.push((tape, digest)),
                Err(_) => {
                    excluded.push(excluded_entry(
                        name,
                        "cannot be cited under canonical form 1",
                        &[Text::from_str(
                            "the tape cannot be digested under canonical form 1",
                        )],
                    ));
                }
            },
        }
    }
    let document = score(&rule_texts, &scored_tapes, excluded, &warnings);
    let mode = if output.is_some() {
        Mode::Strict
    } else {
        Mode::Stdout
    };
    emit(dumps_document(&document, mode), output.as_ref())
}

fn os_error_text(err: &std::io::Error) -> String {
    match err.raw_os_error() {
        Some(_) => format!(
            "[Errno {}] {}",
            err.raw_os_error().unwrap_or(0),
            crate::util::strerror_text(err)
        ),
        None => format!("{err}"),
    }
}

/// The command line entry.
pub fn run_cli() -> i32 {
    let tokens: Vec<Token> = std::env::args_os()
        .skip(1)
        .map(|arg| Token::from_units(fsdecode_units(&crate::util::os_bytes(arg.as_os_str()))))
        .collect();
    match parse_top(&tokens) {
        ParseOutcome::Run(parsed) => run_score(&parsed),
        ParseOutcome::Exit(code) => code,
    }
}
