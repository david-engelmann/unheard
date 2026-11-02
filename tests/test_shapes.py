"""UH-13 to UH-15. The cap and the first step, and the
classification order that protects Round 1."""
from unheard.shapes import classify


def test_the_cap_patterns_are_recognized():
    assert classify("Never call Push more than 2 times") == {
        "shape": "cap", "tool": "Push", "limit": 2}
    assert classify("Do not call Push more than 10 times.") == {
        "shape": "cap", "tool": "Push", "limit": 10}
    assert classify("Call Push at most 2 times") == {
        "shape": "cap", "tool": "Push", "limit": 2}
    assert classify("Use Push at most 0 times") == {
        "shape": "cap", "tool": "Push", "limit": 0}
    assert classify("At most 3 calls to Push") == {
        "shape": "cap", "tool": "Push", "limit": 3}
    assert classify("At most 1 calls to Push.") == {
        "shape": "cap", "tool": "Push", "limit": 1}
    assert classify("Never call Push more than 007 times") == {
        "shape": "cap", "tool": "Push", "limit": 7}
    assert classify("Never call Push more than "
                    "999999999999999999 times") == {
        "shape": "cap", "tool": "Push",
        "limit": 999999999999999999}


def test_cap_near_misses_stay_unmeasured():
    assert classify("Never call Push more than twice") is None
    assert classify("At most three calls to Push") is None
    assert classify(
        "Never call Push more than 2 times on Fridays") is None
    assert classify("Never call Push at most 2 times") is None
    assert classify("At most 2 calls to the Push tool") is None
    assert classify("Call Push at most 1 time") is None
    assert classify("At most 1 call of Push") is None
    assert classify("At most 2 calls of Push") is None
    assert classify("Never call Push more than "
                    "9999999999999999999 times") is None
    assert classify("Never call Push more than "
                    + "9" * 5000 + " times") is None


def test_the_first_step_patterns_are_recognized():
    assert classify("Always start with Lint") == {
        "shape": "first", "tool": "Lint"}
    assert classify("Start with Lint.") == {
        "shape": "first", "tool": "Lint"}
    assert classify("Always begin with Lint") == {
        "shape": "first", "tool": "Lint"}
    assert classify("Begin with Lint") == {
        "shape": "first", "tool": "Lint"}
    assert classify("The first call must be Lint") == {
        "shape": "first", "tool": "Lint"}


def test_first_step_near_misses_stay_unmeasured():
    assert classify("Start with the Lint tool") is None
    assert classify("Always start by running Lint") is None
    assert classify("The first call should be Lint") is None


def test_round_1_classifications_do_not_move():
    assert classify("Never use Grep")["shape"] == "prohibit-tool"
    assert classify("Never call Push") == {
        "shape": "prohibit-tool", "tool": "Push"}
    assert classify("Do not edit secrets/prod.env")["shape"] == \
        "prohibit-path"
    assert classify("Always run Lint before Push")["shape"] == \
        "ordering"
    assert classify("Use Editor for .py files")["shape"] == \
        "required-tool"
