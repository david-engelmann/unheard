//! Canonical form 1 digests, consumed pinned
//! (UH-5a, UH-21).
//!
//! Citations name a tape by its digest under
//! flighttape canonical form 1, the SHA-256 of the
//! canonical form defined by the flighttape
//! contract obligations FT-26 to FT-28: object keys
//! sorted by UTF-16 code units, numbers rendered as
//! ECMAScript renders them, no whitespace. This
//! module renders that form from the crate's value
//! model so unheard can cite the tapes it reads. It
//! is a consumption of the pinned format, never a
//! fork of it: the test suite pins the digest of a
//! known tape to the golden value the flighttape
//! programme published for it, so any drift from
//! the form fails loudly here.
//!
//! The float digits come from the Python digest's
//! own source (CPython repr digits under the
//! ECMAScript case split), as the Python's
//! _render_number reads them off repr.
//!
//! A value that cannot be rendered is a
//! DigestError: a non-finite number, text that
//! cannot be encoded (a lone surrogate in a string
//! or a key, which the Python's UTF-16BE sort key
//! refuses the same way), an integer past the
//! interpreter's digit limit, or a value whose
//! render peak passes the renderer's budget
//! (UH-21). The peak stands in for the
//! interpreter recursion limit the Python's
//! renderer hits: every node carries a peak of
//! its depth plus the extra its own rendering
//! costs, two for a string (rendered through a
//! nested encoder call), one for a float
//! (rendered through a helper call), two for an
//! object holding at least one key (its keys
//! render through the nested encoder call), and
//! nothing for anything else, and a peak past
//! 993 refuses.

use sha2::{Digest, Sha256};

use crate::emit::{escape_string, py_float, Mode};
use crate::value::{Text, Value};

/// The peak budget of a render (UH-21): the
/// root counts as depth one, every node's peak
/// is its depth plus its rendering extra (see
/// the module doc), and a peak past this budget
/// refuses. It stands in for the interpreter
/// recursion limit the Python's renderer hits.
const PEAK_LIMIT: usize = 993;

#[derive(Debug)]
pub struct DigestError;

fn render_number(value: f64) -> String {
    // The Python digest's renderer: repr gives the
    // shortest digits that round trip; from them,
    // point is the count of digits before the decimal
    // point when the value is written out in decimal,
    // and the four cases are the ECMAScript number to
    // string cases.
    if value == 0.0 {
        return "0".to_string();
    }
    let sign = if value < 0.0 { "-" } else { "" };
    let text = py_float(value.abs());
    let (mantissa, exp_part) = match text.split_once('e') {
        Some((m, e)) => (
            m.to_string(),
            Some(e.parse::<i64>().expect("repr exponent")),
        ),
        None => (text, None),
    };
    let (whole, fraction) = match mantissa.split_once('.') {
        Some((w, f)) => (w.to_string(), f.to_string()),
        None => (mantissa, String::new()),
    };
    let raw = format!("{whole}{fraction}");
    let digits_trimmed = raw.trim_start_matches('0');
    let exp10 = match exp_part {
        Some(e) => e,
        None => {
            let leading = (raw.len() - digits_trimmed.len()) as i64;
            whole.len() as i64 - leading - 1
        }
    };
    let mut digits = digits_trimmed.trim_end_matches('0').to_string();
    if digits.is_empty() {
        digits = "0".to_string();
    }
    let point = exp10 + 1;
    let count = digits.len() as i64;
    if count <= point && point <= 21 {
        format!("{sign}{digits}{}", "0".repeat((point - count) as usize))
    } else if 0 < point && point <= 21 {
        let at = point as usize;
        format!("{sign}{}.{}", &digits[..at], &digits[at..])
    } else if -6 < point && point <= 0 {
        format!("{sign}0.{}{digits}", "0".repeat((-point) as usize))
    } else {
        let head = if count == 1 {
            digits.clone()
        } else {
            format!("{}.{}", &digits[..1], &digits[1..])
        };
        format!(
            "{sign}{head}e{}{}",
            if exp10 >= 0 { "+" } else { "-" },
            exp10.abs()
        )
    }
}

struct Renderer<'a> {
    out: &'a mut Vec<u8>,
}

impl Renderer<'_> {
    /// Render one value standing at `depth`, the
    /// root counting as one. A node whose peak,
    /// its depth plus its rendering extra, passes
    /// the budget refuses before it renders.
    fn render(&mut self, value: &Value, depth: usize) -> Result<(), DigestError> {
        let extra = match value {
            // A string renders through a nested
            // encoder call.
            Value::Str(_) => 2,
            // A float renders through a helper call.
            Value::Float(_) => 1,
            // An object holding at least one key
            // renders its keys through the nested
            // encoder call; an empty object does not.
            Value::Object(entries) if !entries.is_empty() => 2,
            _ => 0,
        };
        if depth + extra > PEAK_LIMIT {
            return Err(DigestError);
        }
        match value {
            Value::Null => self.out.extend_from_slice(b"null"),
            Value::Bool(true) => self.out.extend_from_slice(b"true"),
            Value::Bool(false) => self.out.extend_from_slice(b"false"),
            Value::Str(text) => {
                escape_string(text, Mode::Strict, self.out).map_err(|_| DigestError)?;
            }
            Value::Int(digits) => {
                // The Python renders an int through
                // str, which refuses past the
                // interpreter's digit limit; the tape
                // reader accepts integer literals only
                // to the same limit, so a longer digit
                // string can only arrive built by hand.
                if digits.trim_start_matches('-').len() > 4300 {
                    return Err(DigestError);
                }
                self.out.extend_from_slice(digits.as_bytes());
            }
            Value::Float(number) => {
                if !number.is_finite() {
                    return Err(DigestError);
                }
                self.out
                    .extend_from_slice(render_number(*number).as_bytes());
            }
            Value::Array(items) => {
                self.out.push(b'[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        self.out.push(b',');
                    }
                    self.render(item, depth + 1)?;
                }
                self.out.push(b']');
            }
            Value::Object(entries) => {
                // The Python sorts keys by their
                // UTF-16BE encoding; a lone surrogate
                // key fails that encode, which is its
                // cannot be encoded refusal.
                let mut keyed: Vec<(&Text, &Value)> = Vec::with_capacity(entries.len());
                for (key, item) in entries {
                    if !key_is_encodable(key) {
                        return Err(DigestError);
                    }
                    keyed.push((key, item));
                }
                keyed.sort_by(|a, b| a.0.units().cmp(b.0.units()));
                self.out.push(b'{');
                for (index, (key, item)) in keyed.iter().enumerate() {
                    if index > 0 {
                        self.out.push(b',');
                    }
                    escape_string(key, Mode::Strict, self.out).map_err(|_| DigestError)?;
                    self.out.push(b':');
                    self.render(item, depth + 1)?;
                }
                self.out.push(b'}');
            }
        }
        Ok(())
    }
}

fn key_is_encodable(text: &Text) -> bool {
    let units = text.units();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xD800..=0xDBFF).contains(&unit) {
            if index + 1 >= units.len() || !(0xDC00..=0xDFFF).contains(&units[index + 1]) {
                return false;
            }
            index += 2;
        } else if (0xDC00..=0xDFFF).contains(&unit) {
            return false;
        } else {
            index += 1;
        }
    }
    true
}

/// The value's canonical form 1 as UTF-8 bytes.
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>, DigestError> {
    let mut out = Vec::new();
    let mut renderer = Renderer { out: &mut out };
    renderer.render(value, 1)?;
    Ok(out)
}

/// The SHA-256 hex digest of a value under
/// canonical form 1.
pub fn digest_of(value: &Value) -> Result<String, DigestError> {
    let bytes = canonical_bytes(value)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}
