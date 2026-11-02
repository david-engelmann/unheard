# unheard Contract, stub

The full Contract is the first build act of this repo phase. This
stub records the obligations the setup already knows it owes. Each
obligation becomes executable before any feature code that depends
on it lands.

- UH-1. Every standing instruction in scope is scored by observed obedience across the corpus, with counts, never impressions.
- UH-2. Instructions agents systematically ignore are named, with the evidence count beside each one.
- UH-3. Unheard measures the hearing and never edits. Bluepencil does the editing.
- UH-4. Every fixture and corpus sample committed to this repo is synthetic.

## Round 1. Obedience scoring for checkable rules (UH-5 to UH-12)

Contract preparation only in this pass. The build opens
when this repo's turn in the line's build order arrives.

- UH-5. Inputs are a rules file and a directory of
  flighttape tapes at format 1.0, read with the family's
  exclusion discipline, invalid tapes excluded with their
  defects named. The rules file parses into the same
  inventory discipline bluepencil uses: every bullet or
  numbered line under a heading is a rule, identified by
  its exact text. Test, tests/test_scoring.py.
- UH-6. Round 1 scores only rules written in one of four
  checkable shapes, recognized by stated patterns: a
  prohibition of a named tool, a prohibition of a named
  path, a required ordering of two named tools, and a
  required tool for files of a named kind. A rule matching
  no shape is reported as unmeasured, with the count of
  unmeasured rules published beside the scores. Nothing
  is scored by guesswork. Test, tests/test_scoring.py.
- UH-7. A rule is exercised in a tape when its antecedent
  occurs: the tool is called, the path is touched, the
  second tool is attempted, a file of the kind is
  touched. An exercised case is a violation when the
  constraint failed in that tape. Exercise and violation
  counts aggregate across the set, per rule.
  Test, tests/test_scoring.py.
- UH-8. The output is an observations file in the shape
  bluepencil's BP-6 consumes: each observation names a
  rule by exact text, a kind, and a quote that names the
  tape by digest and the event by seq. Exercised cases
  that held become followed observations; violations
  become ignored observations, except a required-tool
  violation, which becomes a misreading observation
  quoting the tool actually used. Test,
  tests/test_scoring.py.
- UH-9. A rule with exercises and no violations across
  the set is reported with its counts and no adverse
  observation invented for it; a rule with no exercises
  is reported unexercised, a different state from
  unmeasured, and the output keeps the two apart.
  Test, tests/test_scoring.py.
- UH-10. The observations file is deterministic: the
  same rules file and tape set produce byte-identical
  output, tapes read in filename order, rules in file
  order. Test, tests/test_scoring.py.
- UH-11. Unheard never writes to the rules file or the
  tapes. Its only output is the observations file, to the
  path named on the command line or to stdout, written
  atomically, and a write failure is a named error with
  exit 1. Test, tests/test_scoring.py.
- UH-12. Scores are reported as counts with their
  denominators, exercised of total tapes and violations
  of exercised, never as a bare rate. Test,
  tests/test_scoring.py.
