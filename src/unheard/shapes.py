"""UH-6, UH-6a, UH-13 to UH-15. The checkable shapes,
recognized by stated patterns over a rule's exact text,
tried in contract order."""
from __future__ import annotations

import re

TOOL = r"([A-Za-z0-9_-]+)"
PROHIBIT_TOOL = re.compile(
    rf"^(?:Never use|Do not use|Never call|Do not call) {TOOL}\.?$")
PROHIBIT_PATH = re.compile(
    r"^(?:Never edit|Do not edit|Never modify|Do not modify|"
    r"Never touch|Do not touch) (\S+?)\.?$")
ORDERING = re.compile(
    rf"^(?:Run|Always run|Call|Always call) {TOOL} before {TOOL}\.?$")
REQUIRED_TOOL = re.compile(
    rf"^(?:Use|Always use) {TOOL} for (\.\S+) files\.?$")
REQUIRED_TOOL_ENDING = re.compile(
    rf"^(?:Use|Always use) {TOOL} for files ending in (\.\S+?)\.?$")
NUMERAL = r"([0-9]{1,18})"
CAP = re.compile(
    rf"^(?:Never call|Do not call) {TOOL} more than {NUMERAL} times\.?$")
CAP_AT_MOST = re.compile(
    rf"^(?:Call|Use) {TOOL} at most {NUMERAL} times\.?$")
CAP_COUNT = re.compile(
    rf"^At most {NUMERAL} calls to {TOOL}\.?$")
FIRST = re.compile(
    rf"^(?:Always start with|Start with|Always begin with|"
    rf"Begin with) {TOOL}\.?$")
FIRST_CALL = re.compile(
    rf"^The first call must be {TOOL}\.?$")


def classify(text: str) -> dict | None:
    """Return the rule's shape, or None when unmeasured."""
    match = PROHIBIT_TOOL.match(text)
    if match:
        return {"shape": "prohibit-tool", "tool": match.group(1)}
    match = PROHIBIT_PATH.match(text)
    if match:
        return {"shape": "prohibit-path", "path": match.group(1)}
    match = ORDERING.match(text)
    if match:
        return {"shape": "ordering", "first": match.group(1),
                "second": match.group(2)}
    match = REQUIRED_TOOL.match(text) or REQUIRED_TOOL_ENDING.match(text)
    if match:
        return {"shape": "required-tool", "tool": match.group(1),
                "extension": match.group(2)}
    match = CAP.match(text) or CAP_AT_MOST.match(text)
    if match:
        return {"shape": "cap", "tool": match.group(1),
                "limit": int(match.group(2))}
    match = CAP_COUNT.match(text)
    if match:
        return {"shape": "cap", "tool": match.group(2),
                "limit": int(match.group(1))}
    match = FIRST.match(text) or FIRST_CALL.match(text)
    if match:
        return {"shape": "first", "tool": match.group(1)}
    return None
