"""UH-5. The rules inventory, consumed from bluepencil's
discipline at its pinned behaviour.

Unheard's observations name rules by the exact texts
bluepencil inventories, so the two tools must read a
rules file identically. This module implements
bluepencil's inventory discipline (BP-5, BP-5a, BP-5b,
BP-5c) from the same rules: every bullet or numbered
line under a heading is a rule, fenced and comment
blocks are context, setext headings open sections,
checkboxes are syntax, empty markers and rules before
the first heading refuse by name. The test suite pins
this parser against the same synthetic rules file
bluepencil's own suite uses, with the same inventory
expected, so drift between the two readers fails here.
"""

from __future__ import annotations

import re

HEADING = re.compile(r"^(#{1,6})\s+(.*\S)\s*$")
BULLET = re.compile(r"^(\s*)[-*+]\s+(.*\S)\s*$")
NUMBERED = re.compile(r"^(\s*)\d+[.)]\s+(.*\S)\s*$")
EMPTY_MARKER = re.compile(r"^\s*(?:[-*+]|\d+[.)])\s*$")
FENCE = re.compile(r"^\s*(`{3,}|~{3,})")
SETEXT_1 = re.compile(r"^\s*=+\s*$")
SETEXT_2 = re.compile(r"^\s*-+\s*$")
CLOSING_HASHES = re.compile(r"^(.*?)\s+#+$")
CHECKBOX = re.compile(r"^\[(?: |x|X)\]\s+(.*\S)\s*$")


class InventoryError(ValueError):
    """A named refusal."""


def _heading_text(raw: str) -> str:
    match = CLOSING_HASHES.match(raw)
    return match.group(1) if match else raw


def parse_rules(text: str) -> tuple[list[dict], list[str]]:
    """Every bullet or numbered line under a heading is a
    rule. Returns (rules, warnings).

    Prose is context. Lines inside fenced code blocks and
    HTML comment blocks are never rules. A fence closes
    only on its own character at at least its own length.
    A rule line before the first heading, and a marker
    with no text, are named refusals.
    """
    rules: list[dict] = []
    warnings: list[str] = []
    section_stack: list[tuple[int, str]] = []
    fence: tuple[str, int, int] | None = None
    in_comment = False
    current: dict | None = None
    last_content_line = 0
    pending_prose: str | None = None

    def close_current() -> None:
        nonlocal current
        if current is not None:
            current["line_span"][1] = last_content_line
            current["text"] = "\n".join(current.pop("_lines"))
            current.pop("_indent")
            rules.append(current)
            current = None

    def push_section(level: int, name: str) -> None:
        while section_stack and section_stack[-1][0] >= level:
            section_stack.pop()
        section_stack.append((level, name))

    lines = text.splitlines()
    for idx, line in enumerate(lines, start=1):
        stripped = line.strip()
        if fence is not None:
            run = FENCE.match(line)
            if (run and run.group(1)[0] == fence[0]
                    and len(run.group(1)) >= fence[1]
                    and not line.strip().strip(
                        fence[0]).strip()):
                fence = None
            continue
        if in_comment:
            if "-->" in line:
                in_comment = False
            continue
        if stripped.startswith("<!--"):
            close_current()
            pending_prose = None
            if "-->" not in stripped[4:]:
                in_comment = True
            continue
        run = FENCE.match(line)
        if run:
            close_current()
            pending_prose = None
            fence = (run.group(1)[0], len(run.group(1)), idx)
            continue
        if pending_prose is not None and (
                SETEXT_1.match(line) or SETEXT_2.match(line)):
            close_current()
            push_section(1 if SETEXT_1.match(line) else 2,
                         pending_prose)
            pending_prose = None
            continue
        heading = HEADING.match(line)
        if heading:
            close_current()
            pending_prose = None
            push_section(len(heading.group(1)),
                         _heading_text(heading.group(2)))
            continue
        if EMPTY_MARKER.match(line):
            raise InventoryError(f"empty rule at line {idx}")
        marker = BULLET.match(line) or NUMBERED.match(line)
        if marker:
            if not section_stack:
                raise InventoryError(
                    f"rule outside any section at line {idx}")
            close_current()
            pending_prose = None
            content = marker.group(2)
            box = CHECKBOX.match(content)
            if box:
                content = box.group(1)
            current = {
                "id": f"rule-{len(rules) + 1}",
                "section": [name for _, name in section_stack],
                "line_span": [idx, idx],
                "_lines": [content],
                "_indent": len(marker.group(1)),
            }
            last_content_line = idx
            continue
        if not stripped:
            pending_prose = None
            continue
        indent = len(line) - len(line.lstrip())
        if current is not None and indent > current["_indent"]:
            current["_lines"].append(stripped)
            last_content_line = idx
            continue
        close_current()
        pending_prose = stripped
    close_current()
    if fence is not None:
        warnings.append(
            f"unclosed fence at line {fence[2]}, the rest of "
            "the file was read as code")
    return rules, warnings
