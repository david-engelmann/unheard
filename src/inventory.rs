//! The rules inventory (UH-5, UH-22): parse a rules
//! file into its rules, under the same discipline
//! the editing tool reads them with. The
//! parser works on code points throughout, with
//! CPython's whitespace and decimal digit sets, and
//! takes its decisions in the Python's own order:
//! fence state, comment state, a comment's opening,
//! a fence's opening, a setext underline, an ATX
//! heading, an empty marker, a rule marker, a blank
//! line, a continuation, and only then prose.

use crate::failure::Failure;
use crate::util::{py_is_space, scalars_of, text_from_scalars};
use crate::value::Text;

/// One inventoried rule: its id by position, its
/// section path, its exact text, and its line span
/// in 1-based line numbers.
#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub id: Text,
    pub section: Vec<Text>,
    pub text: Text,
    pub line_span: [i64; 2],
}

fn strip(scalars: &[u32]) -> &[u32] {
    let mut start = 0;
    let mut end = scalars.len();
    while start < end && py_is_space(scalars[start]) {
        start += 1;
    }
    while end > start && py_is_space(scalars[end - 1]) {
        end -= 1;
    }
    &scalars[start..end]
}

fn last_nonspace(scalars: &[u32], from: usize) -> Option<usize> {
    let mut index = scalars.len();
    while index > from {
        if !py_is_space(scalars[index - 1]) {
            return Some(index);
        }
        index -= 1;
    }
    None
}

fn contains_run(scalars: &[u32], run: &[u32]) -> bool {
    !run.is_empty() && scalars.windows(run.len()).any(|window| window == run)
}

/// The ATX heading shape: one to six hashes, a
/// whitespace run, and text ending at its last
/// nonspace character. Returns the heading's raw
/// text.
fn match_heading(line: &[u32]) -> Option<Vec<u32>> {
    let mut at = 0;
    while at < line.len() && line[at] == 0x23 {
        at += 1;
    }
    if at == 0 || at > 6 || at >= line.len() || !py_is_space(line[at]) {
        return None;
    }
    while at < line.len() && py_is_space(line[at]) {
        at += 1;
    }
    let end = last_nonspace(line, at)?;
    Some(line[at..end].to_vec())
}

/// The heading's name, with a closing hash run and
/// the whitespace before it removed where they
/// stand.
fn heading_text(raw: &[u32]) -> Vec<u32> {
    let mut run_start = raw.len();
    while run_start > 0 && raw[run_start - 1] == 0x23 {
        run_start -= 1;
    }
    if run_start == raw.len() || run_start == 0 || !py_is_space(raw[run_start - 1]) {
        return raw.to_vec();
    }
    let mut prefix_end = run_start;
    while prefix_end > 0 && py_is_space(raw[prefix_end - 1]) {
        prefix_end -= 1;
    }
    raw[..prefix_end].to_vec()
}

/// A fence's opening run: backticks or tildes, at
/// least three, after any leading whitespace.
fn match_fence(line: &[u32]) -> Option<(u32, usize)> {
    let mut at = 0;
    while at < line.len() && py_is_space(line[at]) {
        at += 1;
    }
    if at >= line.len() || (line[at] != 0x60 && line[at] != 0x7E) {
        return None;
    }
    let ch = line[at];
    let mut end = at;
    while end < line.len() && line[end] == ch {
        end += 1;
    }
    if end - at >= 3 {
        Some((ch, end - at))
    } else {
        None
    }
}

/// Whether a line closes the open fence: nothing on
/// it but whitespace and the fence's own character.
fn fence_closes(line: &[u32], ch: u32) -> bool {
    let trimmed = strip(line);
    let mut start = 0;
    let mut end = trimmed.len();
    while start < end && trimmed[start] == ch {
        start += 1;
    }
    while end > start && trimmed[end - 1] == ch {
        end -= 1;
    }
    strip(&trimmed[start..end]).is_empty()
}

fn is_setext(line: &[u32], ch: u32) -> bool {
    let trimmed = strip(line);
    !trimmed.is_empty() && trimmed.iter().all(|&scalar| scalar == ch)
}

/// A marker with no text after it.
fn match_empty_marker(line: &[u32]) -> bool {
    let trimmed = strip(line);
    if trimmed.len() == 1 && matches!(trimmed[0], 0x2D | 0x2A | 0x2B) {
        return true;
    }
    let mut at = 0;
    while at < trimmed.len() && crate::util::decimal_digit(trimmed[at]).is_some() {
        at += 1;
    }
    at > 0 && at + 1 == trimmed.len() && matches!(trimmed[at], 0x2E | 0x29)
}

/// A bullet or numbered marker with its text:
/// the indent in code points and the text, which
/// ends at its last nonspace character.
fn match_marker(line: &[u32]) -> Option<(usize, Vec<u32>)> {
    let mut at = 0;
    while at < line.len() && py_is_space(line[at]) {
        at += 1;
    }
    let indent = at;
    let marker_end = if at < line.len() && matches!(line[at], 0x2D | 0x2A | 0x2B) {
        at + 1
    } else {
        let mut end = at;
        while end < line.len() && crate::util::decimal_digit(line[end]).is_some() {
            end += 1;
        }
        if end > at && end < line.len() && matches!(line[end], 0x2E | 0x29) {
            end + 1
        } else {
            return None;
        }
    };
    if marker_end >= line.len() || !py_is_space(line[marker_end]) {
        return None;
    }
    let mut start = marker_end;
    while start < line.len() && py_is_space(line[start]) {
        start += 1;
    }
    let end = last_nonspace(line, start)?;
    Some((indent, line[start..end].to_vec()))
}

/// A task list checkbox stripped from a rule's
/// text, where the content carries one.
fn match_checkbox(content: &[u32]) -> Option<Vec<u32>> {
    if content.len() < 4
        || content[0] != 0x5B
        || !matches!(content[1], 0x20 | 0x78 | 0x58)
        || content[2] != 0x5D
        || !py_is_space(content[3])
    {
        return None;
    }
    let mut start = 3;
    while start < content.len() && py_is_space(content[start]) {
        start += 1;
    }
    let end = last_nonspace(content, start)?;
    Some(content[start..end].to_vec())
}

struct Current {
    id: Text,
    section: Vec<Text>,
    start: i64,
    lines: Vec<Vec<u32>>,
    indent: usize,
}

/// Parse the rules text into (rules, warnings).
pub fn parse_rules(text: &Text) -> Result<(Vec<Rule>, Vec<Text>), Failure> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut warnings: Vec<Text> = Vec::new();
    let mut section_stack: Vec<(usize, Vec<u32>)> = Vec::new();
    let mut fence: Option<(u32, usize, i64)> = None;
    let mut in_comment = false;
    let mut current: Option<Current> = None;
    let mut last_content_line: i64 = 0;
    let mut pending_prose: Option<Vec<u32>> = None;

    fn close_current(current: &mut Option<Current>, rules: &mut Vec<Rule>, last_content_line: i64) {
        if let Some(done) = current.take() {
            let mut scalars: Vec<u32> = Vec::new();
            for (index, line) in done.lines.iter().enumerate() {
                if index > 0 {
                    scalars.push(0x0A);
                }
                scalars.extend_from_slice(line);
            }
            rules.push(Rule {
                id: done.id,
                section: done.section,
                text: text_from_scalars(&scalars),
                line_span: [done.start, last_content_line],
            });
        }
    }

    fn push_section(stack: &mut Vec<(usize, Vec<u32>)>, level: usize, name: Vec<u32>) {
        while matches!(stack.last(), Some((top, _)) if *top >= level) {
            stack.pop();
        }
        stack.push((level, name));
    }

    let lines = crate::util::py_splitlines(text.units());
    for (index, line_text) in lines.iter().enumerate() {
        let idx = index as i64 + 1;
        let line = scalars_of(line_text.units());
        let stripped = strip(&line).to_vec();
        if let Some((fence_ch, fence_len, _)) = fence {
            if let Some((ch, len)) = match_fence(&line) {
                if ch == fence_ch && len >= fence_len && fence_closes(&line, fence_ch) {
                    fence = None;
                }
            }
            continue;
        }
        if in_comment {
            if contains_run(&line, &[0x2D, 0x2D, 0x3E]) {
                in_comment = false;
            }
            continue;
        }
        if stripped.len() >= 4 && stripped[..4] == [0x3C, 0x21, 0x2D, 0x2D] {
            close_current(&mut current, &mut rules, last_content_line);
            pending_prose = None;
            if !contains_run(&stripped[4..], &[0x2D, 0x2D, 0x3E]) {
                in_comment = true;
            }
            continue;
        }
        if let Some((ch, len)) = match_fence(&line) {
            close_current(&mut current, &mut rules, last_content_line);
            pending_prose = None;
            fence = Some((ch, len, idx));
            continue;
        }
        if pending_prose.is_some() && (is_setext(&line, 0x3D) || is_setext(&line, 0x2D)) {
            let level = if is_setext(&line, 0x3D) { 1 } else { 2 };
            close_current(&mut current, &mut rules, last_content_line);
            push_section(
                &mut section_stack,
                level,
                pending_prose.take().expect("prose stands"),
            );
            continue;
        }
        if let Some(raw) = match_heading(&line) {
            close_current(&mut current, &mut rules, last_content_line);
            pending_prose = None;
            let level = line.iter().take_while(|&&scalar| scalar == 0x23).count();
            push_section(&mut section_stack, level, heading_text(&raw));
            continue;
        }
        if match_empty_marker(&line) {
            return Err(Failure::named(&format!("empty rule at line {idx}")));
        }
        if let Some((indent, content)) = match_marker(&line) {
            if section_stack.is_empty() {
                return Err(Failure::named(&format!(
                    "rule outside any section at line {idx}"
                )));
            }
            close_current(&mut current, &mut rules, last_content_line);
            pending_prose = None;
            let content = match_checkbox(&content).unwrap_or(content);
            current = Some(Current {
                id: Text::from_str(&format!("rule-{}", rules.len() + 1)),
                section: section_stack
                    .iter()
                    .map(|(_, name)| text_from_scalars(name))
                    .collect(),
                start: idx,
                lines: vec![content],
                indent,
            });
            last_content_line = idx;
            continue;
        }
        if stripped.is_empty() {
            pending_prose = None;
            continue;
        }
        let indent = {
            let mut count = 0;
            let mut at = 0;
            let units = line_text.units();
            while at < units.len() {
                let (span, step) = crate::util::step_at(units, at);
                let scalar = match step {
                    crate::util::Step::Scalar(scalar) => scalar,
                    crate::util::Step::Lone(unit) => unit as u32,
                };
                if !py_is_space(scalar) {
                    break;
                }
                count += 1;
                at += span;
            }
            count
        };
        if matches!(&current, Some(open) if indent > open.indent) {
            if let Some(open) = &mut current {
                open.lines.push(stripped.clone());
            }
            last_content_line = idx;
            continue;
        }
        close_current(&mut current, &mut rules, last_content_line);
        pending_prose = Some(stripped);
    }
    close_current(&mut current, &mut rules, last_content_line);
    if let Some((_, _, fence_line)) = fence {
        warnings.push(Text::from_str(&format!(
            "unclosed fence at line {fence_line}, the rest of the file was read as code"
        )));
    }
    Ok((rules, warnings))
}
