//! The value type tapes are read into and the
//! observations document is built from. Mirrors the
//! values Python's json produces: integers of any
//! size held exactly as their decimal digits, floats
//! as f64, and text as UTF-16 code units, the units
//! the rules are stated in. The shape follows the
//! flighttape crate's value model, which this line's
//! ports share; the type stands in this crate
//! because unheard consumes no sibling crate
//! (UH-18).

/// Text as UTF-16 code units. A string carrying an
/// unpaired surrogate is representable here, as it is
/// in Python; `as_str` returns None for such text.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Text(pub Vec<u16>);

impl Text {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Text {
        Text(value.encode_utf16().collect())
    }

    pub fn from_units(units: Vec<u16>) -> Text {
        Text(units)
    }

    pub fn units(&self) -> &[u16] {
        &self.0
    }

    /// The Rust string, when the units form valid text.
    pub fn as_str(&self) -> Option<String> {
        String::from_utf16(&self.0).ok()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(String),
    Float(f64),
    Str(Text),
    Array(Vec<Value>),
    Object(Vec<(Text, Value)>),
}

impl Value {
    pub fn get(&self, key: &str) -> Option<&Value> {
        // A Python dictionary's lookup: the reader
        // collapses a duplicate key onto its first
        // position with the last value, and this
        // lookup takes the last entry regardless, so
        // hand-built values read as the Python's do.
        match self {
            Value::Object(entries) => entries
                .iter()
                .rev()
                .find(|(name, _)| name.as_str().as_deref() == Some(key))
                .map(|(_, value)| value),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(items) => Some(items),
            _ => None,
        }
    }
}
