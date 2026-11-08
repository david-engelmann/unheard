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
- UH-47. This round's subject is the temp
  suffix's draw itself, the question Round
  7's reviews filed and this round
  answers. UH-42 to UH-45 settled the
  suffix's width, its boundary, and its
  byte counting; what the suffix is drawn
  from stood verbatim in the shipped
  source, the first four bytes of a
  SHA-256 hash of the process id, the draw
  counter, and the attempt, and the red
  team's filed cell (P3) showed the draw
  is a public function of the process id.
  Whether that exposure is real against
  the shipped binary, in which postures it
  bites, and what closing it changes are
  this round's whole question.
- UH-48. The CodeRabbit dispositions owed
  from Round 7 discharge here on evidence.
  PR 8 carries zero findings on full
  read-back (checked 2026-11-07: zero
  review comments and zero reviews; its
  summary shell, comment 6687754194,
  posted 2026-11-06T14:54:25Z, seven
  minutes before the 15:01:24Z merge, is a
  completed walkthrough stating that no
  actionable comments were generated, and
  the only later comment is the owner's
  landing note), so Round 7's merge-time
  record of zero findings stands confirmed
  whole. PR 9, the merge-queue shaping,
  likewise carries zero findings on
  read-back (zero review comments, zero
  reviews, one summary shell,
  6700625565, posted before its merge,
  stating the same). There is nothing to
  fix and nothing to dismiss, and the
  absence is the disposition.
- UH-49. The draw's exposure, measured.
  One output write in a fresh process
  draws exactly 65 candidates, the pairs
  (counter, attempt) = (i, i) for i in
  0..=64, the counter starting at 0 and
  both advancing once per collision until
  the attempt passes 64 and the write
  refuses. Every term is public, so the
  65 names of a run are computable from
  its process id alone, and planting them
  in the output's directory makes the
  shipped binary refuse the write with
  the cannot-write line while the Python
  writes. Measured at pid 1 in a fresh
  namespace, at pid 250 in a fresh
  namespace, and at an ambient pid by a
  launcher that plants for its own pid
  before it execs: the refusal lands in
  all three postures. Bounded planting is
  absorbed by the redraw (one, three, and
  64 of the 65 names planted at pid 1, and
  64 at pid 250, the write lands on the
  first unplanted draw). The Python cannot
  be planted against in this shape:
  mkstemp draws eight characters from a
  37-character alphabet, about 3.5
  trillion candidates, from a per-process
  random sequence the operating system's
  entropy seeds, and no feasible plant
  set stands in its way. Correctness is
  never at stake on either side: the
  create-new draw means a plant is never
  opened, so the exposure is refusal
  only, and it needs write access to the
  output directory plus the victim's pid,
  a position that already allows more
  direct refusal. It is convicted anyway,
  because the port made a public draw of
  what the Python keeps unguessable, and
  UH-43's repair is what published it.
- UH-50. The repair: the suffix is still
  exactly eight hexadecimal characters
  from the first four bytes of a SHA-256
  hash, and the hash now takes four
  terms, the process id, the wall-clock
  nanoseconds since the epoch, the draw
  counter, and the attempt, each as
  little-endian bytes. The clock read is
  the crate's first: UH-43 recorded that
  the three hashed terms were the crate's
  whole existing entropy, there being no
  wall-clock read anywhere in it, and
  that sentence is superseded by this
  item wherever the two disagree. The
  counter still increments on every draw
  and the attempt on every collision, so
  no two candidates in a process share a
  draw, and the create-new redraw loop,
  the 0600 mode, the temp's placement,
  the rename onto the constructed path,
  and the unlink on failure all stand as
  UH-34 to UH-36 left them. A clock that
  reads before the epoch fails the draw,
  and the write refuses with the same
  cannot-write line and exit 1 as any
  other write failure, leaving nothing
  behind; UH-51 measures that arm. The
  sibling shape is the family's settled
  one (tetlock's and bluepencil's draws
  carry the clock in the mix, and
  bluepencil's BP-47a contracts the same
  refusal), fitted to this crate: pid,
  nanoseconds, counter, attempt.
- UH-51. The clock arm, measured with an
  LD_PRELOAD shim on clock_gettime. With
  the clock reading one second before the
  epoch, the Python writes its document
  unchanged (mkstemp never reads the clock
  for its names), the pre-repair port
  writes too (it reads no clock anywhere),
  and the repaired port refuses: exit 1,
  the cannot-write line naming the output,
  nothing landed, and no temp left
  behind. With the clock call itself
  failing (the shim returns EINVAL), the
  Python and the pre-repair port again
  write, and the repaired port dies inside
  the standard library's clock read, its
  panic on stderr, exit 1, nothing landed,
  and the tree left exactly as seeded. The
  contracted guarantee is the pair: no
  write under either broken-clock shape;
  the clean refusal is contracted for the
  pre-epoch shape, and the erroring
  shape's panic is recorded as measured,
  not smoothed over.
- UH-52. The byte-counting arm, for the
  draw: the plant set is name-independent.
  No name or path byte enters the hash
  (the draw's inputs are the process id,
  the clock, the counter, and the
  attempt, all integers), and the
  candidate's name is the target's raw
  name bytes end to end, as UH-44
  measured for the width question. The
  twin cell carrying one raw byte in the
  output's name plants the same 65
  suffixes as the plain cell, and it
  convicts and clears with the plain
  cell. There is no lossy seam in the
  suffix path to measure: the tree's one
  lossy conversion stands where UH-44
  left it, the non-unix fallback inside
  the byte reader, off the measured
  platform and off this path.
- UH-53. The differential harness gains
  two cases, 13 cases and 130 steps in
  all. The planted-names case runs the
  plant sets of UH-49 as twin cells: at
  pid 1 and at pid 250 in fresh namespaces
  and at an ambient pid by a self-planting
  launcher, the bounded sets standing as
  genuine-acceptance cells (the write
  lands through the collisions) and the
  full 65 standing as the convicted cell.
  The clock case runs both sides under
  the shim of UH-51 and asserts the two
  contracted splits, the Python writing
  and the port refusing under the
  pre-epoch clock with nothing left
  behind, and the port dying in the clock
  read under the erroring clock with the
  tree left as seeded. The control runs
  are part of the evidence: against the
  pre-repair binary the harness fails on
  exactly seven steps, the five full-plant
  cells and the clock case's two splits,
  and no others, and against the repaired
  binary it runs clean, 130 steps and no
  failures.
- UH-54. Item 3 stands open, stated
  exactly. The one-line description is
  identical in two of its three homes:
  the repository description and the
  README's lede are byte-identical, set
  so by this round, and Cargo.toml
  carries no description field at all.
  The line itself also promises the
  proposing of rewording, relocation, or
  deletion, which the boundary sentence
  assigns to bluepencil: unheard measures
  the hearing and never edits. The lede
  stands verbatim because the standard
  pins it to the repository description,
  and the opening paragraph states the
  boundary, so the first screen read
  whole does not hide the gap. Writing
  the field, and settling the line, is a
  separate pass on David's word, and
  this item is its record.
- UH-55. README records 8 of 100 only in
  the closure commit written after this
  round's three reviews, fixes, and
  owner verification.
- UH-56. CodeRabbit's review of this
  round's pull request (PR 10, review
  5816559322, four actionable comments
  carried in three comment bodies) is
  disposed of here. Three findings are
  repaired in the README, each verified
  against the source and demonstrated
  against the shipped binary before
  its sentence was written. The rules
  file's structure is now stated where
  the rules file is introduced: a list
  item before any heading fails with
  "error, rule outside any section at
  line 1" (demonstrated), a marker
  with no text after it fails with
  "error, empty rule at line 3"
  (demonstrated), and a file of prose
  alone scores no rules at all, its
  document empty in every list
  (demonstrated); numbered and starred
  markers inventory like dashed ones
  (demonstrated), and a prose line
  underlined as a setext heading
  serves as the heading (demonstrated).
  The path prohibition's reach is now
  stated where the shapes are named:
  against a synthetic tape whose Read
  call and Bash call carry
  secrets/prod.env in their input
  strings and whose Edit call names
  another path, the rule "Do not edit
  secrets/prod.env" recorded two
  ignored observations and two
  violations, one per carrying call,
  so a call that only reads the path
  counts as a violation (demonstrated,
  and read in src/scoring.rs, where
  the shape tests every string in a
  call's input and never the tool).
  The six shapes' accepted wordings
  are now listed in the README in
  place of the concept names alone:
  all 28 literal forms the matcher
  accepts (read in src/shapes.rs)
  were scored in one battery and
  every one measured with its
  expected shape, and "Do not ever
  use Grep" scored unmeasured in the
  same battery, the near wording the
  review named (demonstrated). One
  finding is dismissed with its
  reason: the Install block's PATH
  note (comment 4504978392) asks for
  prose about persisting the path,
  and the block is the fleet's
  uniform self-contained verification
  install, a scratch CARGO_HOME
  inside the clone with its bin on
  PATH for the session, verified
  verbatim in that form by this
  round's reviews, with the
  Quickstart running in the same
  shell by design; the same shape
  was dismissed on the same basis at
  flighttape (FT-87). This item lands
  after the closure commit by the
  round's server-side sequence, and
  the rounds line the closure set
  (8 of 100) is untouched and still
  exact.

- UH-57. The owed read-back of PR 10's
  CodeRabbit record, completed before
  this round's work began, finds one
  item standing undisposed, and this
  item disposes it. Review 5816559322's
  four actionable items were disposed
  in UH-56, three repaired and one
  dismissed, with the bot's
  acknowledgments standing on the
  threads. The second shell, issue
  comment 6717096849, states only that
  the action was not completed because
  the last commit was already reviewed;
  it carries no findings. The undisposed
  item is the failed Docstring Coverage
  pre-merge check, standing in review
  5816559322's body and in the summary
  shell 6717023581, reporting 60.00
  percent against an 80.00 percent
  threshold over 5 functions analyzed.
  It is dismissed on the ST-61 basis
  (the touchstone VL-51 basis): the
  check names no function, the
  repository carries no docstring
  configuration or threshold of its
  own for it to enforce (its contract
  carries no docstring rule and its
  tree no docstring tool setting), and
  the check applies the review tool's
  default profile, so there is no repo
  rule for it to enforce. There is
  nothing to fix and nothing to
  configure; the disposition is the
  record.
- UH-58. The temp-name draw's entropy
  adjudication, measured, and the
  repair. UH-50 mixed the wall-clock
  nanoseconds into the draw to end the
  pid-only exposure UH-49 convicted,
  and the question this round settles
  is the frozen clock: under a clock
  frozen at a known reading the
  nanoseconds term is a known constant,
  so the 65 candidates of a fresh
  process's single output write, the
  pairs (counter, attempt) = (i, i)
  for i in 0..=64, are computable again
  by an observer who also fixes the
  pid. Measured against the Round 8
  binary as control, with the clock
  held by interposition and the pid
  fixed by a fresh namespace: at
  1,700,000,000 seconds and 123,456,789
  nanoseconds, pid 1, planting the 65
  computed names refuses the control
  with the cannot-write line and no
  output left behind, and planting 64
  writes on the 65th draw; at
  1,800,000,000 seconds and 987,654,321
  nanoseconds, pid 250, the 65 plants
  refuse it again. The shipped draw
  was plantable under a frozen known
  clock. The repair takes the fleet's
  WW-69 shape: 16 bytes read from
  /dev/urandom for each candidate,
  appended to the hash input after the
  attempt term, the suffix still the
  first four digest bytes as eight
  lowercase hexadecimal characters,
  and a failed or short entropy read
  fails the draw so the write refuses
  through the standing cannot-write
  arm of UH-11. There is no fallback
  source, and off unix, where no
  source is contracted, the draw fails
  the same way. Measured after the
  repair, in the lane battery: the
  same 65 plants at the first frozen
  reading and pid 1 are inert and the
  write lands byte-identical to its
  unplanted document, the pre-epoch
  posture of UH-51 still refuses
  cleanly, and with every open of
  /dev/urandom failing with EACCES
  the write refuses with the directory
  byte-identical before and after.
  The oracle writes under the entropy
  failure, as it writes under the
  frozen clock: CPython's os.urandom
  is served by the getrandom syscall,
  which file interposition does not
  reach, and its tempfile names are
  seeded from that entropy, so no
  observer-held input determines any
  candidate on the oracle's side; the
  repair brings the port to the same
  standing, by the file source WW-69
  contracts. The unit pins change
  with the mix: the entropy term
  deciding the suffix at fixed pid,
  clock, counter, and attempt, and
  the suffix matching an independent
  derivation of the contracted mix,
  are deterministic pins. The shape
  pin and the not-the-public-draw pin
  read the draw under the refusal
  disjunction, a draw that refuses
  asserting nothing. The
  varies-between-draws pin is retired:
  it required two live draws to
  differ, which the WW-73 discipline
  forbids requiring of a 32-bit draw.
- UH-59. The observations document
  carries a version key, and its full
  shape is stated here, in unheard's
  own contract, so the document can
  be implemented and read from
  unheard alone. This is a format
  change and it changes the emitted
  bytes: where the document opened
  with the observations key, it now
  opens with observations_version,
  and every later byte is unchanged,
  as the differential harness of UH-60
  measures. The document is a JSON
  object rendered as json.dumps
  renders it, with an indent of two,
  keys in the order stated here, and
  non-ASCII characters unescaped, with
  a trailing newline. Its keys, in
  order: observations_version, the
  string "1.0". observations, a list
  of objects, one per exercised event,
  each carrying rule (the rule's exact
  text), kind (followed, ignored, or
  misreading), and quote (the word
  tape, the tape's canonical form 1
  digest, the word seq, and the
  event's seq, with a misreading's
  quote adding the word used and the
  tool actually used). scores, a list
  in rule order of objects carrying
  rule, shape (prohibit-tool,
  prohibit-path, ordering,
  required-tool, cap, first, or
  unmeasured for a rule no shape
  matches), tapes_exercised,
  tapes_total, events_exercised, and
  violations, all counts. unmeasured,
  the texts of the unmeasured rules.
  unmeasured_count, their count.
  unexercised, the texts of the
  measured rules no tape exercised.
  tapes, an object carrying read,
  scored, and excluded, the excluded
  a list of objects carrying file,
  reason, and defects. duplicates,
  the texts of rules stated more than
  once, and warnings, the parse
  warnings, stand after tapes, in
  that order, each present only when
  non-empty. The version's law:
  version "1.0" names exactly this
  shape; a change to the shape that a
  reader of "1.0" cannot absorb is a
  new version, never a silent edit of
  this one. UH-8 and UH-12a defer to
  bluepencil's BP-6 for the shape
  bluepencil consumes, and BP-6's text
  names the observations list and its
  entries; the version key and the
  rest of the document's keys stood
  specified nowhere until this item.
  Bluepencil reads the document
  through its observations key, so the
  added first key does not disturb
  the consuming side as it stands;
  the consuming half, reading and
  checking the version, lands in
  bluepencil's own round, not this
  one. The README's Status states the
  emitted version, and its first
  screen capture is regenerated to
  the new shape and verified
  byte-for-byte against a real run of
  the Quickstart. The version is
  pinned in tests/test_scoring.rs:
  the emitted document's first bytes
  are the version key and its parsed
  value is the string "1.0".
- UH-60. The differential harness
  gains two cases and a comparison
  rule, 15 cases and 134 steps in
  all, running clean against the
  repaired binary. The comparison
  rule carries UH-59's contracted
  split: compare strips the version
  line, and only the version line,
  from any document a side emits, so
  every twin comparison still asserts
  the version line is the only
  difference between the port's
  document and the oracle's. The
  frozen clock plant case runs both
  sides at pid 1 and at pid 250 under
  the fixed readings of UH-58 with
  the 65 computed names of the
  clock-only mix planted in both
  trees, and the twins agree, the
  plant inert on the repaired draw.
  The entropy source failure case
  runs both sides under the EACCES
  interposition of UH-58 and asserts
  the contracted split exactly: the
  oracle writes its baseline
  document, and the port refuses
  with the cannot-write line and the
  tree left exactly as seeded.
- UH-61. README records 9 of 100 only
  in the closure commit written after
  this round's three reviews, fixes,
  and owner verification.
- UH-62. Round 9's CodeRabbit review
  5819467864 carried two items, and
  both are discharged here on the
  UH-56 pattern. The Trivial finding
  named the new version test's temp
  directory left behind. The build
  lane dismissed it on the pull
  request as the suite's uniform
  pattern, and the standards review
  overturned that dismissal on the
  count of 37 accumulated roots and
  the fleet precedent that a round
  adding roots repairs them. The
  item was repaired in the repair
  commit 9c39dff9, where the helper
  returns a guard whose drop removes
  the directory at all 13 call
  sites, proven on both arms, the
  passing suite and a panicking
  test each leaving zero roots. The
  correction stands on the pull
  request as comment 6727640185.
  The review's second item, the
  Docstring Coverage recurrence at
  57.14 percent against 80.00
  percent in shell 6726437711, is
  dismissed on the basis UH-57
  states. UH-58 claims no collision
  freedom for its 32-bit suffix.
  The suffix is a plant resistance
  measure only, as WW-69 states for
  its own draw.
