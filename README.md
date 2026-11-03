# unheard

It scores standing instructions by observed obedience across a corpus of runs and names the ones agents systematically ignore, with counts, never impressions.

## What this is

Human interventions and ignored instructions are the highest value retro signal and the least recorded, because transcripts treat a correction as an ordinary turn and nothing counts disobedience per rule. Unheard measures the hearing and never edits. Bluepencil does the editing.

The score command reads a rules file and a directory of flighttape tapes. The measured rules are the ones written in one of six checkable shapes, a named tool prohibited, a named path prohibited, a required ordering of two named tools, a required tool for files of a named kind, a numeric cap on one tool's calls in a session, and the tool a session must open with. Every other rule is reported unmeasured, with the count published, and nothing is scored by guesswork. The output is the observations file bluepencil consumes, followed, ignored, and misreading observations citing the tape by digest and the event by seq, plus the scores with their denominators, exercised of total and violations of exercised, never a bare rate. A rule never exercised is a different state from a rule never measured, and the output keeps the two apart.

## Status

Round 1 is closed. The score command measures the
four original shapes and writes the observations file.

Round 2 is closed. Two more shapes are measured, the
cap and the first step, classified only after the
Round 1 four so no earlier measurement changes its
shape. Setup landed earlier in October 2026.

## Programme status

- Closed perfection rounds, 3 of 100. A round counts only when a
  full cycle of work, audit, review, red team, and approval has
  closed.
- Contract, UH-1 to UH-12 as amended, met for the Round 1 scope.
  UH-13 to UH-16 with amendment UH-13b, written ahead of
  the code and met in Round 2. UH-17 to UH-24, the Rust
  port obligations, written ahead of the port and met in
  Round 3.
- Local gate, python3 scripts/codepoint_scan.py and
  cargo test. CI runs the same commands, with fmt and
  clippy ahead of the tests.

## Licence

MIT. See LICENSE.
