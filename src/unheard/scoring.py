"""UH-7 to UH-12. Score checkable rules against a tape
set and emit the observations document."""
from __future__ import annotations

from .shapes import classify


def _strings(value):
    """Every string in a call's input, keys and values,
    recursively (UH-7a: the path appears in the input)."""
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for key, item in value.items():
            yield from _strings(key)
            yield from _strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from _strings(item)


def _calls(tape: dict) -> list[dict]:
    return [event for event in tape["events"]
            if event["kind"] == "tool_call"]


def _exercises(shape: dict, tape: dict):
    """Yield (seq, held, used_tool) per exercised event."""
    calls = _calls(tape)
    kind = shape["shape"]
    if kind == "prohibit-tool":
        for call in calls:
            if call["tool"] == shape["tool"]:
                yield call["seq"], False, call["tool"]
    elif kind == "prohibit-path":
        for call in calls:
            blob = "\n".join(_strings(call.get("input")))
            if shape["path"] in blob:
                yield call["seq"], False, call["tool"]
    elif kind == "ordering":
        firsts = [c for c in calls if c["tool"] == shape["first"]]
        seconds = [c for c in calls if c["tool"] == shape["second"]]
        if seconds:
            target = seconds[0]
            held = any(c["seq"] < target["seq"] for c in firsts)
            yield target["seq"], held, target["tool"]
    elif kind == "cap":
        count = 0
        for call in calls:
            if call["tool"] == shape["tool"]:
                count += 1
                yield (call["seq"], count <= shape["limit"],
                       call["tool"])
    elif kind == "first":
        if calls:
            first = min(calls, key=lambda c: c["seq"])
            yield (first["seq"],
                   first["tool"] == shape["tool"],
                   first["tool"])
    elif kind == "required-tool":
        for call in calls:
            touched = any(s.endswith(shape["extension"])
                          for s in _strings(call.get("input")))
            if touched:
                yield (call["seq"], call["tool"] == shape["tool"],
                       call["tool"])


def score(rule_texts: list[str], scored_tapes: list,
          excluded: list[dict], warnings=()) -> dict:
    observations: list[dict] = []
    scores: list[dict] = []
    unmeasured: list[str] = []
    unexercised: list[str] = []
    tapes = [tape for tape, _ in scored_tapes]
    digests = {id(tape): digest for tape, digest in scored_tapes}
    seen: set[str] = set()
    duplicates: list[str] = []
    distinct_texts: list[str] = []
    for text in rule_texts:
        if text in seen:
            if text not in duplicates:
                duplicates.append(text)
            continue
        seen.add(text)
        distinct_texts.append(text)
    for text in distinct_texts:
        shape = classify(text)
        if shape is None:
            unmeasured.append(text)
            scores.append({"rule": text, "shape": "unmeasured",
                           "tapes_exercised": 0,
                           "tapes_total": len(tapes),
                           "events_exercised": 0, "violations": 0})
            continue
        events_exercised = 0
        violations = 0
        tapes_exercised = 0
        for tape in tapes:
            exercised = list(_exercises(shape, tape))
            if exercised:
                tapes_exercised += 1
            for seq, held, used in exercised:
                events_exercised += 1
                quote = f"tape {digests[id(tape)]} seq {seq}"
                if held:
                    observations.append(
                        {"rule": text, "kind": "followed",
                         "quote": quote})
                elif shape["shape"] == "required-tool":
                    violations += 1
                    observations.append(
                        {"rule": text, "kind": "misreading",
                         "quote": f"{quote} used {used}"})
                else:
                    violations += 1
                    observations.append(
                        {"rule": text, "kind": "ignored",
                         "quote": quote})
        if events_exercised == 0:
            unexercised.append(text)
        scores.append({"rule": text, "shape": shape["shape"],
                       "tapes_exercised": tapes_exercised,
                       "tapes_total": len(tapes),
                       "events_exercised": events_exercised,
                       "violations": violations})
    document = {
        "observations": observations,
        "scores": scores,
        "unmeasured": unmeasured,
        "unmeasured_count": len(unmeasured),
        "unexercised": unexercised,
        "tapes": {"read": len(tapes) + len(excluded),
                  "scored": len(tapes), "excluded": excluded},
    }
    if duplicates:
        document["duplicates"] = duplicates
    if warnings:
        document["warnings"] = list(warnings)
    return document
