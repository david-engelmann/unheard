//! UH-7 to UH-12, UH-13a, UH-14a, UH-23. Score
//! checkable rules against a tape set and emit the
//! observations document.

use crate::shapes::{classify, Shape};
use crate::util::scalars_of;
use crate::value::{Text, Value};

fn as_text(value: &Value) -> Option<&Text> {
    match value {
        Value::Str(text) => Some(text),
        _ => None,
    }
}

/// Every string in a value, as the Python's
/// generator yields them: a string yields itself,
/// an object yields each key then its value in
/// order, an array yields its items in order
/// (UH-7a: the path appears in the input).
fn strings(value: &Value) -> Vec<&Text> {
    let mut out = Vec::new();
    collect_strings(value, &mut out);
    out
}

fn collect_strings<'a>(value: &'a Value, out: &mut Vec<&'a Text>) {
    match value {
        Value::Str(text) => out.push(text),
        Value::Object(entries) => {
            for (key, item) in entries {
                out.push(key);
                collect_strings(item, out);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_strings(item, out);
            }
        }
        _ => {}
    }
}

/// One exercised event: the event's seq as its
/// integer digits, whether the rule held, and the
/// tool the event used.
#[derive(Clone, Debug, PartialEq)]
pub struct Exercise {
    pub seq: String,
    pub held: bool,
    pub used: Text,
}

fn calls_of(tape: &Value) -> Vec<&Value> {
    match tape.get("events") {
        Some(Value::Array(events)) => events
            .iter()
            .filter(|event| event.get("kind") == Some(&Value::Str(Text::from_str("tool_call"))))
            .collect(),
        _ => Vec::new(),
    }
}

fn tool_of(call: &Value) -> Option<&Text> {
    call.get("tool").and_then(as_text)
}

fn seq_of(call: &Value) -> String {
    match call.get("seq") {
        Some(Value::Int(digits)) => digits.clone(),
        _ => String::new(),
    }
}

/// Compare two integer digit strings as the
/// integers they name: the comparison the cap's
/// sort and the first step's minimum read.
fn cmp_int_digits(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let (a_neg, a_digits) = match a.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, a),
    };
    let (b_neg, b_digits) = match b.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, b),
    };
    let a_zero = a_digits.bytes().all(|byte| byte == b'0');
    let b_zero = b_digits.bytes().all(|byte| byte == b'0');
    let a_neg = a_neg && !a_zero;
    let b_neg = b_neg && !b_zero;
    match (a_neg, b_neg) {
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        _ => {}
    }
    let a_trim = a_digits.trim_start_matches('0');
    let b_trim = b_digits.trim_start_matches('0');
    let magnitude = a_trim
        .len()
        .cmp(&b_trim.len())
        .then_with(|| a_trim.cmp(b_trim));
    if a_neg {
        magnitude.reverse()
    } else {
        magnitude
    }
}

/// Whether a text ends with the extension, code
/// point by code point: the Python's str.endswith
/// over the decoded scalars.
fn ends_with(text: &Text, extension: &Text) -> bool {
    let hay = scalars_of(text.units());
    let needle = scalars_of(extension.units());
    hay.len() >= needle.len() && hay[hay.len() - needle.len()..] == needle[..]
}

/// Whether the path appears in the joined blob of
/// a call's input strings: the Python's substring
/// test, over the decoded scalars of the strings
/// joined by newlines.
fn blob_contains(input: Option<&Value>, path: &Text) -> bool {
    let mut blob: Vec<u32> = Vec::new();
    if let Some(value) = input {
        for (index, string) in strings(value).iter().enumerate() {
            if index > 0 {
                blob.push(0x0A);
            }
            blob.extend(scalars_of(string.units()));
        }
    }
    let needle = scalars_of(path.units());
    !needle.is_empty()
        && blob
            .windows(needle.len())
            .any(|window| window == needle.as_slice())
}

/// The exercised events of one shape in one tape,
/// as (seq, held, used tool) per exercised event.
pub fn exercises(shape: &Shape, tape: &Value) -> Vec<Exercise> {
    let calls = calls_of(tape);
    let mut out: Vec<Exercise> = Vec::new();
    match shape {
        Shape::ProhibitTool { tool } => {
            for call in &calls {
                if tool_of(call) == Some(tool) {
                    out.push(Exercise {
                        seq: seq_of(call),
                        held: false,
                        used: tool.clone(),
                    });
                }
            }
        }
        Shape::ProhibitPath { path } => {
            for call in &calls {
                if blob_contains(call.get("input"), path) {
                    out.push(Exercise {
                        seq: seq_of(call),
                        held: false,
                        used: tool_of(call).cloned().unwrap_or_else(|| Text::from_str("")),
                    });
                }
            }
        }
        Shape::Ordering { first, second } => {
            let target = calls.iter().find(|call| tool_of(call) == Some(second));
            if let Some(target) = target {
                let target_seq = seq_of(target);
                let held = calls.iter().any(|call| {
                    tool_of(call) == Some(first)
                        && cmp_int_digits(&seq_of(call), &target_seq) == std::cmp::Ordering::Less
                });
                out.push(Exercise {
                    seq: target_seq,
                    held,
                    used: second.clone(),
                });
            }
        }
        Shape::Cap { tool, limit } => {
            let mut mine: Vec<&Value> = calls
                .iter()
                .filter(|call| tool_of(call) == Some(tool))
                .copied()
                .collect();
            mine.sort_by(|a, b| cmp_int_digits(&seq_of(a), &seq_of(b)));
            for (index, call) in mine.iter().enumerate() {
                out.push(Exercise {
                    seq: seq_of(call),
                    held: (index as u64) < *limit,
                    used: tool.clone(),
                });
            }
        }
        Shape::First { tool } => {
            if !calls.is_empty() {
                let mut first = calls[0];
                for call in &calls[1..] {
                    if cmp_int_digits(&seq_of(call), &seq_of(first)) == std::cmp::Ordering::Less {
                        first = call;
                    }
                }
                let used = tool_of(first)
                    .cloned()
                    .unwrap_or_else(|| Text::from_str(""));
                out.push(Exercise {
                    seq: seq_of(first),
                    held: used == *tool,
                    used,
                });
            }
        }
        Shape::RequiredTool { tool, extension } => {
            for call in &calls {
                let touched = match call.get("input") {
                    Some(input) => strings(input)
                        .iter()
                        .any(|string| ends_with(string, extension)),
                    None => false,
                };
                if touched {
                    let used = tool_of(call).cloned().unwrap_or_else(|| Text::from_str(""));
                    out.push(Exercise {
                        seq: seq_of(call),
                        held: used == *tool,
                        used,
                    });
                }
            }
        }
    }
    out
}

fn int(value: i64) -> Value {
    Value::Int(value.to_string())
}

fn text_value(text: &Text) -> Value {
    Value::Str(text.clone())
}

fn str_value(text: &str) -> Value {
    Value::Str(Text::from_str(text))
}

/// Score the rule texts against the scored tapes
/// and assemble the observations document, in the
/// Python's field order throughout.
pub fn score(
    rule_texts: &[Text],
    scored_tapes: &[(Value, String)],
    excluded: Vec<Value>,
    warnings: &[Text],
) -> Value {
    let mut observations: Vec<Value> = Vec::new();
    let mut scores: Vec<Value> = Vec::new();
    let mut unmeasured: Vec<Value> = Vec::new();
    let mut unexercised: Vec<Value> = Vec::new();
    let tapes_total = scored_tapes.len() as i64;

    let mut seen: Vec<&Text> = Vec::new();
    let mut duplicates: Vec<Value> = Vec::new();
    let mut distinct: Vec<&Text> = Vec::new();
    for text in rule_texts {
        if seen.contains(&text) {
            if !duplicates.contains(&text_value(text)) {
                duplicates.push(text_value(text));
            }
            continue;
        }
        seen.push(text);
        distinct.push(text);
    }

    for text in distinct {
        let shape = classify(text);
        let shape = match shape {
            None => {
                unmeasured.push(text_value(text));
                scores.push(Value::Object(vec![
                    (Text::from_str("rule"), text_value(text)),
                    (Text::from_str("shape"), str_value("unmeasured")),
                    (Text::from_str("tapes_exercised"), int(0)),
                    (Text::from_str("tapes_total"), int(tapes_total)),
                    (Text::from_str("events_exercised"), int(0)),
                    (Text::from_str("violations"), int(0)),
                ]));
                continue;
            }
            Some(shape) => shape,
        };
        let mut events_exercised: i64 = 0;
        let mut violations: i64 = 0;
        let mut tapes_exercised: i64 = 0;
        for (tape, digest) in scored_tapes {
            let exercised = exercises(&shape, tape);
            if !exercised.is_empty() {
                tapes_exercised += 1;
            }
            for exercise in exercised {
                events_exercised += 1;
                let quote = Text::from_str(&format!("tape {digest} seq {}", exercise.seq));
                if exercise.held {
                    observations.push(Value::Object(vec![
                        (Text::from_str("rule"), text_value(text)),
                        (Text::from_str("kind"), str_value("followed")),
                        (Text::from_str("quote"), text_value(&quote)),
                    ]));
                } else if matches!(shape, Shape::RequiredTool { .. }) {
                    violations += 1;
                    let mut units = quote.units().to_vec();
                    units.extend(" used ".encode_utf16());
                    units.extend_from_slice(exercise.used.units());
                    let quote = Text::from_units(units);
                    observations.push(Value::Object(vec![
                        (Text::from_str("rule"), text_value(text)),
                        (Text::from_str("kind"), str_value("misreading")),
                        (Text::from_str("quote"), text_value(&quote)),
                    ]));
                } else {
                    violations += 1;
                    observations.push(Value::Object(vec![
                        (Text::from_str("rule"), text_value(text)),
                        (Text::from_str("kind"), str_value("ignored")),
                        (Text::from_str("quote"), text_value(&quote)),
                    ]));
                }
            }
        }
        if events_exercised == 0 {
            unexercised.push(text_value(text));
        }
        scores.push(Value::Object(vec![
            (Text::from_str("rule"), text_value(text)),
            (Text::from_str("shape"), str_value(shape.name())),
            (Text::from_str("tapes_exercised"), int(tapes_exercised)),
            (Text::from_str("tapes_total"), int(tapes_total)),
            (Text::from_str("events_exercised"), int(events_exercised)),
            (Text::from_str("violations"), int(violations)),
        ]));
    }

    let tapes_read = tapes_total + excluded.len() as i64;
    let unmeasured_count = unmeasured.len() as i64;
    let mut document = vec![
        (Text::from_str("observations"), Value::Array(observations)),
        (Text::from_str("scores"), Value::Array(scores)),
        (Text::from_str("unmeasured"), Value::Array(unmeasured)),
        (Text::from_str("unmeasured_count"), int(unmeasured_count)),
        (Text::from_str("unexercised"), Value::Array(unexercised)),
        (
            Text::from_str("tapes"),
            Value::Object(vec![
                (Text::from_str("read"), int(tapes_read)),
                (Text::from_str("scored"), int(tapes_total)),
                (Text::from_str("excluded"), Value::Array(excluded)),
            ]),
        ),
    ];
    if !duplicates.is_empty() {
        document.push((Text::from_str("duplicates"), Value::Array(duplicates)));
    }
    if !warnings.is_empty() {
        document.push((
            Text::from_str("warnings"),
            Value::Array(warnings.iter().map(text_value).collect()),
        ));
    }
    Value::Object(document)
}
