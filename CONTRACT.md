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

Built in Round 1, which also added the amendments
below (UH-5a to UH-12a before code, UH-5b and UH-8b
after the reviews, before the repair code).

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

## Round 1 amendments, the stated patterns (recorded before code)

- UH-5a. The tape subset is the one the family reads:
  an object with tape_version 1.0, a run object, and an
  events array whose seq values run consecutively from
  0, whose kinds are in the format's enum, and whose
  tool calls and results satisfy the flighttape FT-4
  pairing rules. A file failing any check is excluded
  with its defects named. A tape's digest is its
  canonical form 1 digest, pinned by the family's
  golden fixture. Test, tests/test_scoring.py.
- UH-6a. The four shapes, matched against a rule's exact
  text with at most a trailing period. Prohibit tool:
  Never use, Do not use, Never call, or Do not call,
  then one tool token. Prohibit path: Never edit, Do not
  edit, Never modify, Do not modify, Never touch, or Do
  not touch, then one path token. Required ordering:
  Run, Always run, Call, or Always call, then a tool
  token, the word before, and a second tool token.
  Required tool: Use or Always use, then a tool token,
  the word for, then an extension token beginning with
  a dot, then the word files; or the words for files
  ending in, then the extension token. Anything else is
  unmeasured. Test, tests/test_scoring.py.
- UH-7a. Exercises are counted per event. For a
  prohibition, every matching call is one exercise and
  one violation. For an ordering, the second tool's
  first call in the tape is the one exercise, held when
  the first tool was called before it. For a required
  tool, every call touching a file of the kind is one
  exercise, held when the tool is the required one. A
  path is touched when the path string appears in the
  call's input; a file of a kind is touched when a
  string value in the call's input ends with the
  extension. Test, tests/test_scoring.py.
- UH-8a. An observation's quote is the words tape, the
  tape's digest, the word seq, and the event's seq. A
  required-tool violation's quote adds the words used
  and the tool actually used. One observation per
  exercised event. Test, tests/test_scoring.py.
- UH-12a. The output document carries the observations
  under the key bluepencil's BP-6 reads, plus a scores
  list in rule order (per rule: its text, its shape or
  unmeasured, tapes exercised of tapes total, events
  exercised, violations), the unmeasured rules, the
  unexercised rules, and the tape set's own account
  (tapes read, tapes scored, tapes excluded with their
  defects named). Test, tests/test_scoring.py.

## Round 1 amendments, after the reviews (recorded before the repair code)

- UH-8b. A rules file may carry the same rule text
  twice. Each distinct text is scored once, in first
  occurrence order; the repeated texts are named in
  the output document's duplicates list, so no event
  is ever counted twice for one sentence. Test,
  tests/test_scoring.py.
- UH-5b. A tape that passes the subset checks but
  cannot be digested under canonical form 1 (a
  non-finite number, a lone surrogate, nesting beyond
  the reader's depth) is excluded like any other
  defective tape, with the reason named in the tape
  account. Test, tests/test_scoring.py.

## Round 2. Two more checkable shapes (UH-13 to UH-16)

- UH-13. A fifth shape, the cap: a rule stating a
  numeric limit on one tool's calls in a session. Its
  stated patterns are "Never call X more than N times",
  "Do not call X more than N times", "Call X at most N
  times", "Use X at most N times", and "At most N calls
  to X", where X is a tool name and N is decimal digits,
  the trailing period optional, the whole rule matching
  and nothing more. A limit written in words is not
  this shape. The shape's name in the scores is cap,
  carrying the tool and the limit. Test,
  tests/test_shapes.py.
- UH-13a. The cap binds within one tape. Every call of
  the tool in the tape is an exercise, counted per
  event like a prohibition (UH-7a): the first N calls
  in the tape held, each call past N one violation and
  one ignored observation quoting that call. A limit
  of zero is a cap like any other; every call violates
  it. Test, tests/test_scoring.py.
- UH-14. A sixth shape, the first step: a rule stating
  which tool opens a session. Its stated patterns are
  "Always start with X", "Start with X", "Always begin
  with X", "Begin with X", and "The first call must be
  X", X a tool name, the trailing period optional, the
  whole rule matching and nothing more. The shape's
  name in the scores is first, carrying the tool.
  Test, tests/test_shapes.py.
- UH-14a. The first step is exercised once in a tape
  holding at least one tool call: held when the tape's
  first tool call is the named tool, one violation and
  one ignored observation quoting that first call
  otherwise. A tape with no tool calls exercises
  nothing, so a set of silent tapes leaves the rule
  unexercised, never violated. Test,
  tests/test_scoring.py.
- UH-15. Classification order is part of the contract:
  the four Round 1 shapes are tried first, in their
  Round 1 order, then the cap, then the first step. No
  rule Round 1 measured changes its shape, and no rule
  matches two shapes. Test, tests/test_shapes.py.
- UH-16. Nothing else changes. The output document
  keeps UH-12a's shape, observations keep UH-8 and
  UH-8a's forms exactly, and a rule matching none of
  the six shapes is unmeasured as before. Test,
  tests/test_scoring.py.
