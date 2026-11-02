"""UH-5 to UH-12. Obedience scoring for checkable rules."""
import json
import shutil
from pathlib import Path

from unheard.cli import main
from unheard.digest import digest_of
from unheard.inventory import parse_rules
from unheard.shapes import classify

FIX = Path(__file__).parent / "fixtures"
GOLDEN = "cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3"


def tape_dir(tmp_path, names=("tape_a.json", "tape_b.json")):
    target = tmp_path / "tapes"
    target.mkdir()
    for name in names:
        shutil.copy(FIX / name, target / name)
    return target


def run(tmp_path, names=("tape_a.json", "tape_b.json")):
    out = tmp_path / "observations.json"
    assert main(["score", str(FIX / "rules_synthetic.md"),
                 str(tape_dir(tmp_path, names)),
                 "-o", str(out)]) == 0
    return json.loads(out.read_text())


def test_the_digest_is_pinned_to_the_family_golden():
    tape = json.loads((FIX / "tape_synthetic.json").read_text())
    assert digest_of(tape) == GOLDEN


def test_the_inventory_matches_bluepencils_on_the_shared_fixture():
    rules, _ = parse_rules((FIX / "rules_bp_synthetic.md").read_text())
    assert [r["text"] for r in rules] == [
        "Always run the local gate before pushing",
        "Never force push to main",
        "Sign every commit",
        "Run the full suite locally, never trust CI first\n"
        "including the slow integration half",
        "Keep tests able to fail for a real reason",
        "Write commit messages in plain words",
        "Keep functions short",
    ]


def test_the_four_shapes_are_recognized_and_near_misses_are_not():
    assert classify("Never use Grep") == {
        "shape": "prohibit-tool", "tool": "Grep"}
    assert classify("Do not edit secrets/prod.env") == {
        "shape": "prohibit-path", "path": "secrets/prod.env"}
    assert classify("Always run Lint before Push") == {
        "shape": "ordering", "first": "Lint", "second": "Push"}
    assert classify("Use Editor for .py files") == {
        "shape": "required-tool", "tool": "Editor",
        "extension": ".py"}
    assert classify("Use Editor for files ending in .py") == {
        "shape": "required-tool", "tool": "Editor",
        "extension": ".py"}
    assert classify("Never use the Grep tool") is None
    assert classify("Use Editor for Python files") is None
    assert classify("Keep the standup notes brief") is None
    assert classify("Never use Grep on Fridays") is None


def test_observations_name_rules_kinds_and_tape_positions(tmp_path):
    document = run(tmp_path)
    by_rule = {}
    for obs in document["observations"]:
        by_rule.setdefault(obs["rule"], []).append(obs)
    assert [o["kind"] for o in by_rule["Never use Grep"]] == ["ignored"]
    assert by_rule["Never use Grep"][0]["quote"].endswith("seq 1")
    assert [o["kind"] for o in
            by_rule["Do not edit secrets/prod.env"]] == ["ignored"]
    ordering = by_rule["Always run Lint before Push"]
    assert [o["kind"] for o in ordering] == ["followed", "ignored"]
    required = by_rule["Use Editor for .py files"]
    assert [o["kind"] for o in required] == ["followed", "misreading"]
    assert required[1]["quote"].endswith("used Writer")
    for obs in document["observations"]:
        assert obs["quote"].startswith("tape ")
        digest = obs["quote"].split()[1]
        assert len(digest) == 64


def test_scores_carry_denominators_and_states_stay_apart(tmp_path):
    document = run(tmp_path)
    scores = {s["rule"]: s for s in document["scores"]}
    ordering = scores["Always run Lint before Push"]
    assert ordering["tapes_exercised"] == 2
    assert ordering["tapes_total"] == 2
    assert ordering["events_exercised"] == 2
    assert ordering["violations"] == 1
    assert scores["Never use Grep"]["violations"] == 1
    assert document["unmeasured"] == ["Keep the standup notes brief"]
    assert document["unexercised"] == ["Never call Teleport"]
    assert document["tapes"] == {"read": 2, "scored": 2,
                                  "excluded": []}


def test_an_invalid_tape_is_excluded_with_its_defects_named(tmp_path):
    target = tape_dir(tmp_path)
    (target / "tape_broken.json").write_text(
        json.dumps({"tape_version": "0.9", "run": {}, "events": []}))
    out = tmp_path / "observations.json"
    assert main(["score", str(FIX / "rules_synthetic.md"),
                 str(target), "-o", str(out)]) == 0
    document = json.loads(out.read_text())
    assert document["tapes"]["scored"] == 2
    assert document["tapes"]["excluded"][0]["file"] == "tape_broken.json"
    assert "tape_version is not 1.0" in (
        document["tapes"]["excluded"][0]["defects"])


def test_output_is_deterministic(tmp_path):
    run(tmp_path)
    nested = tmp_path / "two"
    nested.mkdir()
    out = tmp_path / "second.json"
    assert main(["score", str(FIX / "rules_synthetic.md"),
                 str(tape_dir(nested)),
                 "-o", str(out)]) == 0
    assert out.read_bytes() == (tmp_path / "observations.json").read_bytes()


def test_unheard_refuses_to_write_into_its_inputs(tmp_path, capsys):
    target = tape_dir(tmp_path)
    code = main(["score", str(FIX / "rules_synthetic.md"),
                 str(target), "-o", str(target / "observations.json")])
    assert code == 1
    assert "never writes to its inputs" in capsys.readouterr().err


def test_period_form_rules_measure_like_their_bare_forms():
    assert classify("Do not edit secrets/prod.env.") == {
        "shape": "prohibit-path", "path": "secrets/prod.env"}
    assert classify("Use Editor for files ending in .py.") == {
        "shape": "required-tool", "tool": "Editor",
        "extension": ".py"}
    assert classify("Never use Grep.") == {
        "shape": "prohibit-tool", "tool": "Grep"}
    assert classify("never use Grep") is None
    assert classify("Use Editor for py files") is None


def test_a_tape_that_cannot_be_digested_is_excluded(tmp_path):
    target = tape_dir(tmp_path)
    bad = {"tape_version": "1.0", "run": {},
           "events": [{"seq": 0, "kind": "message",
                       "temp": float("nan")}]}
    (target / "tape_nan.json").write_text(json.dumps(bad))
    out = tmp_path / "observations.json"
    assert main(["score", str(FIX / "rules_synthetic.md"),
                 str(target), "-o", str(out)]) == 0
    document = json.loads(out.read_text())
    excluded = {e["file"]: e for e in document["tapes"]["excluded"]}
    assert "canonical form 1" in excluded["tape_nan.json"]["reason"]


def test_a_duplicated_rule_is_scored_once_and_named(tmp_path):
    rules = tmp_path / "rules.md"
    rules.write_text("# S\n\n- Never use Grep\n- Never use Grep\n")
    out = tmp_path / "observations.json"
    assert main(["score", str(rules), str(tape_dir(tmp_path)),
                 "-o", str(out)]) == 0
    document = json.loads(out.read_text())
    assert len(document["observations"]) == 1
    assert document["duplicates"] == ["Never use Grep"]


def test_an_unclosed_fence_warning_reaches_the_document(tmp_path):
    rules = tmp_path / "rules.md"
    rules.write_text("# S\n\n- Never use Grep\n\n```\n- Never call X\n")
    out = tmp_path / "observations.json"
    assert main(["score", str(rules), str(tape_dir(tmp_path)),
                 "-o", str(out)]) == 0
    document = json.loads(out.read_text())
    assert any("unclosed fence" in w for w in document["warnings"])


def test_the_relative_path_guard_holds(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    (tmp_path / "r.md").write_text((FIX / "rules_synthetic.md").read_text())
    tape_dir(tmp_path)
    code = main(["score", "r.md", "tapes", "-o", "tapes/obs.json"])
    assert code == 1


def test_a_path_in_an_input_key_is_a_touch(tmp_path):
    tape = {"tape_version": "1.0", "run": {},
            "events": [
                {"seq": 0, "kind": "tool_call", "tool": "Editor",
                 "input": {"secrets/prod.env": "x"}, "call_id": "k1"},
                {"seq": 1, "kind": "tool_result", "call_id": "k1",
                 "is_error": False}]}
    target = tape_dir(tmp_path, names=())
    (target / "tape_k.json").write_text(json.dumps(tape))
    out = tmp_path / "observations.json"
    assert main(["score", str(FIX / "rules_synthetic.md"),
                 str(target), "-o", str(out)]) == 0
    document = json.loads(out.read_text())
    kinds = [o["kind"] for o in document["observations"]
             if o["rule"] == "Do not edit secrets/prod.env"]
    assert kinds == ["ignored"]


def test_the_golden_tapes_quotes_carry_its_real_digest(tmp_path):
    document = run(tmp_path, names=("tape_synthetic.json",))
    # The golden tape holds one Read call on widget.py, which no
    # rule in the synthetic set governs, so assert through the
    # tape account instead: the set scores, nothing excluded.
    assert document["tapes"]["scored"] == 1
    assert digest_of(json.loads(
        (FIX / "tape_synthetic.json").read_text())) == GOLDEN


def test_stdout_mode_and_refusals(tmp_path, capsys):
    target = tape_dir(tmp_path)
    assert main(["score", str(FIX / "rules_synthetic.md"),
                 str(target)]) == 0
    document = json.loads(capsys.readouterr().out)
    assert document["unmeasured_count"] == 1
    assert main(["score", str(tmp_path / "missing.md"),
                 str(target)]) == 1
    assert "no such rules file" in capsys.readouterr().err


def test_first_second_call_decides_the_ordering():
    from unheard.scoring import _exercises
    shape = {"shape": "ordering", "first": "Lint", "second": "Push"}
    tape = {"events": [
        {"seq": 0, "kind": "tool_call", "tool": "Push", "input": {},
         "call_id": "a"},
        {"seq": 1, "kind": "tool_call", "tool": "Lint", "input": {},
         "call_id": "b"},
        {"seq": 2, "kind": "tool_call", "tool": "Push", "input": {},
         "call_id": "c"}]}
    assert list(_exercises(shape, tape)) == [(0, False, "Push")]


def test_the_path_appearing_inside_a_longer_string_is_a_touch():
    from unheard.scoring import _exercises
    shape = {"shape": "prohibit-path", "path": "secrets/prod.env"}
    tape = {"events": [
        {"seq": 0, "kind": "tool_call", "tool": "Read",
         "input": {"note": "see secrets/prod.env.bak for history"},
         "call_id": "a"}]}
    assert [seq for seq, _, _ in _exercises(shape, tape)] == [0]


def _tape(*events):
    return {"events": list(events)}


def _call(seq, tool):
    return {"kind": "tool_call", "seq": seq, "tool": tool,
            "input": {}}


def _score(rules, tapes):
    from unheard.scoring import score
    pairs = [(tape, digest_of(tape)) for tape in tapes]
    return score(rules, pairs, [])


def test_the_cap_counts_per_event_and_per_tape():
    tape = _tape(_call(0, "Read"), _call(1, "Push"),
                 _call(2, "Push"), _call(3, "Push"))
    document = _score(["Never call Push more than 2 times"],
                      [tape])
    entry = document["scores"][0]
    assert entry["shape"] == "cap"
    assert entry["events_exercised"] == 3
    assert entry["violations"] == 1
    assert entry["tapes_exercised"] == 1
    kinds = [(o["kind"], o["quote"].rsplit(" ", 1)[1])
             for o in document["observations"]]
    assert kinds == [("followed", "1"), ("followed", "2"),
                     ("ignored", "3")]
    fresh = _tape(_call(0, "Push"), _call(1, "Push"))
    document = _score(["Never call Push more than 2 times"],
                      [tape, fresh])
    assert document["scores"][0]["violations"] == 1
    assert document["scores"][0]["events_exercised"] == 5


def test_a_zero_cap_is_a_prohibition_with_a_count():
    tape = _tape(_call(0, "Push"))
    document = _score(["Use Push at most 0 times"], [tape])
    assert document["scores"][0]["violations"] == 1
    assert document["observations"][0]["kind"] == "ignored"


def test_the_first_step_holds_violates_and_waits():
    held = _tape(_call(0, "Lint"), _call(1, "Push"))
    document = _score(["Always start with Lint"], [held])
    assert document["scores"][0]["violations"] == 0
    assert document["observations"][0]["kind"] == "followed"

    violated = _tape(_call(0, "Push"), _call(1, "Lint"))
    document = _score(["Always start with Lint"], [violated])
    entry = document["scores"][0]
    assert entry["violations"] == 1
    assert entry["events_exercised"] == 1
    observation = document["observations"][0]
    assert observation["kind"] == "ignored"
    assert observation["quote"].endswith("seq 0")

    silent = _tape({"kind": "message", "seq": 0})
    document = _score(["Always start with Lint"], [silent])
    assert document["scores"][0]["events_exercised"] == 0
    assert document["unexercised"] == ["Always start with Lint"]
