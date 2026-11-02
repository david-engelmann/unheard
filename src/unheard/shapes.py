"""UH-6, UH-6a. The four checkable shapes, recognized by
stated patterns over a rule's exact text."""
from __future__ import annotations

import re

TOOL = r"([A-Za-z0-9_-]+)"
PROHIBIT_TOOL = re.compile(
    rf"^(?:Never use|Do not use|Never call|Do not call) {TOOL}\.?$")
PROHIBIT_PATH = re.compile(
    r"^(?:Never edit|Do not edit|Never modify|Do not modify|"
    r"Never touch|Do not touch) (\S+)\.?$")
ORDERING = re.compile(
    rf"^(?:Run|Always run|Call|Always call) {TOOL} before {TOOL}\.?$")
REQUIRED_TOOL = re.compile(
    rf"^(?:Use|Always use) {TOOL} for (\.\S+) files\.?$")
REQUIRED_TOOL_ENDING = re.compile(
    rf"^(?:Use|Always use) {TOOL} for files ending in (\.\S+)\.?$")


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
    return None
