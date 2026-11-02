# unheard

It scores standing instructions by observed obedience across a corpus of runs and names the ones agents systematically ignore, with counts, never impressions.

## What this is

Human interventions and ignored instructions are the highest value retro signal and the least recorded, because transcripts treat a correction as an ordinary turn and nothing counts disobedience per rule. Unheard measures the hearing and never edits. Bluepencil does the editing.

The score command reads a rules file and a directory of flighttape tapes. Round 1 scores the rules written in one of four checkable shapes, a named tool prohibited, a named path prohibited, a required ordering of two named tools, and a required tool for files of a named kind. Every other rule is reported unmeasured, with the count published, and nothing is scored by guesswork. The output is the observations file bluepencil consumes, followed, ignored, and misreading observations citing the tape by digest and the event by seq, plus the scores with their denominators, exercised of total and violations of exercised, never a bare rate. A rule never exercised is a different state from a rule never measured, and the output keeps the two apart.

## Status

Round 1 is built and in review on its branch. Setup landed earlier in October 2026.

## Programme status

- Closed perfection rounds, 1 of 100. A round counts only when a
  full cycle of work, audit, review, red team, and approval has
  closed.
- Contract, UH-1 to UH-12 as amended, met for the Round 1 scope.
- Local gate, python3 scripts/codepoint_scan.py and
  python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
