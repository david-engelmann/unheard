//! Reading flighttape tapes (UH-5a, UH-19, UH-20).
//!
//! Unheard reads tapes at flighttape format version
//! 1.0 only, never raw harness session files. The
//! checks here are the format rules the rest of the
//! tool stands on: the value is an object carrying
//! tape_version 1.0, a run object, and an events
//! array whose seq values run consecutively from 0
//! and whose kinds are in the format's enum, and
//! the tool call shapes the flighttape validator
//! enforces under its FT-4 pairing rule, a tool
//! call carrying a non-empty tool name and a unique
//! non-empty call id, a tool result answering an
//! earlier call, an is_error that is a boolean when
//! present. Those rules are load-bearing, not
//! ceremony: obedience scoring trusts that a call
//! recorded in a tape happened as recorded, and a
//! tape that breaks the pairing can manufacture
//! evidence that never happened, so a file that
//! fails any check is excluded with its defects
//! named, in the rollup discipline, and the run
//! proceeds on the tapes that remain.

use std::collections::HashSet;
use std::path::Path;

use crate::json::{self, JsonError};
use crate::pyrepr::{repr_text, repr_value};
use crate::value::{Text, Value};

const EVENT_KINDS: [&str; 8] = [
    "session_start",
    "message",
    "tool_call",
    "tool_result",
    "diff",
    "spend",
    "note",
    "session_end",
];

/// The outcome of reading one tape file: the tape,
/// or the failure's reason with its format defects
/// when the file parsed but is not a valid tape.
pub enum TapeLoad {
    Loaded(Value),
    Failed(&'static str, Vec<Text>),
}

fn defect(text: String) -> Text {
    Text::from_str(&text)
}

fn is_object(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

fn field<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.get(key)
}

fn as_text(value: &Value) -> Option<&Text> {
    match value {
        Value::Str(text) => Some(text),
        _ => None,
    }
}

/// Whether an event's seq stands at its index: an
/// integer whose digits are the index. A float, a
/// boolean, or any other shape is not the index.
fn seq_is_index(value: Option<&Value>, index: usize) -> bool {
    match value {
        Some(Value::Int(digits)) => digits == &index.to_string(),
        _ => false,
    }
}

/// The format checks the tool's evidence stands on,
/// in the order the checks are stated.
pub fn tape_defects(tape: &Value) -> Vec<Text> {
    if !is_object(tape) {
        return vec![defect("the tape is not an object".to_string())];
    }
    let mut defects: Vec<Text> = Vec::new();
    if field(tape, "tape_version") != Some(&Value::Str(Text::from_str("1.0"))) {
        defects.push(defect("tape_version is not 1.0".to_string()));
    }
    if !field(tape, "run").map(is_object).unwrap_or(false) {
        defects.push(defect("run is not an object".to_string()));
    }
    let events = match field(tape, "events") {
        Some(Value::Array(items)) if !items.is_empty() => items,
        _ => {
            defects.push(defect("events must hold at least one event".to_string()));
            return defects;
        }
    };
    let mut seen_calls: HashSet<Text> = HashSet::new();
    for (index, event) in events.iter().enumerate() {
        let where_ = format!("events[{index}]");
        if !is_object(event) {
            defects.push(defect(format!("{where_} is not an object")));
            continue;
        }
        if !seq_is_index(field(event, "seq"), index) {
            defects.push(defect(format!("{where_}.seq is not {index}")));
        }
        let kind_name = field(event, "kind")
            .and_then(as_text)
            .and_then(|text| text.as_str());
        let kind_name = match kind_name {
            Some(name) if EVENT_KINDS.iter().any(|kind| *kind == name) => name,
            _ => {
                defects.push(defect(format!("{where_}.kind is not in the format enum")));
                continue;
            }
        };
        let kind_name = kind_name.as_str();
        let call_id = field(event, "call_id");
        match kind_name {
            "tool_call" => {
                let tool = field(event, "tool").and_then(as_text);
                if !matches!(tool, Some(text) if !text.units().is_empty()) {
                    defects.push(defect(format!("{where_}.tool must be a non-empty string")));
                }
                match call_id.and_then(as_text) {
                    Some(text) if !text.units().is_empty() => {
                        if seen_calls.contains(text) {
                            defects.push(defect(format!(
                                "{where_}.call_id {} is used twice",
                                repr_text(text)
                            )));
                        } else {
                            seen_calls.insert(text.clone());
                        }
                    }
                    _ => {
                        defects.push(defect(format!(
                            "{where_}.call_id must be a non-empty string"
                        )));
                    }
                }
            }
            "tool_result" => {
                let answered = match call_id.and_then(as_text) {
                    Some(text) => seen_calls.contains(text),
                    None => false,
                };
                if !answered {
                    let shown = match call_id {
                        Some(value) => repr_value(value),
                        None => "None".to_string(),
                    };
                    defects.push(defect(format!(
                        "{where_}.call_id {shown} answers no earlier tool_call"
                    )));
                }
                if let Some(is_error) = field(event, "is_error") {
                    if !matches!(is_error, Value::Bool(_)) {
                        defects.push(defect(format!("{where_}.is_error must be a boolean")));
                    }
                }
            }
            _ => {}
        }
    }
    defects
}

/// Read one tape file. A file that cannot be read,
/// is not valid UTF-8, or is not valid JSON fails
/// with that reason and no defects; a file that
/// parses but breaks the format fails as not a
/// valid tape, with its defects.
pub fn load_tape_file(path: &Path) -> TapeLoad {
    let raw = match std::fs::read(path) {
        Ok(raw) => raw,
        Err(_) => return TapeLoad::Failed("cannot read the tape file", Vec::new()),
    };
    let text = match String::from_utf8(raw) {
        Ok(text) => text,
        Err(_) => return TapeLoad::Failed("the file is not valid UTF-8", Vec::new()),
    };
    let scalars: Vec<u32> = text.chars().map(|ch| ch as u32).collect();
    let tape = match json::parse(&scalars) {
        Ok(value) => value,
        Err(JsonError::IntTooLong) => {
            return TapeLoad::Failed("the file holds a number too large to read", Vec::new());
        }
        Err(_) => return TapeLoad::Failed("the file is not valid JSON", Vec::new()),
    };
    let defects = tape_defects(&tape);
    if !defects.is_empty() {
        return TapeLoad::Failed("not a valid tape", defects);
    }
    TapeLoad::Loaded(tape)
}
