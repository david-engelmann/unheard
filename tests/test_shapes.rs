//! UH-13 to UH-15. The cap and the first step, and
//! the classification order that protects Round 1,
//! ported from the Python suite file for file.

use unheard::shapes::{classify, Shape};
use unheard::value::Text;

fn text(value: &str) -> Text {
    Text::from_str(value)
}

fn cap(tool: &str, limit: u64) -> Option<Shape> {
    Some(Shape::Cap {
        tool: text(tool),
        limit,
    })
}

fn first(tool: &str) -> Option<Shape> {
    Some(Shape::First { tool: text(tool) })
}

#[test]
fn the_cap_patterns_are_recognized() {
    assert_eq!(
        classify(&text("Never call Push more than 2 times")),
        cap("Push", 2)
    );
    assert_eq!(
        classify(&text("Do not call Push more than 10 times.")),
        cap("Push", 10)
    );
    assert_eq!(classify(&text("Call Push at most 2 times")), cap("Push", 2));
    assert_eq!(classify(&text("Use Push at most 0 times")), cap("Push", 0));
    assert_eq!(classify(&text("At most 3 calls to Push")), cap("Push", 3));
    assert_eq!(classify(&text("At most 1 calls to Push.")), cap("Push", 1));
    assert_eq!(
        classify(&text("Never call Push more than 007 times")),
        cap("Push", 7)
    );
    assert_eq!(
        classify(&text("Never call Push more than 999999999999999999 times")),
        cap("Push", 999999999999999999)
    );
}

#[test]
fn cap_near_misses_stay_unmeasured() {
    assert_eq!(classify(&text("Never call Push more than twice")), None);
    assert_eq!(classify(&text("At most three calls to Push")), None);
    assert_eq!(
        classify(&text("Never call Push more than 2 times on Fridays")),
        None
    );
    assert_eq!(classify(&text("Never call Push at most 2 times")), None);
    assert_eq!(classify(&text("At most 2 calls to the Push tool")), None);
    assert_eq!(classify(&text("Call Push at most 1 time")), None);
    assert_eq!(classify(&text("At most 1 call of Push")), None);
    assert_eq!(classify(&text("At most 2 calls of Push")), None);
    assert_eq!(
        classify(&text("Never call Push more than 9999999999999999999 times")),
        None
    );
    let long = format!("Never call Push more than {} times", "9".repeat(5000));
    assert_eq!(classify(&text(&long)), None);
}

#[test]
fn the_first_step_patterns_are_recognized() {
    assert_eq!(classify(&text("Always start with Lint")), first("Lint"));
    assert_eq!(classify(&text("Start with Lint.")), first("Lint"));
    assert_eq!(classify(&text("Always begin with Lint")), first("Lint"));
    assert_eq!(classify(&text("Begin with Lint")), first("Lint"));
    assert_eq!(
        classify(&text("The first call must be Lint")),
        first("Lint")
    );
}

#[test]
fn first_step_near_misses_stay_unmeasured() {
    assert_eq!(classify(&text("Start with the Lint tool")), None);
    assert_eq!(classify(&text("Always start by running Lint")), None);
    assert_eq!(classify(&text("The first call should be Lint")), None);
}

#[test]
fn round_1_classifications_do_not_move() {
    assert_eq!(
        classify(&text("Never use Grep")),
        Some(Shape::ProhibitTool { tool: text("Grep") })
    );
    assert_eq!(
        classify(&text("Never call Push")),
        Some(Shape::ProhibitTool { tool: text("Push") })
    );
    assert_eq!(
        classify(&text("Do not edit secrets/prod.env")),
        Some(Shape::ProhibitPath {
            path: text("secrets/prod.env")
        })
    );
    assert_eq!(
        classify(&text("Always run Lint before Push")),
        Some(Shape::Ordering {
            first: text("Lint"),
            second: text("Push")
        })
    );
    assert_eq!(
        classify(&text("Use Editor for .py files")),
        Some(Shape::RequiredTool {
            tool: text("Editor"),
            extension: text(".py")
        })
    );
}
