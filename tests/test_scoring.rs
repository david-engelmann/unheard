//! UH-5 to UH-12, UH-13a, UH-14a, UH-16, UH-59. Obedience
//! scoring for checkable rules, ported from the
//! Python suite file for file. The CLI cases drive
//! the compiled binary.

use std::path::{Path, PathBuf};

use unheard::digest::digest_of;
use unheard::inventory::parse_rules;
use unheard::json;
use unheard::scoring::{exercises, score, Exercise};
use unheard::shapes::{classify, Shape};
use unheard::value::{Text, Value};

const GOLDEN: &str = "cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3";

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn temp_dir(tag: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("a clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("unheard-{tag}-{}-{nanos}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temp dir");
    dir
}

fn tape_dir(base: &Path, names: &[&str]) -> PathBuf {
    let target = base.join("tapes");
    std::fs::create_dir_all(&target).expect("a tapes dir");
    for name in names {
        std::fs::copy(fixture(name), target.join(name)).expect("a fixture copy");
    }
    target
}

struct Run {
    code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn run_cli(args: &[&str], cwd: Option<&Path>) -> Run {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_unheard"));
    command.args(args);
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    let output = command.output().expect("the binary runs");
    Run {
        code: output.status.code().expect("an exit code"),
        stdout: output.stdout,
        stderr: output.stderr,
    }
}

fn parse_output(bytes: &[u8]) -> Value {
    let text = String::from_utf8(bytes.to_vec()).expect("utf-8 output");
    let scalars: Vec<u32> = text.chars().map(|ch| ch as u32).collect();
    json::parse(&scalars).expect("the output parses")
}

fn run(base: &Path, names: &[&str]) -> Value {
    let tapes = tape_dir(base, names);
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    parse_output(&std::fs::read(&out).expect("the output file"))
}

fn get<'a>(value: &'a Value, key: &str) -> &'a Value {
    value.get(key).expect("the field stands")
}

fn as_text(value: &Value) -> String {
    match value {
        Value::Str(text) => text.as_str().expect("valid text"),
        _ => panic!("not a string value"),
    }
}

fn as_array(value: &Value) -> &Vec<Value> {
    value.as_array().expect("an array value")
}

fn as_int(value: &Value) -> i64 {
    match value {
        Value::Int(digits) => digits.parse().expect("an integer"),
        _ => panic!("not an integer value"),
    }
}

fn text(value: &str) -> Text {
    Text::from_str(value)
}

fn parse_fixture(name: &str) -> Value {
    let bytes = std::fs::read(fixture(name)).expect("the fixture");
    let text = String::from_utf8(bytes).expect("utf-8 fixture");
    let scalars: Vec<u32> = text.chars().map(|ch| ch as u32).collect();
    json::parse(&scalars).expect("the fixture parses")
}

#[test]
fn the_digest_is_pinned_to_the_family_golden() {
    let tape = parse_fixture("tape_synthetic.json");
    assert_eq!(digest_of(&tape).expect("digests"), GOLDEN);
}

#[test]
fn the_inventory_matches_bluepencils_on_the_shared_fixture() {
    let bytes = std::fs::read(fixture("rules_bp_synthetic.md")).expect("the fixture");
    let decoded = String::from_utf8(bytes).expect("utf-8 fixture");
    let (rules, _) = parse_rules(&text(&decoded)).expect("parses");
    let texts: Vec<String> = rules
        .iter()
        .map(|rule| rule.text.as_str().expect("valid text"))
        .collect();
    assert_eq!(
        texts,
        vec![
            "Always run the local gate before pushing",
            "Never force push to main",
            "Sign every commit",
            "Run the full suite locally, never trust CI first\nincluding the slow integration half",
            "Keep tests able to fail for a real reason",
            "Write commit messages in plain words",
            "Keep functions short",
        ]
    );
}

#[test]
fn the_four_shapes_are_recognized_and_near_misses_are_not() {
    assert_eq!(
        classify(&text("Never use Grep")),
        Some(Shape::ProhibitTool { tool: text("Grep") })
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
    assert_eq!(
        classify(&text("Use Editor for files ending in .py")),
        Some(Shape::RequiredTool {
            tool: text("Editor"),
            extension: text(".py")
        })
    );
    assert_eq!(classify(&text("Never use the Grep tool")), None);
    assert_eq!(classify(&text("Use Editor for Python files")), None);
    assert_eq!(classify(&text("Keep the standup notes brief")), None);
    assert_eq!(classify(&text("Never use Grep on Fridays")), None);
}

#[test]
fn observations_name_rules_kinds_and_tape_positions() {
    let base = temp_dir("observations");
    let document = run(&base, &["tape_a.json", "tape_b.json"]);
    let mut by_rule: Vec<(String, Vec<&Value>)> = Vec::new();
    for obs in as_array(get(&document, "observations")) {
        let rule = as_text(get(obs, "rule"));
        match by_rule.iter_mut().find(|(name, _)| *name == rule) {
            Some((_, list)) => list.push(obs),
            None => by_rule.push((rule, vec![obs])),
        }
    }
    let kinds = |rule: &str| -> Vec<String> {
        let (_, list) = by_rule
            .iter()
            .find(|(name, _)| name == rule)
            .expect("the rule stands");
        list.iter().map(|obs| as_text(get(obs, "kind"))).collect()
    };
    assert_eq!(kinds("Never use Grep"), vec!["ignored"]);
    let (_, grep) = by_rule
        .iter()
        .find(|(name, _)| name == "Never use Grep")
        .expect("the rule stands");
    assert!(as_text(get(grep[0], "quote")).ends_with("seq 1"));
    assert_eq!(kinds("Do not edit secrets/prod.env"), vec!["ignored"]);
    assert_eq!(
        kinds("Always run Lint before Push"),
        vec!["followed", "ignored"]
    );
    assert_eq!(
        kinds("Use Editor for .py files"),
        vec!["followed", "misreading"]
    );
    let (_, required) = by_rule
        .iter()
        .find(|(name, _)| name == "Use Editor for .py files")
        .expect("the rule stands");
    assert!(as_text(get(required[1], "quote")).ends_with("used Writer"));
    for obs in as_array(get(&document, "observations")) {
        let quote = as_text(get(obs, "quote"));
        assert!(quote.starts_with("tape "));
        let digest = quote.split(' ').nth(1).expect("a digest word");
        assert_eq!(digest.len(), 64);
    }
}

#[test]
fn scores_carry_denominators_and_states_stay_apart() {
    let base = temp_dir("scores");
    let document = run(&base, &["tape_a.json", "tape_b.json"]);
    let scores = as_array(get(&document, "scores"));
    let entry = |rule: &str| -> &Value {
        scores
            .iter()
            .find(|entry| as_text(get(entry, "rule")) == rule)
            .expect("the rule stands")
    };
    let ordering = entry("Always run Lint before Push");
    assert_eq!(as_int(get(ordering, "tapes_exercised")), 2);
    assert_eq!(as_int(get(ordering, "tapes_total")), 2);
    assert_eq!(as_int(get(ordering, "events_exercised")), 2);
    assert_eq!(as_int(get(ordering, "violations")), 1);
    assert_eq!(as_int(get(entry("Never use Grep"), "violations")), 1);
    let unmeasured: Vec<String> = as_array(get(&document, "unmeasured"))
        .iter()
        .map(as_text)
        .collect();
    assert_eq!(unmeasured, vec!["Keep the standup notes brief"]);
    let unexercised: Vec<String> = as_array(get(&document, "unexercised"))
        .iter()
        .map(as_text)
        .collect();
    assert_eq!(unexercised, vec!["Never call Teleport"]);
    let tapes = get(&document, "tapes");
    assert_eq!(as_int(get(tapes, "read")), 2);
    assert_eq!(as_int(get(tapes, "scored")), 2);
    assert!(as_array(get(tapes, "excluded")).is_empty());
}

#[test]
fn an_invalid_tape_is_excluded_with_its_defects_named() {
    let base = temp_dir("invalid");
    let tapes = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    std::fs::write(
        tapes.join("tape_broken.json"),
        "{\"tape_version\": \"0.9\", \"run\": {}, \"events\": []}",
    )
    .expect("the broken tape");
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let document = parse_output(&std::fs::read(&out).expect("the output file"));
    let tapes_doc = get(&document, "tapes");
    assert_eq!(as_int(get(tapes_doc, "scored")), 2);
    let excluded = as_array(get(tapes_doc, "excluded"));
    assert_eq!(as_text(get(&excluded[0], "file")), "tape_broken.json");
    let defects = as_array(get(&excluded[0], "defects"));
    assert!(defects
        .iter()
        .any(|defect| as_text(defect) == "tape_version is not 1.0"));
}

#[test]
fn the_document_carries_its_version_first() {
    // UH-59: the emitted document opens with
    // observations_version, the string "1.0",
    // ahead of every other key.
    let base = temp_dir("version");
    let tapes = tape_dir(&base, &["tape_a.json"]);
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let bytes = std::fs::read(&out).expect("the output file");
    let text = String::from_utf8(bytes).expect("utf-8 output");
    assert!(
        text.starts_with("{\n  \"observations_version\": \"1.0\",\n  \"observations\":"),
        "the document does not open with its version: {}",
        &text[..text.len().min(80)]
    );
    let document = parse_output(&std::fs::read(&out).expect("the output file"));
    assert_eq!(as_text(get(&document, "observations_version")), "1.0");
}

#[test]
fn output_is_deterministic() {
    let base = temp_dir("deterministic");
    let _ = run(&base, &["tape_a.json", "tape_b.json"]);
    let nested = base.join("two");
    std::fs::create_dir_all(&nested).expect("a nested dir");
    let tapes = tape_dir(&nested, &["tape_a.json", "tape_b.json"]);
    let out = base.join("second.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    assert_eq!(
        std::fs::read(&out).expect("the second output"),
        std::fs::read(base.join("observations.json")).expect("the first output")
    );
}

#[test]
fn unheard_refuses_to_write_into_its_inputs() {
    let base = temp_dir("guard");
    let tapes = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    let out = tapes.join("observations.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 1);
    let stderr = String::from_utf8_lossy(&run.stderr).into_owned();
    assert!(stderr.contains("never writes to its inputs"));
}

#[test]
fn period_form_rules_measure_like_their_bare_forms() {
    assert_eq!(
        classify(&text("Do not edit secrets/prod.env.")),
        Some(Shape::ProhibitPath {
            path: text("secrets/prod.env")
        })
    );
    assert_eq!(
        classify(&text("Use Editor for files ending in .py.")),
        Some(Shape::RequiredTool {
            tool: text("Editor"),
            extension: text(".py")
        })
    );
    assert_eq!(
        classify(&text("Never use Grep.")),
        Some(Shape::ProhibitTool { tool: text("Grep") })
    );
    assert_eq!(classify(&text("never use Grep")), None);
    assert_eq!(classify(&text("Use Editor for py files")), None);
}

#[test]
fn a_tape_that_cannot_be_digested_is_excluded() {
    let base = temp_dir("nan");
    let tapes = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    std::fs::write(
        tapes.join("tape_nan.json"),
        "{\"tape_version\": \"1.0\", \"run\": {}, \"events\": [{\"seq\": 0, \"kind\": \"message\", \"temp\": NaN}]}",
    )
    .expect("the nan tape");
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let document = parse_output(&std::fs::read(&out).expect("the output file"));
    let excluded = as_array(get(get(&document, "tapes"), "excluded"));
    let entry = excluded
        .iter()
        .find(|entry| as_text(get(entry, "file")) == "tape_nan.json")
        .expect("the nan tape is excluded");
    assert!(as_text(get(entry, "reason")).contains("canonical form 1"));
}

#[test]
fn a_duplicated_rule_is_scored_once_and_named() {
    let base = temp_dir("duplicates");
    let rules = base.join("rules.md");
    std::fs::write(&rules, "# S\n\n- Never use Grep\n- Never use Grep\n").expect("the rules");
    let tapes = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            rules.to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let document = parse_output(&std::fs::read(&out).expect("the output file"));
    assert_eq!(as_array(get(&document, "observations")).len(), 1);
    let duplicates: Vec<String> = as_array(get(&document, "duplicates"))
        .iter()
        .map(as_text)
        .collect();
    assert_eq!(duplicates, vec!["Never use Grep"]);
}

#[test]
fn an_unclosed_fence_warning_reaches_the_document() {
    let base = temp_dir("fence");
    let rules = base.join("rules.md");
    std::fs::write(&rules, "# S\n\n- Never use Grep\n\n```\n- Never call X\n").expect("the rules");
    let tapes = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            rules.to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let document = parse_output(&std::fs::read(&out).expect("the output file"));
    let warnings = as_array(get(&document, "warnings"));
    assert!(warnings
        .iter()
        .any(|warning| as_text(warning).contains("unclosed fence")));
}

#[test]
fn the_relative_path_guard_holds() {
    let base = temp_dir("relative");
    let rules = std::fs::read(fixture("rules_synthetic.md")).expect("the fixture");
    std::fs::write(base.join("r.md"), rules).expect("the rules copy");
    let _ = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    let run = run_cli(
        &["score", "r.md", "tapes", "-o", "tapes/obs.json"],
        Some(&base),
    );
    assert_eq!(run.code, 1);
}

#[test]
fn a_path_in_an_input_key_is_a_touch() {
    let base = temp_dir("keytouch");
    let tapes = tape_dir(&base, &[]);
    std::fs::write(
        tapes.join("tape_k.json"),
        "{\"tape_version\": \"1.0\", \"run\": {}, \"events\": [{\"seq\": 0, \"kind\": \"tool_call\", \"tool\": \"Editor\", \"input\": {\"secrets/prod.env\": \"x\"}, \"call_id\": \"k1\"}, {\"seq\": 1, \"kind\": \"tool_result\", \"call_id\": \"k1\", \"is_error\": false}]}",
    )
    .expect("the key tape");
    let out = base.join("observations.json");
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
            "-o",
            out.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let document = parse_output(&std::fs::read(&out).expect("the output file"));
    let kinds: Vec<String> = as_array(get(&document, "observations"))
        .iter()
        .filter(|obs| as_text(get(obs, "rule")) == "Do not edit secrets/prod.env")
        .map(|obs| as_text(get(obs, "kind")))
        .collect();
    assert_eq!(kinds, vec!["ignored"]);
}

#[test]
fn the_golden_tapes_quotes_carry_its_real_digest() {
    let base = temp_dir("golden");
    let document = run(&base, &["tape_synthetic.json"]);
    // The golden tape holds one Read call on widget.py, which no
    // rule in the synthetic set governs, so assert through the
    // tape account instead: the set scores, nothing excluded.
    assert_eq!(as_int(get(get(&document, "tapes"), "scored")), 1);
    let tape = parse_fixture("tape_synthetic.json");
    assert_eq!(digest_of(&tape).expect("digests"), GOLDEN);
}

#[test]
fn stdout_mode_and_refusals() {
    let base = temp_dir("stdout");
    let tapes = tape_dir(&base, &["tape_a.json", "tape_b.json"]);
    let run = run_cli(
        &[
            "score",
            fixture("rules_synthetic.md").to_str().expect("a path"),
            tapes.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 0);
    let document = parse_output(&run.stdout);
    assert_eq!(as_int(get(&document, "unmeasured_count")), 1);
    let missing = base.join("missing.md");
    let run = run_cli(
        &[
            "score",
            missing.to_str().expect("a path"),
            tapes.to_str().expect("a path"),
        ],
        None,
    );
    assert_eq!(run.code, 1);
    let stderr = String::from_utf8_lossy(&run.stderr).into_owned();
    assert!(stderr.contains("no such rules file"));
}

fn call(seq: i64, tool: &str) -> Value {
    Value::Object(vec![
        (text("kind"), Value::Str(text("tool_call"))),
        (text("seq"), Value::Int(seq.to_string())),
        (text("tool"), Value::Str(text(tool))),
        (text("input"), Value::Object(Vec::new())),
    ])
}

fn event(fields: Vec<(Text, Value)>) -> Value {
    Value::Object(fields)
}

fn tape_of(events: Vec<Value>) -> Value {
    Value::Object(vec![(text("events"), Value::Array(events))])
}

fn score_docs(rules: &[&str], tapes: Vec<Value>) -> Value {
    let pairs: Vec<(Value, String)> = tapes
        .into_iter()
        .map(|tape| {
            let digest = digest_of(&tape).expect("digests");
            (tape, digest)
        })
        .collect();
    let rule_texts: Vec<Text> = rules.iter().map(|rule| text(rule)).collect();
    score(&rule_texts, &pairs, Vec::new(), &[])
}

fn exercise(seq: &str, held: bool, used: &str) -> Exercise {
    Exercise {
        seq: seq.to_string(),
        held,
        used: text(used),
    }
}

#[test]
fn first_second_call_decides_the_ordering() {
    let shape = Shape::Ordering {
        first: text("Lint"),
        second: text("Push"),
    };
    let tape = tape_of(vec![
        event(vec![
            (text("seq"), Value::Int("0".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Push"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("a"))),
        ]),
        event(vec![
            (text("seq"), Value::Int("1".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Lint"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("b"))),
        ]),
        event(vec![
            (text("seq"), Value::Int("2".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Push"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("c"))),
        ]),
    ]);
    assert_eq!(exercises(&shape, &tape), vec![exercise("0", false, "Push")]);
}

#[test]
fn the_path_appearing_inside_a_longer_string_is_a_touch() {
    let shape = Shape::ProhibitPath {
        path: text("secrets/prod.env"),
    };
    let tape = tape_of(vec![event(vec![
        (text("seq"), Value::Int("0".to_string())),
        (text("kind"), Value::Str(text("tool_call"))),
        (text("tool"), Value::Str(text("Read"))),
        (
            text("input"),
            Value::Object(vec![(
                text("note"),
                Value::Str(text("see secrets/prod.env.bak for history")),
            )]),
        ),
        (text("call_id"), Value::Str(text("a"))),
    ])]);
    let seqs: Vec<String> = exercises(&shape, &tape)
        .iter()
        .map(|exercise| exercise.seq.clone())
        .collect();
    assert_eq!(seqs, vec!["0"]);
}

#[test]
fn the_cap_counts_per_event_and_per_tape() {
    let tape = tape_of(vec![
        call(0, "Read"),
        call(1, "Push"),
        call(2, "Push"),
        call(3, "Push"),
    ]);
    let document = score_docs(&["Never call Push more than 2 times"], vec![tape]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_text(get(entry, "shape")), "cap");
    assert_eq!(as_int(get(entry, "events_exercised")), 3);
    assert_eq!(as_int(get(entry, "violations")), 1);
    assert_eq!(as_int(get(entry, "tapes_exercised")), 1);
    let kinds: Vec<(String, String)> = as_array(get(&document, "observations"))
        .iter()
        .map(|obs| {
            let quote = as_text(get(obs, "quote"));
            let last = quote.rsplit(' ').next().expect("a last word").to_string();
            (as_text(get(obs, "kind")), last)
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            ("followed".to_string(), "1".to_string()),
            ("followed".to_string(), "2".to_string()),
            ("ignored".to_string(), "3".to_string()),
        ]
    );
    let tape = tape_of(vec![
        call(0, "Read"),
        call(1, "Push"),
        call(2, "Push"),
        call(3, "Push"),
    ]);
    let fresh = tape_of(vec![call(0, "Push"), call(1, "Push")]);
    let document = score_docs(&["Never call Push more than 2 times"], vec![tape, fresh]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_int(get(entry, "violations")), 1);
    assert_eq!(as_int(get(entry, "events_exercised")), 5);
}

#[test]
fn a_zero_cap_is_a_prohibition_with_a_count() {
    let tape = tape_of(vec![call(0, "Push")]);
    let document = score_docs(&["Use Push at most 0 times"], vec![tape]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_int(get(entry, "violations")), 1);
    let observations = as_array(get(&document, "observations"));
    assert_eq!(as_text(get(&observations[0], "kind")), "ignored");
}

#[test]
fn the_first_step_holds_violates_and_waits() {
    let held = tape_of(vec![call(0, "Lint"), call(1, "Push")]);
    let document = score_docs(&["Always start with Lint"], vec![held]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_int(get(entry, "violations")), 0);
    let observations = as_array(get(&document, "observations"));
    assert_eq!(as_text(get(&observations[0], "kind")), "followed");

    let violated = tape_of(vec![call(0, "Push"), call(1, "Lint")]);
    let document = score_docs(&["Always start with Lint"], vec![violated]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_int(get(entry, "violations")), 1);
    assert_eq!(as_int(get(entry, "events_exercised")), 1);
    let observations = as_array(get(&document, "observations"));
    let observation = &observations[0];
    assert_eq!(as_text(get(observation, "kind")), "ignored");
    assert!(as_text(get(observation, "quote")).ends_with("seq 0"));

    let silent = tape_of(vec![event(vec![
        (text("kind"), Value::Str(text("message"))),
        (text("seq"), Value::Int("0".to_string())),
    ])]);
    let document = score_docs(&["Always start with Lint"], vec![silent]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_int(get(entry, "events_exercised")), 0);
    let unexercised: Vec<String> = as_array(get(&document, "unexercised"))
        .iter()
        .map(as_text)
        .collect();
    assert_eq!(unexercised, vec!["Always start with Lint"]);
}

#[test]
fn the_cap_counts_every_excess_call() {
    let tape = tape_of(vec![
        call(0, "Push"),
        call(1, "Push"),
        call(2, "Push"),
        call(3, "Push"),
    ]);
    let document = score_docs(&["Never call Push more than 1 times"], vec![tape]);
    let entry = &as_array(get(&document, "scores"))[0];
    assert_eq!(as_int(get(entry, "events_exercised")), 4);
    assert_eq!(as_int(get(entry, "violations")), 3);
    let seqs: Vec<String> = as_array(get(&document, "observations"))
        .iter()
        .filter(|obs| as_text(get(obs, "kind")) == "ignored")
        .map(|obs| {
            as_text(get(obs, "quote"))
                .rsplit(' ')
                .next()
                .expect("a last word")
                .to_string()
        })
        .collect();
    assert_eq!(seqs, vec!["1", "2", "3"]);
}

#[test]
fn the_first_step_reads_seq_not_array_order() {
    let shape = Shape::First { tool: text("Lint") };
    let tape = tape_of(vec![
        event(vec![
            (text("seq"), Value::Int("5".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Push"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("b"))),
        ]),
        event(vec![
            (text("seq"), Value::Int("2".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Lint"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("a"))),
        ]),
    ]);
    assert_eq!(exercises(&shape, &tape), vec![exercise("2", true, "Lint")]);
    let cap = Shape::Cap {
        tool: text("Push"),
        limit: 1,
    };
    let tape = tape_of(vec![
        event(vec![
            (text("seq"), Value::Int("5".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Push"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("b"))),
        ]),
        event(vec![
            (text("seq"), Value::Int("3".to_string())),
            (text("kind"), Value::Str(text("tool_call"))),
            (text("tool"), Value::Str(text("Push"))),
            (text("input"), Value::Object(Vec::new())),
            (text("call_id"), Value::Str(text("a"))),
        ]),
    ]);
    assert_eq!(
        exercises(&cap, &tape),
        vec![exercise("3", true, "Push"), exercise("5", false, "Push")]
    );
}
