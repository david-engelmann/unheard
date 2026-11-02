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
