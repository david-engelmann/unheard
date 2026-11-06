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

## Round 2 post-review amendments, recorded before the repairs

- UH-13b. The cap's N is one to eighteen ASCII decimal
  digits, read at face value, leading zeros standing
  as ordinary notation. A longer numeral, and digits
  in any other script, are not this shape, so the rule
  is unmeasured. Eighteen digits passes any count of
  events a tape can hold, so no real limit is lost;
  the bound keeps the numeral convertible on every
  supported interpreter, where an unbounded one raised
  an uncaught error and killed the whole run in the
  reviews' hands, and ASCII keeps the stated pattern
  surface exact. Test, tests/test_shapes.py.

## Round 3. The Rust port (UH-17 to UH-24)

Round 3 ports unheard to Rust. The Python on main at
03658d9 is the behavioral spec; the port reproduces
its behavior exactly, and every place exact
reproduction is impossible or a crash of the
interpreter is the behavior is named as a deviation
in the round dossier.

- UH-17. Exact behavior is the obligation: both
  refusal orders and texts, the argparse surface,
  error text, exit codes, and output bytes are the
  Python's, including the crash class, the cases
  where the interpreter dies with a traceback (the
  output guard resolving a path through a symlink
  loop among them) and the port exits 1 silently.
  Test, the whole ported suite and the differential
  harness.
- UH-18. Library crate plus a thin binary, the
  family shape. No sibling crate is consumed: the
  flighttape crate's reader counts a deeply nested
  file whole and so cannot mirror this surface's
  parse refusal (UH-19), and the format is consumed
  pinned by the golden digest exactly as digest.py
  consumed it, by re-implementation under the
  golden test. The only dependency is sha2. Cargo
  tests are the primary suite. No Python bindings
  are offered; unheard is a process-boundary CLI
  and no Python consumer exists. Tests,
  tests/test_scoring.rs and tests/test_shapes.rs.
- UH-19. The tape reader is one strict iterative
  JSON reader whose refusals are decided in parse
  order: a file nested past 9994 containers in
  total is not valid JSON, and an integer literal
  past 4300 digits as written is a number too
  large to read, whichever the text reaches first.
  Duplicate keys keep the last value at the first
  key's position; NaN and Infinity read as floats;
  the minus zero literal normalizes. Tape bytes
  decode as strict UTF-8 with no signature
  stripping; a decode failure names the file not
  valid UTF-8, and an unreadable file cannot be
  read. Test, tests/test_scoring.rs.
- UH-20. Tape defects are computed in the Python's
  order with its phrases: the object, version, run,
  and events checks; an empty events array ends
  the checks; per event the seq, kind, and pairing
  checks, with a call id's repr rendered as
  CPython renders it. Sidechain events are checked
  like any other and stay invisible to the shapes,
  as Rounds 1 and 2 carried. Test,
  tests/test_scoring.rs.
- UH-21. A scored tape is cited by its canonical
  form 1 digest, computed locally with CPython
  repr-exact digits over the value model. The
  digest refuses, and the tape is excluded as
  unable to be cited, on a non-finite number, on
  text that cannot be encoded, or on a value
  nested past 992 containers in total. The golden
  fixture digest stands pinned. Test,
  tests/test_scoring.rs.
- UH-22. The rules file parses under the same
  inventory discipline bluepencil uses, the shared
  synthetic fixture carrying the same expectation
  in both suites. The file decodes as utf-8-sig;
  a read or decode failure is the named cannot
  read refusal, and an inventory refusal is named.
  Test, tests/test_scoring.rs.
- UH-23. The six shapes classify by hand matchers
  reproducing the stated patterns exactly, in
  contract order: the tool token, the lazy path
  token, the extension runs, the cap numeral at
  one to eighteen ASCII digits (UH-13b), and the
  end anchor's trailing newline rule. Scoring
  follows UH-7 to UH-16: exercises per event,
  quotes naming tape digest and seq, duplicates
  scored once and named, the document's field
  order, tapes iterated in filename order with
  directories and non-files skipped. The same
  tape filed twice counts twice, as carried.
  Tests, tests/test_scoring.rs and
  tests/test_shapes.rs.
- UH-24. The command surface is one subcommand,
  score, with argparse behavior pinned at 80
  columns. The output guard compares resolved
  paths (the output naming the rules file, the
  tapes directory, or a path inside it), and an
  unresolvable path is the crash class of UH-17.
  The document renders before any temporary file
  exists; the write is mkstemp-shaped and atomic.
  All 28 Python tests are ported file for file,
  and a differential harness exercises the whole
  command surface against the Python. The port
  report stands in the round dossier. Tests,
  tests/test_scoring.rs and tests/test_shapes.rs.
- UH-25. Round 4 settles the output guard's
  remaining shapes against the oracle. The
  cancelled loop and the dangling family (single
  link and chains) are probed at the oracle CLI:
  this oracle's guard lets pathlib's resolve
  raise uncaught on a genuine loop (the crash
  class named in UH-21), and the port takes the
  oracle's outcome on every shape, repairing
  toward the finished-path and iterated-splice
  forms its siblings reached where it diverges.
  Whether the oracle's guard also wraps an
  exists() arm (bluepencil's BP-27a shape) is
  read from its source and probed, and mirrored
  if present.
- UH-26. The hardlink question is decided and
  recorded: the oracle's guard carries no
  file-identity arm, and a hardlinked output
  overwrites the rules file on both twins. The
  port stays faithful; no identity arm is added,
  because this repo's contract is exact
  reproduction and the divergence would be a
  behavior change wearing a repair's clothes.
  The shape is pinned by a harness cell so any
  future change is deliberate, and this clause
  is the decision record.
- UH-27. The round's verification is the full
  local gate plus the differential harness at
  zero failures, the new cells included.
  CodeRabbit's advisory pass on Round 3's PR 4
  posted no findings (zero entries on the review
  endpoints), so no disposition is owed from it.
- UH-25a. Amendment, from the Round 4 reviews.
  The walk this oracle performs is recursive in
  the interpreter, one frame per nested splice,
  under the default recursion limit, and at the
  shipped CLI that budget is measured: 990
  nested splices resolve, the 991st raises
  RecursionError out of resolve() (the crash
  class). The bound is on nesting depth, not
  splice count. The port's walk carries the
  same budget: a resolution whose nesting
  passes the measured bound fails, and the
  guard reads the failure as it reads a loop.
  The number is the shipped CLI's, recorded as
  measured; a differently invoked interpreter
  could shift it. This amendment also corrects
  UH-25 in two places: the crash class is
  UH-17's, not UH-21's, and the repair the
  round landed is a transcription of the
  interpreter's walk under this budget, not
  the sibling composite UH-25 sketched.
- UH-28. Round 5 repairs the deleted-working-
  directory shape at the output guard, filed by
  secondbill's Round 4 red team and carried in
  this line's owed file: from a deleted working
  directory, a relative output spelling makes
  the interpreter's resolve raise uncaught (the
  crash class this crate already carries for
  loops), while the port's resolver falls back
  to a manufactured prefix and can compare a
  fiction. The repair: a failed current_dir
  read fails the resolution for relative
  spellings (the crash class follows); absolute
  spellings never consult the working directory.
  SB-25a in the sibling is the shape of the fix,
  but this crate's call order is its own: the
  guard's rules is_file and tapes is_dir checks
  precede resolution, and the probe pass fixes
  which of them already crash or refuse from a
  deleted cwd before the repair is written.
- UH-29. The differential harness gains a
  deleted-cwd launch mode and the cells (a
  relative output spelling onto the rules or
  tapes, a relative spelling away, an absolute
  spelling away, an absolute spelling onto),
  with the control run against the pre-repair
  binary failing exactly the new cells. The
  standing harness stays clean.
- UH-30. README records 5 of 100 only in the
  closure commit written after this round's
  three reviews, fixes, and owner verification.
- UH-31. Round 6 probes two write-seam
  families at this crate's single command,
  both filed before this round opened.
  First, the tailed output spelling (filed at
  Round 5): whether the write consumes the
  raw argv spelling or the pathlib
  constructed spelling (empty and dot
  components dropped wherever they stand, a
  leading separator pair standing as the
  anchor), measured over the trailing
  separator and dot forms, an overwrite under
  a tail, a missing name under a tail, a
  directory under a tail, the collapsed
  spellings, and the boundary pins, against
  the oracle. Second, the temp placement law
  (measured in secondbill's Round 5 and the
  Round 6 rounds of retroforge, wallofwrong,
  and bluepencil): the oracle's write creates
  its temp through tempfile.mkstemp, which
  absolutizes the parent lexically (cwd join,
  then normpath, no link resolution), so the
  popped-link family is probed here too (an
  output whose parent contains a symlink
  component that a following dotdot pops,
  across the existence boundary, in plain,
  tailed, absolute, deep, and escape-link
  spellings). This crate's guard resolves the
  output's normalized spelling and the oracle
  crashes uncaught on a guard loop (the UH-25
  posture), so the probes record for each
  shape whether the guard is reached and
  what it does, as the sibling rounds did.
  Each family is measured against the oracle
  before any repair is written; repairs
  follow the measurements under new clauses,
  each family with its own control run
  against the pre-repair binary; the harness
  gains the proven cells with boundary pins
  ordered before convicted cells where a
  one-sided write could echo through later
  snapshots.
- UH-32. The CodeRabbit disposition owed from
  Round 5 discharges here on evidence: the
  Round 5 pull request carries zero inline
  findings on read-back (checked 2026-11-04 in
  the rotation-wide late sweep), so there is
  nothing to fix and the absence is the
  disposition.
- UH-33. README records 6 of 100 only in the
  closure commit written after this round's
  three reviews, fixes, and owner
  verification.
- UH-34. The tailed-spelling family, measured
  under UH-31 (probe table in the round's
  lane record): the write consumes the
  pathlib constructed spelling, never the raw
  argv spelling. The oracle builds Path(output)
  once and hands the constructed path to the
  guard and the write alike, so the temp's
  parent, the temp's prefix name, and the
  replace target all follow the construction
  (empty and dot components dropped wherever
  they stand, dotdot standing whole, a leading
  separator pair standing as the anchor). The
  port's emit handed the raw spelling to its
  write, whose rename the kernel refuses on a
  trailing separator or a trailing dot
  component: fresh and overwrite spellings
  under a slash, dot, and double-slash tail
  (six shapes) are refused cannot-write by
  the port where the oracle writes. The
  repair hands the constructed path to the
  write; the guard and the cannot-write
  message already consume the construction
  and do not move.
- UH-35. The harness gains the tailed cells of
  UH-34: the trailing separator and dot forms
  fresh and over an existing output, the
  double-slash tail, the missing name and the
  directory under a tail, the collapsed
  spellings, and the boundary pins (a trailing
  dotdot through a real and a missing
  directory, a dotdot followed by a name,
  three dots as a name, and the guard shapes
  under tails), with the control run against
  the pre-repair binary failing exactly the
  six convicted cells.
- UH-36. The temp placement law, measured
  under UH-31: the write's temp is created
  under the lexically absolutized parent of
  the constructed path. The oracle's
  _write_atomically hands tempfile.mkstemp
  the constructed parent as a string, and
  mkstemp absolutizes it lexically (cwd join,
  then posixpath.normpath, no link
  resolution) before creating the temp; the
  port created its temp under the spelled
  parent, so the kernel resolved a symlink
  component physically before a following
  dotdot popped it. The convicted family is
  the popped-link shapes where only the
  physically resolved parent exists (plain,
  overwrite, absolute, deep, escape-link,
  two links in sequence, and a dotdot
  carrying target): the port writes at exit 0
  where the oracle refuses cannot-write. The
  repair creates the temp under the lexical
  parent so computed; the rename target
  remains the constructed path itself.
- UH-37. The boundary agreements of the temp
  law: where the lexical parent exists and
  the physical does not, both sides refuse
  (the oracle at its replace, the port at
  its temp creation); where both exist, both
  write at the physical target; where neither
  exists, both refuse; a real-directory pop
  leaves the parents coincident and agrees
  either way; a lexical parent standing as a
  plain file refuses on both sides. A failed
  cwd read under a relative parent fails the
  write step on the oracle's terms (mkstemp's
  own raise reaching its cannot-write catch);
  that arm is unreachable behind the guard,
  whose resolve dies on the same failed read
  first (UH-28), and stands for totality; an
  absolute parent never consults the cwd.
  The tailed only-physical shapes refuse on
  both sides before and after either single
  repair, because the tail defect and the
  temp defect each refuse alone; they are
  carried as composed cells under UH-38.
- UH-38. The harness gains the popped-link
  cells of UH-36: the convicted shapes and
  the boundary pins of UH-37 ordered before
  them, the guard cells (pops onto the rules
  file and into the tapes directory refused
  at the guard, a cancelled loop passing the
  guard, a standing loop in the guard's crash
  class), and the composed tail-under-pop
  cells, with control runs against the
  pre-repair binary separated per family:
  the temp family's control fails exactly
  its seven convicted cells, and the
  composed cells fail under either family's
  control accounting, never counted twice in
  one family's total.
- UH-39. Correction to UH-38's composed-cell
  sentence, on the spec review's measurement.
  The combined control against the pre-repair
  binary fails exactly thirteen steps (the
  six tailed and the seven popped) and the
  composed cells pass it, invisible there
  because each defect masks the other. The
  composed cells fail under exactly one
  half-repair control: with the tail repaired
  and the temp live they diverge (nine
  failures: the seven popped and the two
  composed); with the temp repaired and the
  tail live they pass (six failures: the six
  tailed). The temp defect is the composed
  cells' sole producing cause, so they are
  carried in the temp family's accounting,
  and UH-38's "fail under either family's
  control accounting" is superseded by this
  sentence wherever the two disagree.
- UH-40. This round's subject is the temp
  suffix's width, the one arm of the write
  seam left unmeasured at Round 6 and filed
  there for this round. UH-34 to UH-39
  settled the write's spelling, the temp's
  placement, and the two families'
  composition; the candidate's name was
  measured as the target's raw name and a
  dot. What the candidate carries after the
  dot is this round's whole question.
- UH-41. The CodeRabbit disposition owed
  from Round 6 discharges here on evidence:
  PR 7 carries zero inline findings on
  read-back (checked 2026-11-06: zero review
  comments and zero reviews; its summary
  shell finalized after the merge recording
  that the review failed because the pull
  request is closed, with a findings-free
  walkthrough, and a second bot comment
  recording the follow-up action not
  completed for the same reason), so there
  is nothing to fix and nothing to dismiss,
  and the absence is the disposition.
- UH-42. The suffix law, measured: where
  mkstemp appends exactly eight characters,
  so the Python's candidate is the name plus
  nine and it writes basenames through 246
  bytes and refuses at 247 at every pid, the
  port's generator concatenates the decimal
  digits of the process id, a per-process
  counter, and the attempt, so its suffix
  runs from three characters upward with
  the pid's width and its boundary moves
  with it. Measured in three postures
  against the Python: at pid 1 the suffix is
  three and the port writes through 251,
  refusing at 252; at pid 203 the suffix is
  five and it writes through 249, refusing
  at 250; at a six-digit ambient pid the
  suffix is eight by coincidence and the
  boundary stands at the Python's own 246
  and 247. The Round 6 red team's filed
  measurement completes the law's other
  direction: at a seven-digit pid the suffix
  is nine and the port refuses the 246-byte
  name the Python writes. The observable
  law is the suffix's length; its characters
  are not behavior on either side.
- UH-43. The repair: the suffix is exactly
  eight hexadecimal characters, drawn from
  the first four bytes of a SHA-256 hash of
  the process id, the counter, and the
  attempt, each as little-endian bytes. The
  shape is the family's settled one (ST-37,
  VL-42, FT-74: eight hex characters from
  four drawn bytes) fitted to this crate:
  sha2 is already its only dependency and
  its digest module's standing tool, and the
  three hashed terms are the crate's whole
  existing entropy, there being no
  wall-clock read anywhere in it. The
  counter increments on every draw and the
  attempt on every collision, so no two
  candidates in a process share a draw, and
  the create-new redraw loop, the 0600 mode,
  the temp's placement, the rename onto the
  constructed path, and the unlink on
  failure all stand as UH-34 to UH-36 left
  them.
- UH-44. The byte-counting arm, measured
  absent: the candidate's name is the
  target's raw name bytes end to end, from
  the spelled output's OS bytes through the
  byte split, the lexical absolutization,
  and the byte join, and the one lossy
  conversion in the tree is the non-unix
  fallback inside the byte reader, off the
  measured platform. Twin cells carrying
  one raw byte at the boundary lengths
  diverge before the repair only where
  UH-42's width puts them (at pid 1, the
  247- and 251-byte raw names the Python
  refuses), and the raw byte lands intact
  on both sides; after the repair every
  raw cell agrees.
- UH-45. The differential harness gains the
  boundary cells in both pid postures: the
  plain lengths around every measured flip
  (245 to 252), the popped-parent lengths,
  and the raw-byte lengths, each run at the
  ambient pid and at pid 1 in a fresh
  namespace. The control runs are part of
  the evidence: against the pre-repair
  binary the new cells fail exactly where
  UH-42's law puts them and no others, and
  two scratch mutants of the repaired
  generator, one widening the suffix to
  eighteen and one narrowing it to four,
  each fail the new cells in their own
  direction.
- UH-46. README records 7 of 100 only in
  the closure commit written after this
  round's three reviews, fixes, and owner
  verification.
