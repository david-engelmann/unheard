//! The JSON reader (UH-19): a strict iterative
//! scanner over the decoded text, deciding every
//! refusal in text order, as CPython's json.loads
//! does. It reports errors in the interpreter's own
//! terms; the tape loader collapses every one but
//! the integer limit to its one named refusal.
//! Duplicate keys follow Python dictionary
//! assignment: the last value wins, the first key
//! position is retained. Integers keep their exact
//! digits, a literal past 4300 digits refuses with
//! the interpreter's limit text, and NaN and the
//! infinities read as floats.

use crate::value::{Text, Value};

/// The reader's nesting limit in containers, probed
/// at the shipped CLI: a text nested 9994 containers
/// deep parses, and the container that would open
/// past it refuses with the recursion wording.
pub(crate) const DEPTH_LIMIT: usize = 9994;

/// An integer literal's digit limit, the
/// interpreter's conversion bound.
pub(crate) const INT_DIGIT_LIMIT: usize = 4300;

#[derive(Debug, PartialEq)]
pub enum JsonError {
    /// A syntax refusal with CPython's message and
    /// the character position it names.
    Syntax { msg: &'static str, pos: usize },
    /// A string that never closes.
    Unterminated { start: usize },
    /// A backslash escape outside the JSON set.
    InvalidEscape { pos: usize },
    /// A malformed unicode escape, its position at
    /// the escape's u.
    InvalidUnicode { pos: usize },
    /// A raw control character inside a string.
    InvalidControl { pos: usize },
    /// An integer literal past the digit limit.
    IntTooLong { digits: usize },
    /// The container that would open past the
    /// nesting limit, and its kind for the recursion
    /// wording.
    Depth { object: bool },
}

fn line_col(source: &[u32], pos: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut last_newline: Option<usize> = None;
    for (index, &scalar) in source.iter().enumerate().take(pos) {
        if scalar == 0x0A {
            line += 1;
            last_newline = Some(index);
        }
    }
    let col = match last_newline {
        None => pos + 1,
        Some(index) => pos - index,
    };
    (line, col)
}

impl JsonError {
    /// The CPython exception text for this refusal:
    /// what json.loads raises, as a string.
    pub fn describe(&self, source: &[u32]) -> String {
        match self {
            JsonError::IntTooLong { digits } => format!(
                "Exceeds the limit (4300 digits) for integer string conversion: \
                 value has {digits} digits; use sys.set_int_max_str_digits() \
                 to increase the limit"
            ),
            JsonError::Depth { object } => format!(
                "maximum recursion depth exceeded while decoding a JSON {} \
                 from a unicode string",
                if *object { "object" } else { "array" }
            ),
            JsonError::Syntax { msg, pos } => {
                let (line, col) = line_col(source, *pos);
                format!("{msg}: line {line} column {col} (char {pos})")
            }
            JsonError::Unterminated { start } => {
                let (line, col) = line_col(source, *start);
                format!(
                    "Unterminated string starting at: line {line} column {col} \
                     (char {start})"
                )
            }
            JsonError::InvalidEscape { pos } => {
                let (line, col) = line_col(source, *pos);
                format!("Invalid \\escape: line {line} column {col} (char {pos})")
            }
            JsonError::InvalidUnicode { pos } => {
                let (line, col) = line_col(source, *pos);
                format!("Invalid \\uXXXX escape: line {line} column {col} (char {pos})")
            }
            JsonError::InvalidControl { pos } => {
                let (line, col) = line_col(source, *pos);
                format!(
                    "Invalid control character at: line {line} column {col} \
                     (char {pos})"
                )
            }
        }
    }
}

enum Frame {
    Array(Vec<Value>),
    Object {
        entries: Vec<(Text, Value)>,
        pending: Option<Text>,
    },
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    /// A value is required.
    Value,
    /// A value is required, or the array just opened
    /// closes empty.
    FirstValue,
    /// An object key is required; `first` also
    /// allows the object to close empty.
    Key { first: bool },
    /// The colon after a key is required.
    Colon,
    /// A comma or the open container's close.
    AfterValue,
}

struct Scanner<'a> {
    chars: &'a [u32],
    at: usize,
}

impl<'a> Scanner<'a> {
    fn peek(&self) -> Option<u32> {
        self.chars.get(self.at).copied()
    }

    fn peek_at(&self, offset: usize) -> Option<u32> {
        self.chars.get(self.at + offset).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(0x20 | 0x09 | 0x0A | 0x0D)) {
            self.at += 1;
        }
    }

    fn literal(&mut self, word: &str, start: usize) -> Result<Value, JsonError> {
        let letters: Vec<u32> = word.chars().map(|c| c as u32).collect();
        if self.chars.len() - self.at < letters.len()
            || self.chars[self.at..self.at + letters.len()] != letters[..]
        {
            return Err(JsonError::Syntax {
                msg: "Expecting value",
                pos: start,
            });
        }
        self.at += letters.len();
        Ok(match word {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            "null" => Value::Null,
            "NaN" => Value::Float(f64::NAN),
            "Infinity" => Value::Float(f64::INFINITY),
            "-Infinity" => Value::Float(f64::NEG_INFINITY),
            _ => unreachable!("a literal word"),
        })
    }

    /// Four hex digits after the u at `upos`. The
    /// interpreter also reports this refusal when the
    /// text ends exactly after the fourth digit, a
    /// quirk of its scanner preserved here.
    fn hex4(&mut self, upos: usize) -> Result<u16, JsonError> {
        let mut value: u32 = 0;
        for offset in 1..=4 {
            let digit = self.chars.get(upos + offset).copied();
            value = value * 16
                + match digit {
                    Some(0x30..=0x39) => digit.expect("a digit") - 0x30,
                    Some(0x61..=0x66) => digit.expect("a digit") - 0x61 + 10,
                    Some(0x41..=0x46) => digit.expect("a digit") - 0x41 + 10,
                    _ => {
                        return Err(JsonError::InvalidUnicode { pos: upos });
                    }
                };
        }
        self.at = upos + 5;
        if self.at == self.chars.len() {
            return Err(JsonError::InvalidUnicode { pos: upos });
        }
        Ok(value as u16)
    }

    fn string(&mut self) -> Result<Text, JsonError> {
        // The caller has seen the opening quote.
        let start = self.at;
        self.at += 1;
        let mut out: Vec<u16> = Vec::new();
        loop {
            let scalar = match self.peek() {
                Some(scalar) => scalar,
                None => return Err(JsonError::Unterminated { start }),
            };
            match scalar {
                0x22 => {
                    self.at += 1;
                    return Ok(Text::from_units(out));
                }
                0x5C => {
                    let slash = self.at;
                    self.at += 1;
                    let escape = match self.peek() {
                        Some(escape) => escape,
                        None => return Err(JsonError::Unterminated { start }),
                    };
                    match escape {
                        0x22 => out.push(0x22),
                        0x5C => out.push(0x5C),
                        0x2F => out.push(0x2F),
                        0x62 => out.push(0x08),
                        0x66 => out.push(0x0C),
                        0x6E => out.push(0x0A),
                        0x72 => out.push(0x0D),
                        0x74 => out.push(0x09),
                        0x75 => {
                            let value = self.hex4(self.at)?;
                            out.push(value);
                            // A high surrogate followed
                            // by a second escape reads
                            // that escape too; the pair
                            // stands or falls as units,
                            // as the Python's string
                            // holds them.
                            if (0xD800..=0xDBFF).contains(&value)
                                && self.peek() == Some(0x5C)
                                && self.peek_at(1) == Some(0x75)
                            {
                                let second = self.hex4(self.at + 1)?;
                                out.push(second);
                            }
                            continue;
                        }
                        _ => return Err(JsonError::InvalidEscape { pos: slash }),
                    }
                    self.at += 1;
                }
                0x00..=0x1F => {
                    return Err(JsonError::InvalidControl { pos: self.at });
                }
                _ => {
                    if scalar < 0x10000 {
                        out.push(scalar as u16);
                    } else {
                        let value = scalar - 0x10000;
                        out.push(0xD800 + (value >> 10) as u16);
                        out.push(0xDC00 + (value & 0x3FF) as u16);
                    }
                    self.at += 1;
                }
            }
        }
    }

    fn number(&mut self) -> Result<Value, JsonError> {
        let start = self.at;
        if self.peek() == Some(0x2D) {
            self.at += 1;
        }
        match self.peek() {
            Some(0x30) => {
                self.at += 1;
            }
            Some(0x31..=0x39) => {
                while matches!(self.peek(), Some(0x30..=0x39)) {
                    self.at += 1;
                }
            }
            _ => {
                return Err(JsonError::Syntax {
                    msg: "Expecting value",
                    pos: start,
                })
            }
        }
        let int_digits = self.chars[start..self.at]
            .iter()
            .filter(|&&scalar| (0x30..=0x39).contains(&scalar))
            .count();
        let mut is_float = false;
        if self.peek() == Some(0x2E) && matches!(self.peek_at(1), Some(0x30..=0x39)) {
            is_float = true;
            self.at += 1;
            while matches!(self.peek(), Some(0x30..=0x39)) {
                self.at += 1;
            }
        }
        if matches!(self.peek(), Some(0x65) | Some(0x45)) {
            let mut probe = self.at + 1;
            if matches!(self.chars.get(probe), Some(0x2B) | Some(0x2D)) {
                probe += 1;
            }
            if matches!(self.chars.get(probe), Some(0x30..=0x39)) {
                is_float = true;
                self.at = probe;
                while matches!(self.peek(), Some(0x30..=0x39)) {
                    self.at += 1;
                }
            }
        }
        let token: String = self.chars[start..self.at]
            .iter()
            .map(|&scalar| char::from_u32(scalar).expect("a number token is ASCII"))
            .collect();
        if is_float {
            token
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| JsonError::Syntax {
                    msg: "Expecting value",
                    pos: start,
                })
        } else {
            if int_digits > INT_DIGIT_LIMIT {
                return Err(JsonError::IntTooLong { digits: int_digits });
            }
            // CPython's int reading drops the sign of
            // zero: -0 is the integer 0.
            let token = if token == "-0" {
                "0".to_string()
            } else {
                token
            };
            Ok(Value::Int(token))
        }
    }
}

/// Attach a completed value to the open frame, or
/// return it as the root when no frame is open.
fn attach(stack: &mut [Frame], value: Value) -> Option<Value> {
    match stack.last_mut() {
        None => Some(value),
        Some(Frame::Array(items)) => {
            items.push(value);
            None
        }
        Some(Frame::Object { entries, pending }) => {
            let key = pending.take().expect("a member value has its key");
            match entries.iter_mut().find(|(name, _)| *name == key) {
                Some((_, slot)) => *slot = value,
                None => entries.push((key, value)),
            }
            None
        }
    }
}

fn close_frame(frame: Frame) -> Value {
    match frame {
        Frame::Array(items) => Value::Array(items),
        Frame::Object { entries, .. } => Value::Object(entries),
    }
}

/// Parse a whole JSON text. The characters are the
/// decoded text's code points, the units the
/// interpreter's positions count in.
pub fn parse(source: &[u32]) -> Result<Value, JsonError> {
    // A leading byte order mark is its own refusal
    // in the interpreter, before any scanning.
    if source.first() == Some(&0xFEFF) {
        return Err(JsonError::Syntax {
            msg: "Unexpected UTF-8 BOM (decode using utf-8-sig)",
            pos: 0,
        });
    }
    let mut scanner = Scanner {
        chars: source,
        at: 0,
    };
    let mut stack: Vec<Frame> = Vec::new();
    let mut mode = Mode::Value;
    'outer: loop {
        match mode {
            Mode::Value | Mode::FirstValue => {
                scanner.skip_ws();
                let at = scanner.at;
                let scalar = match scanner.peek() {
                    Some(scalar) => scalar,
                    None => {
                        return Err(JsonError::Syntax {
                            msg: "Expecting value",
                            pos: at,
                        })
                    }
                };
                if mode == Mode::FirstValue && scalar == 0x5D {
                    scanner.at += 1;
                    let frame = stack.pop().expect("an array frame");
                    if let Some(done) = attach(&mut stack, close_frame(frame)) {
                        scanner.skip_ws();
                        if scanner.at != source.len() {
                            return Err(JsonError::Syntax {
                                msg: "Extra data",
                                pos: scanner.at,
                            });
                        }
                        return Ok(done);
                    }
                    mode = Mode::AfterValue;
                    continue 'outer;
                }
                let value = match scalar {
                    0x7B => {
                        if stack.len() >= DEPTH_LIMIT {
                            return Err(JsonError::Depth { object: true });
                        }
                        scanner.at += 1;
                        stack.push(Frame::Object {
                            entries: Vec::new(),
                            pending: None,
                        });
                        mode = Mode::Key { first: true };
                        continue 'outer;
                    }
                    0x5B => {
                        if stack.len() >= DEPTH_LIMIT {
                            return Err(JsonError::Depth { object: false });
                        }
                        scanner.at += 1;
                        stack.push(Frame::Array(Vec::new()));
                        mode = Mode::FirstValue;
                        continue 'outer;
                    }
                    0x22 => Value::Str(scanner.string()?),
                    0x74 => scanner.literal("true", at)?,
                    0x66 => scanner.literal("false", at)?,
                    0x6E => scanner.literal("null", at)?,
                    0x4E => scanner.literal("NaN", at)?,
                    0x49 => scanner.literal("Infinity", at)?,
                    0x2D => {
                        if scanner.peek_at(1) == Some(0x49) {
                            scanner.literal("-Infinity", at)?
                        } else {
                            scanner.number()?
                        }
                    }
                    0x30..=0x39 => scanner.number()?,
                    _ => {
                        return Err(JsonError::Syntax {
                            msg: "Expecting value",
                            pos: at,
                        })
                    }
                };
                if let Some(done) = attach(&mut stack, value) {
                    scanner.skip_ws();
                    if scanner.at != source.len() {
                        return Err(JsonError::Syntax {
                            msg: "Extra data",
                            pos: scanner.at,
                        });
                    }
                    return Ok(done);
                }
                mode = Mode::AfterValue;
            }
            Mode::Key { first } => {
                scanner.skip_ws();
                let at = scanner.at;
                match scanner.peek() {
                    Some(0x7D) if first => {
                        scanner.at += 1;
                        let frame = stack.pop().expect("an object frame");
                        if let Some(done) = attach(&mut stack, close_frame(frame)) {
                            scanner.skip_ws();
                            if scanner.at != source.len() {
                                return Err(JsonError::Syntax {
                                    msg: "Extra data",
                                    pos: scanner.at,
                                });
                            }
                            return Ok(done);
                        }
                        mode = Mode::AfterValue;
                    }
                    Some(0x22) => {
                        let key = scanner.string()?;
                        if let Some(Frame::Object { pending, .. }) = stack.last_mut() {
                            *pending = Some(key);
                        }
                        mode = Mode::Colon;
                    }
                    _ => {
                        return Err(JsonError::Syntax {
                            msg: "Expecting property name enclosed in double quotes",
                            pos: at,
                        })
                    }
                }
            }
            Mode::Colon => {
                scanner.skip_ws();
                if scanner.peek() != Some(0x3A) {
                    return Err(JsonError::Syntax {
                        msg: "Expecting ':' delimiter",
                        pos: scanner.at,
                    });
                }
                scanner.at += 1;
                mode = Mode::Value;
            }
            Mode::AfterValue => {
                scanner.skip_ws();
                let at = scanner.at;
                let is_object = matches!(stack.last(), Some(Frame::Object { .. }));
                match (is_object, scanner.peek()) {
                    (false, Some(0x2C)) => {
                        scanner.at += 1;
                        mode = Mode::Value;
                    }
                    (true, Some(0x2C)) => {
                        scanner.at += 1;
                        mode = Mode::Key { first: false };
                    }
                    (false, Some(0x5D)) | (true, Some(0x7D)) => {
                        scanner.at += 1;
                        let frame = stack.pop().expect("a frame");
                        if let Some(done) = attach(&mut stack, close_frame(frame)) {
                            scanner.skip_ws();
                            if scanner.at != source.len() {
                                return Err(JsonError::Syntax {
                                    msg: "Extra data",
                                    pos: scanner.at,
                                });
                            }
                            return Ok(done);
                        }
                        mode = Mode::AfterValue;
                    }
                    _ => {
                        return Err(JsonError::Syntax {
                            msg: "Expecting ',' delimiter",
                            pos: at,
                        })
                    }
                }
            }
        }
    }
}
