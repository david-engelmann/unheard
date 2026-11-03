//! The JSON reader (UH-19): a strict iterative
//! scanner over the decoded text, deciding every
//! refusal in text order, as CPython's json.loads
//! does. Its error type carries only the one
//! distinction its consumer names: an integer
//! literal past the digit limit stands apart, and
//! the tape loader collapses every other refusal
//! to its one named refusal. Duplicate keys
//! follow Python dictionary assignment: the last
//! value wins, the first key position is
//! retained. Integers keep their exact digits,
//! and NaN and the infinities read as floats.

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
    /// Any refusal the tape loader names only as
    /// not valid JSON: a syntax error, a string
    /// unterminated or holding a bad escape or a
    /// raw control character, or a container that
    /// would open past the nesting limit.
    Invalid,
    /// An integer literal past the digit limit,
    /// the one refusal the tape loader names apart
    /// from the rest.
    IntTooLong,
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

    fn literal(&mut self, word: &str) -> Result<Value, JsonError> {
        let letters: Vec<u32> = word.chars().map(|c| c as u32).collect();
        if self.chars.len() - self.at < letters.len()
            || self.chars[self.at..self.at + letters.len()] != letters[..]
        {
            return Err(JsonError::Invalid);
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
                        return Err(JsonError::Invalid);
                    }
                };
        }
        self.at = upos + 5;
        if self.at == self.chars.len() {
            return Err(JsonError::Invalid);
        }
        Ok(value as u16)
    }

    fn string(&mut self) -> Result<Text, JsonError> {
        // The caller has seen the opening quote.
        self.at += 1;
        let mut out: Vec<u16> = Vec::new();
        loop {
            let scalar = match self.peek() {
                Some(scalar) => scalar,
                None => return Err(JsonError::Invalid),
            };
            match scalar {
                0x22 => {
                    self.at += 1;
                    return Ok(Text::from_units(out));
                }
                0x5C => {
                    self.at += 1;
                    let escape = match self.peek() {
                        Some(escape) => escape,
                        None => return Err(JsonError::Invalid),
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
                        _ => return Err(JsonError::Invalid),
                    }
                    self.at += 1;
                }
                0x00..=0x1F => {
                    return Err(JsonError::Invalid);
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
                return Err(JsonError::Invalid);
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
                .map_err(|_| JsonError::Invalid)
        } else {
            if int_digits > INT_DIGIT_LIMIT {
                return Err(JsonError::IntTooLong);
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
        return Err(JsonError::Invalid);
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
                let scalar = match scanner.peek() {
                    Some(scalar) => scalar,
                    None => return Err(JsonError::Invalid),
                };
                if mode == Mode::FirstValue && scalar == 0x5D {
                    scanner.at += 1;
                    let frame = stack.pop().expect("an array frame");
                    if let Some(done) = attach(&mut stack, close_frame(frame)) {
                        scanner.skip_ws();
                        if scanner.at != source.len() {
                            return Err(JsonError::Invalid);
                        }
                        return Ok(done);
                    }
                    mode = Mode::AfterValue;
                    continue 'outer;
                }
                let value = match scalar {
                    0x7B => {
                        if stack.len() >= DEPTH_LIMIT {
                            return Err(JsonError::Invalid);
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
                            return Err(JsonError::Invalid);
                        }
                        scanner.at += 1;
                        stack.push(Frame::Array(Vec::new()));
                        mode = Mode::FirstValue;
                        continue 'outer;
                    }
                    0x22 => Value::Str(scanner.string()?),
                    0x74 => scanner.literal("true")?,
                    0x66 => scanner.literal("false")?,
                    0x6E => scanner.literal("null")?,
                    0x4E => scanner.literal("NaN")?,
                    0x49 => scanner.literal("Infinity")?,
                    0x2D => {
                        if scanner.peek_at(1) == Some(0x49) {
                            scanner.literal("-Infinity")?
                        } else {
                            scanner.number()?
                        }
                    }
                    0x30..=0x39 => scanner.number()?,
                    _ => {
                        return Err(JsonError::Invalid);
                    }
                };
                if let Some(done) = attach(&mut stack, value) {
                    scanner.skip_ws();
                    if scanner.at != source.len() {
                        return Err(JsonError::Invalid);
                    }
                    return Ok(done);
                }
                mode = Mode::AfterValue;
            }
            Mode::Key { first } => {
                scanner.skip_ws();
                match scanner.peek() {
                    Some(0x7D) if first => {
                        scanner.at += 1;
                        let frame = stack.pop().expect("an object frame");
                        if let Some(done) = attach(&mut stack, close_frame(frame)) {
                            scanner.skip_ws();
                            if scanner.at != source.len() {
                                return Err(JsonError::Invalid);
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
                        return Err(JsonError::Invalid);
                    }
                }
            }
            Mode::Colon => {
                scanner.skip_ws();
                if scanner.peek() != Some(0x3A) {
                    return Err(JsonError::Invalid);
                }
                scanner.at += 1;
                mode = Mode::Value;
            }
            Mode::AfterValue => {
                scanner.skip_ws();
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
                                return Err(JsonError::Invalid);
                            }
                            return Ok(done);
                        }
                        mode = Mode::AfterValue;
                    }
                    _ => {
                        return Err(JsonError::Invalid);
                    }
                }
            }
        }
    }
}
