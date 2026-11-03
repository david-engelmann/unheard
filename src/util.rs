//! Shared constructions: argument and path text in
//! CPython's filesystem decoding, the messages the
//! score command's phrases are built from, the
//! strerror table the OS error phrases read, and the
//! Unicode tables the argument matcher and the rules
//! parser consult.

use crate::value::Text;

/// Decode OS bytes the way CPython decodes file system
/// and argument strings: valid UTF-8 decodes normally,
/// and each byte that cannot decode becomes the
/// surrogateescape unit 0xDC00 plus the byte.
pub fn fsdecode_units(bytes: &[u8]) -> Vec<u16> {
    let mut units = Vec::new();
    let mut rest = bytes;
    loop {
        match std::str::from_utf8(rest) {
            Ok(text) => {
                units.extend(text.encode_utf16());
                return units;
            }
            Err(error) => {
                let valid = error.valid_up_to();
                let head = std::str::from_utf8(&rest[..valid]).expect("the valid prefix");
                units.extend(head.encode_utf16());
                match error.error_len() {
                    Some(len) => {
                        for byte in &rest[valid..valid + len] {
                            units.push(0xDC00 + *byte as u16);
                        }
                        rest = &rest[valid + len..];
                    }
                    None => {
                        for byte in &rest[valid..] {
                            units.push(0xDC00 + *byte as u16);
                        }
                        return units;
                    }
                }
            }
        }
    }
}

/// Whether a standard stream was closed at launch.
/// The Rust runtime repairs a missing standard
/// descriptor before main runs by opening /dev/null
/// over it read-write; that substitution is the
/// only ordinary source of a read-write /dev/null
/// on a standard descriptor, and /proc carries both
/// the target and the access mode. A stream the
/// caller aimed at /dev/null itself is opened
/// read-only or write-only and reads as present, as
/// does any file, pipe, or socket.
pub fn closed_at_launch(fd: u32) -> bool {
    let target = match std::fs::read_link(format!("/proc/self/fd/{fd}")) {
        Ok(target) => target,
        Err(_) => return false,
    };
    if target != std::path::Path::new("/dev/null") {
        return false;
    }
    let info = match std::fs::read_to_string(format!("/proc/self/fdinfo/{fd}")) {
        Ok(info) => info,
        Err(_) => return false,
    };
    for line in info.lines() {
        if let Some(flags) = line.strip_prefix("flags:") {
            if let Ok(value) = i64::from_str_radix(flags.trim(), 8) {
                return value & 0o3 == 2;
            }
        }
    }
    false
}

/// The scalar a surrogate pair encodes.
pub fn decode_pair(high: u16, low: u16) -> u32 {
    0x10000 + (((high as u32) - 0xD800) << 10) + ((low as u32) - 0xDC00)
}

/// One step of the decode walk every consumer
/// shares: what the unit at an index holds, and how
/// many units it spans. A valid pair decodes to its
/// astral scalar; a BMP unit is its own scalar; an
/// unpaired surrogate is a lone unit, and each
/// destination applies its own law to it (a raw byte,
/// a replacement, an escape, a refusal).
pub enum Step {
    Scalar(u32),
    Lone(u16),
}

/// The step at `index`, with its span in units.
pub fn step_at(units: &[u16], index: usize) -> (usize, Step) {
    let unit = units[index];
    if (0xD800..=0xDBFF).contains(&unit)
        && index + 1 < units.len()
        && (0xDC00..=0xDFFF).contains(&units[index + 1])
    {
        (2, Step::Scalar(decode_pair(unit, units[index + 1])))
    } else if (0xD800..=0xDFFF).contains(&unit) {
        (1, Step::Lone(unit))
    } else {
        (1, Step::Scalar(unit as u32))
    }
}

/// The decoded scalars of a unit string: surrogate
/// pairs as their astral scalar, lone units at face
/// value.
pub fn scalars_of(units: &[u16]) -> Vec<u32> {
    let mut out = Vec::with_capacity(units.len());
    let mut index = 0;
    while index < units.len() {
        let (span, step) = step_at(units, index);
        out.push(match step {
            Step::Scalar(scalar) => scalar,
            Step::Lone(unit) => unit as u32,
        });
        index += span;
    }
    out
}

/// The inverse: encode text units back to OS bytes,
/// mapping surrogateescape units back to their bytes.
/// A lone surrogate outside the escape range cannot
/// be encoded; it renders as the UTF-8 replacement,
/// a case argument strings from the OS never reach
/// (their surrogates come only from fsdecode).
#[cfg(unix)]
fn fsencode_bytes(units: &[u16]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut index = 0;
    while index < units.len() {
        let (span, step) = step_at(units, index);
        match step {
            Step::Scalar(scalar) => {
                let mut buf = [0u8; 4];
                out.extend_from_slice(
                    char::from_u32(scalar)
                        .expect("a valid scalar")
                        .encode_utf8(&mut buf)
                        .as_bytes(),
                );
            }
            Step::Lone(unit) => {
                if (0xDC80..=0xDCFF).contains(&unit) {
                    out.push((unit - 0xDC00) as u8);
                } else {
                    out.extend_from_slice("\u{FFFD}".as_bytes());
                }
            }
        }
        index += span;
    }
    out
}

/// Render text for a stderr message the way CPython's
/// stderr does (its error handler is backslashreplace):
/// valid text passes through, and every lone surrogate
/// becomes its \uXXXX escape.
pub fn stderr_render(units: &[u16]) -> String {
    let mut out = String::new();
    let mut index = 0;
    while index < units.len() {
        let (span, step) = step_at(units, index);
        match step {
            Step::Scalar(scalar) => {
                out.push(char::from_u32(scalar).expect("a valid scalar"));
            }
            Step::Lone(unit) => {
                out.push_str(&format!("\\u{unit:04x}"));
            }
        }
        index += span;
    }
    out
}

/// A message under construction, held as text units
/// so a name the Python inserts raw (a rules or
/// tapes path, an excluded tape's file name) keeps
/// its exact units until the message is rendered
/// for its destination: stderr renders
/// backslashreplace, stdout surrogateescape.
#[derive(Clone, Debug, PartialEq)]
pub struct Msg(pub Vec<u16>);

#[allow(clippy::new_without_default)]
impl Msg {
    pub fn new() -> Msg {
        Msg(Vec::new())
    }

    pub fn str(text: &str) -> Msg {
        let mut msg = Msg::new();
        msg.push_str(text);
        msg
    }

    pub fn push_str(&mut self, text: &str) {
        self.0.extend(text.encode_utf16());
    }

    pub fn render_stderr(&self) -> String {
        stderr_render(&self.0)
    }
}

/// The OS path for a normalized display path: the
/// fsencoding of its units. The empty path is the
/// current directory, as pathlib's Path("") is
/// Path("."); an empty PathBuf names nothing.
pub fn os_path_from_units(units: &[u16]) -> std::path::PathBuf {
    if units.is_empty() {
        return std::path::PathBuf::from(".");
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        std::path::PathBuf::from(std::ffi::OsString::from_vec(fsencode_bytes(units)))
    }
    #[cfg(not(unix))]
    {
        std::path::PathBuf::from(String::from_utf16_lossy(units))
    }
}

/// The first code point of each block of ten
/// Unicode decimal digits (the Nd category): every
/// block runs 0 to 9 from its start.
const DECIMAL_DIGIT_STARTS: [u32; 68] = [
    0x30, 0x660, 0x6F0, 0x7C0, 0x966, 0x9E6, 0xA66, 0xAE6, 0xB66, 0xBE6, 0xC66, 0xCE6, 0xD66,
    0xDE6, 0xE50, 0xED0, 0xF20, 0x1040, 0x1090, 0x17E0, 0x1810, 0x1946, 0x19D0, 0x1A80, 0x1A90,
    0x1B50, 0x1BB0, 0x1C40, 0x1C50, 0xA620, 0xA8D0, 0xA900, 0xA9D0, 0xA9F0, 0xAA50, 0xABF0, 0xFF10,
    0x104A0, 0x10D30, 0x11066, 0x110F0, 0x11136, 0x111D0, 0x112F0, 0x11450, 0x114D0, 0x11650,
    0x116C0, 0x11730, 0x118E0, 0x11950, 0x11C50, 0x11D50, 0x11DA0, 0x11F50, 0x16A60, 0x16AC0,
    0x16B50, 0x1D7CE, 0x1D7D8, 0x1D7E2, 0x1D7EC, 0x1D7F6, 0x1E140, 0x1E2F0, 0x1E4F0, 0x1E950,
    0x1FBF0,
];

/// A scalar's Unicode decimal digit value, where it
/// has one: the digit set CPython's float() and
/// argparse's negative number matcher read.
pub fn decimal_digit(scalar: u32) -> Option<u32> {
    let at = DECIMAL_DIGIT_STARTS.partition_point(|start| *start <= scalar);
    if at == 0 {
        return None;
    }
    let start = DECIMAL_DIGIT_STARTS[at - 1];
    let offset = scalar - start;
    if offset < 10 {
        Some(offset)
    } else {
        None
    }
}

/// CPython's whitespace set, shared by str.strip()
/// and the re module's \s: the ASCII whitespace and
/// file separators plus the Unicode space characters.
pub(crate) fn py_is_space(scalar: u32) -> bool {
    matches!(scalar,
        0x09..=0x0D
        | 0x1C..=0x1F
        | 0x20
        | 0x85
        | 0xA0
        | 0x1680
        | 0x2000..=0x200A
        | 0x2028
        | 0x2029
        | 0x202F
        | 0x205F
        | 0x3000)
}

/// The strerror text for an OS error, from the
/// table this platform's CPython prints; errnos
/// outside the table fall back to the Rust display
/// (a named deviation in the round dossier).
pub fn strerror_text(err: &std::io::Error) -> String {
    match err.raw_os_error() {
        Some(errno) => {
            let text = match errno {
                1 => "Operation not permitted",
                2 => "No such file or directory",
                13 => "Permission denied",
                20 => "Not a directory",
                21 => "Is a directory",
                24 => "Too many open files",
                28 => "No space left on device",
                30 => "Read-only file system",
                32 => "Broken pipe",
                36 => "File name too long",
                40 => "Too many levels of symbolic links",
                _ => "",
            };
            if text.is_empty() {
                format!("{}", err)
            } else {
                text.to_string()
            }
        }
        None => format!("{}", err),
    }
}

/// Build text from scalar values, the inverse of
/// scalars_of: a value in the surrogate range stands
/// as its own unit, as a lone surrogate does in the
/// Python's strings, and an astral value becomes its
/// pair.
pub fn text_from_scalars(scalars: &[u32]) -> Text {
    let mut units = Vec::with_capacity(scalars.len());
    for &scalar in scalars {
        if scalar >= 0x10000 {
            let value = scalar - 0x10000;
            units.push(0xD800 + (value >> 10) as u16);
            units.push(0xDC00 + (value & 0x3FF) as u16);
        } else {
            units.push(scalar as u16);
        }
    }
    Text::from_units(units)
}

/// Whether a scalar ends a line for str.splitlines:
/// the wide boundary set, with CR LF read as one
/// boundary by the caller.
fn is_line_boundary(scalar: u32) -> bool {
    matches!(
        scalar,
        0x0A | 0x0B | 0x0C | 0x0D | 0x1C | 0x1D | 0x1E | 0x85 | 0x2028 | 0x2029
    )
}

/// str.splitlines over text units: the lines, without
/// their boundaries. A trailing boundary leaves no
/// empty final line, and the empty text has no lines.
pub fn py_splitlines(units: &[u16]) -> Vec<Text> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < units.len() {
        let (span, step) = step_at(units, index);
        let scalar = match step {
            Step::Scalar(scalar) => scalar,
            Step::Lone(unit) => unit as u32,
        };
        if is_line_boundary(scalar) {
            lines.push(Text::from_units(units[start..index].to_vec()));
            index += span;
            if scalar == 0x0D && index < units.len() && units[index] == 0x0A {
                index += 1;
            }
            start = index;
        } else {
            index += span;
        }
    }
    if start < units.len() {
        lines.push(Text::from_units(units[start..].to_vec()));
    }
    lines
}

/// The unit index at which a scalar index falls.
pub fn unit_offset(units: &[u16], scalar_index: usize) -> usize {
    let mut unit_at = 0;
    let mut seen = 0;
    while seen < scalar_index && unit_at < units.len() {
        let (span, _) = step_at(units, unit_at);
        unit_at += span;
        seen += 1;
    }
    unit_at
}

/// Decode bytes as the interpreter's utf-8-sig
/// decode does: a leading byte order mark is
/// stripped, and the rest must be strict UTF-8.
pub fn decode_utf8_sig(bytes: &[u8]) -> Option<String> {
    let body = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    };
    String::from_utf8(body.to_vec()).ok()
}

/// The bytes of an OS string, for decoding a path
/// the way CPython decodes it.
pub fn os_bytes(value: &std::ffi::OsStr) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        value.as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        value.to_string_lossy().into_owned().into_bytes()
    }
}
