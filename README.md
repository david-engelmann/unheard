# unheard

It scores every instruction by observed obedience across hundreds of runs and names the ones agents systematically ignore, then proposes rewording, relocation, or deletion.

## What this is

Human interventions and ignored instructions are the highest value retro signal and the least recorded, because transcripts treat a correction as an ordinary turn and nothing counts disobedience per rule or role. Unheard counts obedience per instruction across the fleet's history and opens its report with the record of the sentence the operator was proudest of. The point is the fix, not the chart. Each named instruction comes with the change the evidence suggests, reword it, move it, or strike it. The boundary with its sibling is one sentence. Cuesheet shows what the agent could do next, unheard shows what it was told and did not do.

## Status

Seeded in October 2026. Local setup is in progress on the setup
branch. The remote seeding pass is waiting on a token access fix, so
work is banked locally until pushes clear. This repo is in phase 3
of the Retrospective Line and its build waits on the phase gate, the
main chat confirmation that the Gate Truth evidence pack schema has
merged and passed acceptance. Design work may proceed, build does
not start before the gate.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a
  full cycle of work, audit, review, red team, and approval has
  closed.
- Contract, stub in place, not yet met.
- Local gate, python3 scripts/codepoint_scan.py and, once tests
  exist, python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
