//! UH-6, UH-6a, UH-13 to UH-15, UH-23. The
//! checkable shapes, recognized by stated patterns
//! over a rule's exact text, tried in contract
//! order. The patterns are hand matchers over the
//! text's code points, mirroring the interpreter's
//! regular expressions: the end anchor also stands
//! just before one trailing newline at the very end
//! of the text, a \S run is the complement of
//! CPython's whitespace set, and the one lazy token
//! (a prohibition's path) takes the shortest run
//! its ending allows.

use crate::util::{py_is_space, scalars_of};
use crate::value::Text;

/// One recognized shape, carrying the fields its
/// scoring reads: the shape's name in the scores,
/// and the tool, path, ordering pair, extension, or
/// limit it names.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    ProhibitTool { tool: Text },
    ProhibitPath { path: Text },
    Ordering { first: Text, second: Text },
    RequiredTool { tool: Text, extension: Text },
    Cap { tool: Text, limit: u64 },
    First { tool: Text },
}

impl Shape {
    /// The shape's name in the scores list.
    pub fn name(&self) -> &'static str {
        match self {
            Shape::ProhibitTool { .. } => "prohibit-tool",
            Shape::ProhibitPath { .. } => "prohibit-path",
            Shape::Ordering { .. } => "ordering",
            Shape::RequiredTool { .. } => "required-tool",
            Shape::Cap { .. } => "cap",
            Shape::First { .. } => "first",
        }
    }
}

fn text_of(scalars: &[u32]) -> Text {
    crate::util::text_from_scalars(scalars)
}

/// Whether the scalars at `at` spell the literal,
/// returning the position just past it.
fn literal(scalars: &[u32], at: usize, word: &str) -> Option<usize> {
    let letters: Vec<u32> = word.chars().map(|ch| ch as u32).collect();
    if scalars.len() - at.min(scalars.len()) < letters.len() {
        return None;
    }
    if scalars[at..at + letters.len()] == letters[..] {
        Some(at + letters.len())
    } else {
        None
    }
}

/// The first of the literal forms that stands at
/// `at`, in the pattern's own alternation order.
fn any_literal(scalars: &[u32], at: usize, words: &[&str]) -> Option<usize> {
    words.iter().find_map(|word| literal(scalars, at, word))
}

/// A tool token, [A-Za-z0-9_-]+: the run's end, or
/// None when no token character stands at `at`.
/// The run is greedy, as the pattern's is; the
/// following literal never begins with a token
/// character, so the run never has to give a
/// character back.
fn tool_token(scalars: &[u32], at: usize) -> Option<usize> {
    let mut end = at;
    while end < scalars.len()
        && matches!(scalars[end], 0x30..=0x39 | 0x41..=0x5A | 0x61..=0x7A | 0x5F | 0x2D)
    {
        end += 1;
    }
    if end > at {
        Some(end)
    } else {
        None
    }
}

/// The pattern's ending: an optional period, then
/// the end of the text.
fn finish(scalars: &[u32], at: usize) -> bool {
    scalars.len() == at || (scalars.len() == at + 1 && scalars[at] == 0x2E)
}

/// A lazy \S token with the pattern's ending: the
/// shortest run of non-whitespace whose remainder
/// is empty or a single period. Returns the run's
/// end. The run may not cross a whitespace
/// character, so the search stops at the first one.
fn lazy_token(scalars: &[u32], at: usize) -> Option<usize> {
    let mut end = at;
    loop {
        end += 1;
        if end > scalars.len() || py_is_space(scalars[end - 1]) {
            return None;
        }
        if finish(scalars, end) {
            return Some(end);
        }
    }
}

/// A cap numeral, [0-9]{1,18}: the value and the
/// run's end. A longer digit run is not this shape
/// at any length, because every shorter reading is
/// followed by another digit where the pattern
/// wants a word (UH-13b).
fn numeral(scalars: &[u32], at: usize) -> Option<(u64, usize)> {
    let mut end = at;
    while end < scalars.len() && (0x30..=0x39).contains(&scalars[end]) {
        end += 1;
    }
    let count = end - at;
    if count == 0 || count > 18 {
        return None;
    }
    let mut value: u64 = 0;
    for &digit in &scalars[at..end] {
        value = value * 10 + (digit - 0x30) as u64;
    }
    Some((value, end))
}

/// An extension run for the required-tool shape: a
/// dot and a full \S run, the position past both.
/// The literal that follows the run begins with a
/// space, which no \S character is, so no shorter
/// reading of the run can match either.
fn extension_run(scalars: &[u32], at: usize) -> Option<usize> {
    if scalars.get(at) != Some(&0x2E) {
        return None;
    }
    let mut end = at + 1;
    while end < scalars.len() && !py_is_space(scalars[end]) {
        end += 1;
    }
    if end > at + 1 {
        Some(end)
    } else {
        None
    }
}

fn prohibit_tool(scalars: &[u32]) -> Option<Shape> {
    let at = any_literal(
        scalars,
        0,
        &["Never use ", "Do not use ", "Never call ", "Do not call "],
    )?;
    let end = tool_token(scalars, at)?;
    if finish(scalars, end) {
        Some(Shape::ProhibitTool {
            tool: text_of(&scalars[at..end]),
        })
    } else {
        None
    }
}

fn prohibit_path(scalars: &[u32]) -> Option<Shape> {
    let at = any_literal(
        scalars,
        0,
        &[
            "Never edit ",
            "Do not edit ",
            "Never modify ",
            "Do not modify ",
            "Never touch ",
            "Do not touch ",
        ],
    )?;
    let end = lazy_token(scalars, at)?;
    Some(Shape::ProhibitPath {
        path: text_of(&scalars[at..end]),
    })
}

fn ordering(scalars: &[u32]) -> Option<Shape> {
    let start = any_literal(
        scalars,
        0,
        &["Run ", "Always run ", "Call ", "Always call "],
    )?;
    let first_end = tool_token(scalars, start)?;
    let second_at = literal(scalars, first_end, " before ")?;
    let second_end = tool_token(scalars, second_at)?;
    if finish(scalars, second_end) {
        Some(Shape::Ordering {
            first: text_of(&scalars[start..first_end]),
            second: text_of(&scalars[second_at..second_end]),
        })
    } else {
        None
    }
}

fn required_tool(scalars: &[u32]) -> Option<Shape> {
    let start = any_literal(scalars, 0, &["Use ", "Always use "])?;
    let tool_end = tool_token(scalars, start)?;
    let tool = text_of(&scalars[start..tool_end]);
    if let Some(at) = literal(scalars, tool_end, " for ") {
        if let Some(ext_end) = extension_run(scalars, at) {
            if let Some(end) = literal(scalars, ext_end, " files") {
                if finish(scalars, end) {
                    return Some(Shape::RequiredTool {
                        tool,
                        extension: text_of(&scalars[at..ext_end]),
                    });
                }
            }
        }
    }
    if let Some(at) = literal(scalars, tool_end, " for files ending in ") {
        // The ending form's extension is lazy, like
        // the prohibition's path: the shortest run
        // whose remainder is empty or one period.
        let end = extension_lazy(scalars, at)?;
        return Some(Shape::RequiredTool {
            tool,
            extension: text_of(&scalars[at..end]),
        });
    }
    None
}

/// The ending form's extension: a dot and a lazy
/// \S run under the pattern's ending.
fn extension_lazy(scalars: &[u32], at: usize) -> Option<usize> {
    if scalars.get(at) != Some(&0x2E) {
        return None;
    }
    // The run holds the dot and at least one more
    // character; lazy_token counts from its first
    // character, so require one run character past
    // the dot before the ending may stand.
    let mut end = at + 1;
    loop {
        end += 1;
        if end > scalars.len() || py_is_space(scalars[end - 1]) {
            return None;
        }
        if finish(scalars, end) {
            return Some(end);
        }
    }
}

fn cap(scalars: &[u32]) -> Option<Shape> {
    if let Some(at) = any_literal(scalars, 0, &["Never call ", "Do not call "]) {
        if let Some(tool_end) = tool_token(scalars, at) {
            if let Some(num_at) = literal(scalars, tool_end, " more than ") {
                if let Some((limit, num_end)) = numeral(scalars, num_at) {
                    if let Some(end) = literal(scalars, num_end, " times") {
                        if finish(scalars, end) {
                            return Some(Shape::Cap {
                                tool: text_of(&scalars[at..tool_end]),
                                limit,
                            });
                        }
                    }
                }
            }
        }
    }
    if let Some(at) = any_literal(scalars, 0, &["Call ", "Use "]) {
        if let Some(tool_end) = tool_token(scalars, at) {
            if let Some(num_at) = literal(scalars, tool_end, " at most ") {
                if let Some((limit, num_end)) = numeral(scalars, num_at) {
                    if let Some(end) = literal(scalars, num_end, " times") {
                        if finish(scalars, end) {
                            return Some(Shape::Cap {
                                tool: text_of(&scalars[at..tool_end]),
                                limit,
                            });
                        }
                    }
                }
            }
        }
    }
    if let Some(num_at) = literal(scalars, 0, "At most ") {
        if let Some((limit, num_end)) = numeral(scalars, num_at) {
            if let Some(tool_at) = literal(scalars, num_end, " calls to ") {
                if let Some(tool_end) = tool_token(scalars, tool_at) {
                    if finish(scalars, tool_end) {
                        return Some(Shape::Cap {
                            tool: text_of(&scalars[tool_at..tool_end]),
                            limit,
                        });
                    }
                }
            }
        }
    }
    None
}

fn first_step(scalars: &[u32]) -> Option<Shape> {
    if let Some(at) = any_literal(
        scalars,
        0,
        &[
            "Always start with ",
            "Start with ",
            "Always begin with ",
            "Begin with ",
        ],
    ) {
        if let Some(end) = tool_token(scalars, at) {
            if finish(scalars, end) {
                return Some(Shape::First {
                    tool: text_of(&scalars[at..end]),
                });
            }
        }
    }
    if let Some(at) = literal(scalars, 0, "The first call must be ") {
        if let Some(end) = tool_token(scalars, at) {
            if finish(scalars, end) {
                return Some(Shape::First {
                    tool: text_of(&scalars[at..end]),
                });
            }
        }
    }
    None
}

fn classify_scalars(scalars: &[u32]) -> Option<Shape> {
    prohibit_tool(scalars)
        .or_else(|| prohibit_path(scalars))
        .or_else(|| ordering(scalars))
        .or_else(|| required_tool(scalars))
        .or_else(|| cap(scalars))
        .or_else(|| first_step(scalars))
}

/// Return the rule's shape, or None when
/// unmeasured. The interpreter's end anchor also
/// stands just before one trailing newline at the
/// very end of the text, so a text the patterns
/// miss is tried once more without it.
pub fn classify(text: &Text) -> Option<Shape> {
    let scalars = scalars_of(text.units());
    if let Some(shape) = classify_scalars(&scalars) {
        return Some(shape);
    }
    if scalars.last() == Some(&0x0A) {
        return classify_scalars(&scalars[..scalars.len() - 1]);
    }
    None
}
