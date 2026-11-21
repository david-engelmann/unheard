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
- UH-63. Round 9's second CodeRabbit
  review 5819827425, posted on the
  repair head 9c39dff9 after the
  first review's record had closed,
  carried one inline comment,
  4507950835, a Minor at README.md
  line 168: keep the closed-round
  count at 8 until the closure
  conditions are met. It is
  dismissed, and its ground is
  stated exactly. The line it names
  is the closure commit's line, and
  a closure commit takes effect on
  main only at the merge. Main at
  the round's base read 8 of 100,
  and it read 8 of 100 at every
  moment before the merge; the
  merge 3504b1ee landed only after
  the three reviews, the repair
  commit, the three delta reviews
  on the repair, and the owner's
  verification, as the landing
  comment 6727877771 records. No
  reader of main was ever shown 9
  of 100 before the conditions the
  line counts were met, and the
  line on main today is true.
  There is nothing to repair. The
  review's summary shell 6726437711
  repeats the item as its merge
  risk note and carries no second
  finding in it, and the remaining
  shell, issue comment 6726484655,
  states the action was not
  completed because the review was
  rate limited, and carries no
  findings.
- UH-64. The same review's body
  carries the failed Docstring
  Coverage pre-merge check a second
  time, in its prompt section, with
  no figure and no function named;
  the figure stands in the summary
  shell 6726437711 at 57.14 percent
  against 80.00 percent over 7
  functions, the item UH-62 already
  dismissed citing that shell. It
  is the same check on the same
  diff, and it is dismissed on the
  basis UH-57 states: it names no
  function, the repo carries no
  docstring configuration or
  threshold of its own, and the
  check applies the review tool's
  default profile, so there is no
  repo rule for it to enforce.
- UH-65. The observations document's
  emitting half, measured fresh
  against the Python oracle (the
  archive of 03658d9 at uh-oracle,
  its fixture blobs verified
  against main's tree before any
  cell was trusted; the port's
  emitting sources are
  blob-identical to the Round 9
  head's). At the fixture size the
  port's document is 2,463 bytes
  and the oracle's 2,430, and the
  port's document is the oracle's
  with exactly the version line
  inserted after the opening brace,
  in file and stdout modes alike.
  At realistic size (235 rules, 24
  tapes, 5,276 events) the document
  is 8,181,065 bytes, its first
  keys are observations_version and
  observations in that order, the
  parsed version is the string
  "1.0", and stripping the version
  line leaves the oracle's document
  byte for byte. No other byte
  differs from UH-59's law.
- UH-66. The draw's entropy posture,
  measured fresh. The 65-plant
  clock-only shape of UH-58 stays
  inert at both frozen readings of
  that item, 1,700,000,000 seconds
  and 123,456,789 nanoseconds at
  pid 1 and 1,800,000,000 seconds
  and 987,654,321 nanoseconds at
  pid 250: the write lands and the
  document is byte-identical to its
  unplanted baseline. Entropy
  failure refuses both ways it can
  arrive. With every open of
  /dev/urandom failing with EACCES
  the write refuses with the
  cannot-write line and the
  directory byte-identical as
  seeded, and with those opens
  redirected to a file holding five
  bytes, so the read succeeds but
  is short, the write refuses the
  same way with the directory as
  seeded; the short-read cell is
  new this round. The pre-epoch
  posture of UH-51 stands: the port
  refuses cleanly and the oracle
  writes, the contracted split.
- UH-67. The hearing surfaces least
  recently measured, measured
  fresh. The pick: shape
  classification and its order
  (UH-6a, UH-13b, UH-15), per-event
  exercise and violation counting
  with quote formation (UH-7a,
  UH-8a, UH-13a, UH-14a), and the
  inventory sections (UH-8b,
  UH-12a, UH-20). The ground: every
  dedicated measurement item from
  Round 4 onward measures the write
  seam (UH-25a through UH-53,
  UH-58) or the version key (UH-59,
  UH-60); these surfaces were last
  dedicated-measured in the Round 3
  port differential and since then
  only in the harness's fixed small
  cells. The fresh corpus of UH-65
  (235 rules, 27 tapes read and 24
  scored with 3 excluded, 5,276
  events) yields 234 score entries
  across all seven shapes (61
  prohibit-tool, 40 prohibit-path,
  30 ordering, 30 required-tool, 32
  cap, 20 first, 21 unmeasured)
  and 42,941
  observations (12,661 followed,
  6,008 ignored, 24,272
  misreading), with 21 unmeasured,
  100 unexercised, and 1 duplicate,
  and warnings stand empty on both
  sides, the key absent from both
  documents; the twins agree byte
  for byte once the version line is
  stripped. A grid of 20
  classification edge cells (the
  cap at 0, 1, and 18 digits, 18
  digits with leading zeros, the
  18-digit maximum, the 19-digit
  overflow, digits in another
  script, both at-most forms, the
  trailing-period arms, the
  first-step forms, and a lowercase
  non-match among them) agrees in
  all 20 cells. No conviction;
  nothing is repaired.
- UH-68. The differential harness
  stands at 15 cases and 134 steps,
  running clean against this
  round's build, and is unchanged
  this round: no case convicted, so
  no case was added.
- UH-69. README records 10 of 100
  only in the closure commit
  written after this round's three
  reviews, fixes, and owner
  verification.

## Round 11. The launch witness, the walk boundary, and the entropy read (UH-70 to UH-77)

- UH-70. The owed read-back of PR 12's
  CodeRabbit record, completed before this
  round's work began, on the UH-56 pattern.
  The record stands: zero reviews and zero
  review comments on the review endpoints,
  and four issue comments. The trigger
  (6759320681) carries nothing. The summary
  shell 6759322226, edited in place to its
  final form at 2026-11-10T05:39:37Z, stands
  in two parts. Its recent-review part
  (Run ID fa9a25c4-1532-4c54-85e4-394da91f7e61)
  reviews the pull request from the base
  894a7bd4 to the closure head 3f12ac78 over
  two files, CONTRACT.md and README.md, and
  its pre-merge checks stand at 4 passed and
  none failed: Description Check, Title
  check, Linked Issues check, and Out of
  Scope Changes check, each Passed. No
  Docstring Coverage failure stands in this
  shell, a change from Rounds 9 and 10.
  Its Merge Risk stands at Minimal, previous
  review 3f12a, its coverage naming the
  closure head 3f12ac78 as the covered
  commit, and its text states that the
  documentation and status update are ready
  to merge and that the count wording
  discrepancy does not affect behavior. Its
  later part (Run ID
  58d5dab4-93bd-4641-8bb3-4b9d88f754c3) is a
  rate limit notice, Review limit reached,
  covering the range from 3f12ac78 to the
  repair head 3a4ed402 over the one file
  CONTRACT.md, and it records that no
  actionable comments were generated in the
  recent review. The remaining bot shell,
  6759421759, states only that the action
  was not completed because the last commit
  was already reviewed; it carries no
  findings. The disposition: no actionable
  comment, no inline comment, and no failed
  check stands anywhere in PR 12's
  CodeRabbit record, so there is nothing to
  fix and nothing to dismiss, and the
  absence is the disposition. The Merge
  Risk note's count wording discrepancy is
  not a CodeRabbit finding: it carries no
  comment and no check, and the discrepancy
  it names is the UH-67 count sentence the
  owner's standards review found, repaired
  in 3a4ed402 before the merge and passed
  by both delta reviews, as the owner's
  landing comment 6760070827 records. The
  repair head 3a4ed402 was never reviewed
  by CodeRabbit; its review is the owner's,
  and this item rests on the whole record
  read to its ends, not on any merge-time
  read.
- UH-71. The fresh gate and the regression
  of UH-65 to UH-67, measured at the base
  (the Round 10 merge 3b0245e1, its tree
  verified blob for blob against the API
  tree before any cell was trusted). The
  oracle pairing stands as UH-65 verifies
  it: the archive of the Python final
  commit 03658d9, its five fixture blobs
  hashing to exactly the base tree's
  blobs, and its own suite passing 28.
  The gate at the base and again on this
  round's final tree: cargo test 33
  passed and 0 failed (4 unit, 24 scoring,
  5 shapes), clippy with warnings denied
  clean, fmt clean, and the codepoint scan
  at zero for both dashes. The Round 10
  battery rerun against this round's build
  passes every cell with its Round 10
  numbers unchanged: the fixture document
  at 2,463 bytes against the oracle's
  2,430 with exactly the version line
  inserted, the realistic corpus (235
  rules, 27 tapes read and 24 scored with
  3 excluded, 5,276 events) yielding the
  8,181,065-byte document and 42,941
  observations, the 65 clock-only plants
  inert at both frozen readings and pids,
  the entropy open refusal and short read
  refusals, the pre-epoch split, and the
  classification edge grid agreeing in
  all 20 cells.
- UH-72. The launch witness, measured
  fresh: the port's closed_at_launch read
  of /proc/self/fd and fdinfo (src/util.rs),
  a surface no earlier item contracts.
  Twin cells launch both sides with a
  standard stream closed at launch. The
  twins agree exactly, exit and surviving
  stream bytes alike, in every cell but
  one. Help with stdout closed lands on
  stderr at exit 0, and with stderr closed
  lands on stdout at exit 0. A parse error
  with stderr closed puts the usage line
  alone on stdout at exit 2, its error
  line lost on both sides; with both
  streams open the control puts usage and
  error on stderr at exit 2. An error line
  with stderr closed migrates to stdout at
  exit 1 in all three of its shapes: the
  missing rules file (error, no such rules
  file), the guard's refusal (error,
  unheard never writes to its inputs), and
  the cannot-write line naming its output.
  A successful score with stderr closed
  puts its document on stdout at exit 0,
  the port's document the oracle's with
  exactly the version line inserted. A
  stream aimed at /dev/null in the
  ordinary write-only way reads as present
  on both sides: help exits 0 and nothing
  reaches stderr. The one split: a score
  to stdout with stdout itself closed at
  launch. The port exits 1 silently,
  nothing on the surviving stream and no
  document anywhere; the oracle exits 1
  with a traceback on stderr, its
  sys.stdout standing as None at the write
  (AttributeError at the oracle cli.py's
  stdout write). The exits agree and no
  byte is emitted on either side; only the
  clothing differs, the crash clothing
  UH-17 already contracts. No conviction;
  nothing is repaired.
- UH-73. The resolver walk's budget,
  measured fresh at its boundary (UH-25a).
  A chain of nested symlink splices names
  the output. At 989 and at 990 splices
  both sides pass the guard and refuse at
  the write, byte-identical on both sides:
  exit 1 and the cannot-write line naming
  the spelled output, nothing landed; the
  cannot-write line is the evidence the
  write stage was reached, since the crash
  class wears no line. At 991 and at 992
  the guard's resolution itself fails, in
  the crash class of UH-17: the port exits
  1 silently and the oracle exits 1 under
  a RecursionError traceback, nothing
  landed on either side. The boundary
  stands exactly where UH-25a puts it:
  990 nested splices resolve, the 991st
  fails. A sequential spelling, two chains
  of 600 splices in one output path (1,200
  splices in total, no nesting deeper than
  600), passes the guard on both sides and
  refuses with the same byte-identical
  cannot-write line, so the budget counts
  nesting depth and not splices in total,
  as UH-25a states.
- UH-74. The entropy read's semantics,
  measured fresh (UH-58, UH-66), under a
  counting and shaping interposition on
  /dev/urandom. One file-mode write with
  no collision opens /dev/urandom exactly
  once and reads exactly 16 bytes in one
  read; a stdout-mode score opens it zero
  times, no candidate being drawn when no
  temp is created. A chunked source, its
  reads capped at five bytes per call
  against the real source, still lands the
  write, its document byte-identical to
  its baseline: the port's reads run 5, 5,
  5, and 1, summing to the candidate's 16
  bytes, so a short read that is not the
  end of the source is absorbed by the
  read's own looping, and only the
  end-of-source short reads of UH-66
  refuse. The refusal arm takes one more
  shape: opens redirected to a directory
  succeed at the open and fail at the read
  (the counting log records one open and
  one read returning -1), and the write
  refuses with the cannot-write line and
  the directory byte-identical as seeded.
  An empty source, zero bytes, refuses on
  the same arm as UH-66's five-byte file.
  The redraw path's open count stands
  unmeasured: a collision cannot be forced
  against an entropy draw from outside the
  process, so no cell counts a second
  candidate's opens, and this item claims
  none.
- UH-75. The document's remaining joints
  of UH-65 and UH-67, measured fresh. At
  realistic size in stdout mode (the
  corpus of UH-71) the port's stdout
  document is byte-identical to its own
  file document at 8,181,065 bytes, and
  stripping its version line leaves the
  oracle's stdout document byte for byte;
  UH-65 measured the stdout agreement at
  the fixture size only, and it stands at
  realistic size too. A warnings-present
  corpus, a rules file carrying a
  duplicate rule and an unclosed fence,
  agrees between the twins byte for byte
  once the version line is stripped. Its
  document's keys run in UH-59's order
  with the two conditional keys live and
  trailing in their stated order,
  duplicates then warnings: duplicates
  carries the one repeated text, and
  warnings carries the one warning, the
  unclosed fence at its line with the rest
  of the file read as code. UH-67's fresh
  corpus carried warnings empty on both
  sides with the key absent; the key's
  present shape is measured here.
- UH-76. The differential harness stands
  at 15 cases and 134 steps, running clean
  against this round's build, and is
  unchanged this round: no case convicted,
  so no case was added.
- UH-77. README records 11 of 100 only in
  the closure commit written after this
  round's three reviews, fixes, and owner
  verification.

## Round 12. The read-back, the tape directory's enumeration, and the degenerate inputs (UH-78 to UH-82)

- UH-78. The owed read-back of PR 13's
  CodeRabbit record, read back whole
  through the API. PR 13 stands merged
  at cf4858f6, its parents 3b0245e1 and
  the closure head 1bf2c5ae, the merge
  tree 2c34dc67 identical to the
  reviewed head tree, and the README
  on main recording 11 of 100. The
  body stands in its final patched
  form at 17,819 bytes, the bot's
  release-notes appendix at 689 bytes
  preserved byte for byte through
  both completion lane patches, its
  corrected CI attribution naming
  both hosted runs on the closure
  head and none on the slice head.
  The census at this read-back: four
  issue comments, zero reviews, zero
  review comments. Three comments
  stood at landing (the trigger, the
  summary shell, and the follow-up);
  the fourth is the owner's landing
  comment 6945325111 itself, posted
  after the merge, and no CodeRabbit
  comment was posted after the
  landing, so nothing post-landing
  stands to disposition. The shell
  6804891086 in the form standing at
  this read-back is 10,646 characters,
  last edited 2026-11-17T05:33:00Z,
  its coverage naming the closure
  head 1bf2c5ae as both source and
  covered commit, pre-merge checks
  at 4 passed and none failed (Title,
  Linked Issues, Out of Scope,
  Description), Merge Risk at
  Minimal, Run ID
  6e42fa9a-c988-45ee-9012-a7a01f857686,
  no Docstring Coverage row, and the
  sentence that no actionable
  comments were generated in the
  recent review. The completion lane
  first recorded the shell at 10,642
  characters last edited
  2026-11-11T19:15:13Z; it was edited
  once more, four characters longer,
  with every disposition-bearing
  element unchanged. The follow-up
  6805054660 states only that the
  action was not completed because
  the last commit was already
  reviewed. There is nothing to fix
  and nothing to dismiss, and the
  absence is the disposition. The
  landing comment closes the
  battery11 X989 and X990 question
  on the reviews' evidence: the two
  cells were the completion lane's
  own mis-specification, UH-73
  stands as written, and no repair
  round is owed. Two observations
  the landing comment charges are
  dispositioned here. First, UH-73's
  text does not spell its output
  construction; this round's
  measurement family does not touch
  the resolver walk, so the
  observation is carried by name:
  the head-spelling posture (a head
  spelling replaces the chain's head
  link itself and lands at exit 0 on
  both sides; a through-chain
  spelling refuses at the write; the
  guard resolves through 990 splices
  and fails at the 991st under every
  construction) stands on the Round
  11 reviews' reproduction,
  uncontracted by any item, and is
  owed to the next round whose
  family touches the walk. Second,
  UH-77's closure form: the closure
  commit precedes the reviews, as in
  Round 10 and Round 11, and the
  substance is satisfied because
  the owner merges only after the
  reviews; this round's closure
  item UH-82 follows the honest
  form and says so in its own text.
- UH-79. The tape directory's
  enumeration and the degenerate
  inputs, measured fresh against
  the Python oracle, a surface no
  earlier item contracts: UH-5
  names the inputs, UH-10 the
  filename order, and UH-5a the
  subset checks a file must pass
  once read, but the candidate set
  of a directory and the degenerate
  inputs stood unmeasured. Every
  regular file in the directory is
  a candidate, whatever its name:
  a plain-text file is read and
  excluded as not valid JSON, and
  valid tapes under a name with no
  extension, a text extension, a
  dotfile name, an uppercase JSON
  extension, and a name holding a
  space are all read and scored;
  the extension is never consulted.
  Subdirectories are skipped
  silently, including one named
  like a tape. Symlinks are
  followed both out of the
  directory and into it, with no
  path identity anywhere in the
  document: three read, three
  scored, nine observations; a
  broken symlink is skipped
  silently. The filename order is
  codepoint order (0bad.json,
  10.json, 2.json, B.json, a.json,
  z.json, é.json), identical for
  forward and reverse creation
  orders, the malformed 0bad.json
  excluded for its own defect, six
  observations in all. The
  degenerate inputs score cleanly:
  an empty tape directory reads
  zero, scores six, and leaves
  five unexercised; a zero-event
  tape is excluded as not a valid
  tape with the defect that events
  must hold at least one event; an
  empty rules file and a
  headings-only rules file both
  produce the minimal 176-byte
  document with zero scores and
  zero observations. The same
  content under two names is
  scored twice, once per name,
  with no deduplication, and file
  mode with an output path outside
  the tapes directory agrees byte
  for byte. The twins agreed in
  every cell: exit, stderr, the
  input tree untouched, and the
  document byte for byte once the
  port's version line is stripped.
  No conviction; nothing is
  repaired.
- UH-80. The fresh gate and the
  regression of UH-65 to UH-67 and
  UH-72 to UH-75, measured at the
  base (the Round 11 merge
  cf4858f6, its tree verified blob
  for blob against the API tree
  before any cell was trusted). The
  oracle pairing stands as UH-65
  verifies it: the archive of the
  Python final commit 03658d9, its
  five fixture blobs hashing to
  exactly the base tree's blobs,
  and its own suite passing 28.
  The gate at the base and again on
  this round's final tree: cargo
  test 33 passed and 0 failed (4
  unit, 24 scoring, 5 shapes),
  clippy with warnings denied
  clean, fmt clean, and the
  codepoint scan at zero for both
  dashes. The regression batteries
  stand as they stood. battery10
  passes whole: the fixture and
  realistic version shapes (the
  realistic corpus at 235 rules,
  24 tapes, 5,276 events, and
  42,941 observations over an
  8,181,065-byte document), the
  launch witness cells, and the
  classification edge grid at 20
  of 20. battery11 passes 18 cells
  across six byte-identical runs,
  and its two remaining cells are
  read as the settlement of
  UH-78's read-back, not as a new
  conviction: X989 and X990 still
  carry the Round 11 instrument's
  refusal expectation, and both
  arms instead land the write by
  head-link replacement at exit 0,
  the tree changed at the head
  link alone, while X991 and X992
  stand in the crash class and the
  sequential-splice cell stands as
  contracted.
- UH-81. The differential harness
  stands at 15 cases and 134 steps,
  running clean against this
  round's build (0 failures, the
  build's sha256 verified identical
  at the harness's standing path),
  and is unchanged this round: no
  case convicted, so no case was
  added.
- UH-82. README records 12 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in
  the standing form of UH-69 and
  UH-77, and the record takes
  effect only when the owner
  merges after the reviews, fixes,
  and owner verification.
## Round 13. The read-back, the standing regression, and the rules file's byte surface (UH-83 to UH-87)

- UH-83. The owed read-back of PR 14's
  record, read back whole through the
  API. PR 14 stands merged at 1942cb39,
  its single parent cf4858f6, the merge
  tree 082c2b7f identical to the
  reviewed closure head tree, and the
  README on main recording 12 of 100.
  The body stands in its final form at
  12,199 characters, the bot's
  release-notes appendix at 3,497 bytes
  preserved, the owner completion
  record appended to it with the three
  reviews APPROVE on every axis. The
  census at this read-back: six issue
  comments, zero reviews, zero review
  comments. Five comments stood at
  landing (the Trunk merged note, the
  summary shell, the trigger, the
  follow-up, and the lane's merge
  request); the sixth is the owner's
  landing comment 7033219359 itself,
  posted after the merge, and no
  CodeRabbit comment was posted after
  the landing, so nothing post-landing
  stands to disposition. The shell
  7030237352 in its true final form at
  this read-back is 11,067 characters,
  last edited 2026-11-20T04:20:22Z, its
  coverage naming the closure head
  da854cbf as both source and covered
  commit, pre-merge checks passed,
  Merge Risk at Minimal, Run ID
  d90fa258-06e3-43b4-b5e5-76ae55609a84,
  and the sentence that no actionable
  comments were generated in the recent
  review. The build lane first recorded
  the shell at 11,065 characters; it
  was edited once more, two characters
  longer, with every
  disposition-bearing element
  unchanged, and the owner record
  states it at 11,067. Its one style
  observation, the title long at 126
  characters with the verdict
  unchanged, stands recorded in the
  body. The follow-up 7030297124 states
  only that the action was not
  completed because the last commit was
  already reviewed. There is nothing to
  fix and nothing to dismiss, and the
  absence is the disposition. This item
  also contracts UH-79's family
  enumeration by name, in the standing
  erratum shape the landing comment
  charges. UH-79's contracted family is
  sixteen cells, N1 to N16. The banked
  instrument's CELLS table holds
  fifteen entries, and two of them
  split in the running: N8 runs as a
  forward build, a reversed build, and
  a creation-order-free comparison, and
  N11 runs as an empty rules file and a
  headings-only rules file, so the
  battery's log measures eighteen PASS
  lines across fifteen distinct cells,
  and FAILURES: 0 is the battery's own
  summary. Two cells the design prose
  never names stand contracted in the
  instrument before any run: N15, a
  valid tape under a name holding a
  space, read and scored like any
  regular file; and N16, file mode with
  an output path outside the tapes
  directory, agreeing byte for byte
  with stdout mode. The sixteenth
  contracted cell, N14, a directory
  named exactly like a tape file, was
  folded into N5's subdirectory cell by
  contraction before any run: it has no
  run of its own, and its posture is
  the one UH-79's prose states, that
  subdirectories are skipped silently,
  including one named like a tape. No
  cell was added after its results were
  known, and no measurement is wrong;
  the enumeration stands contracted
  here by name, and no tree change was
  owed.
- UH-84. The standing regression, in
  the contracted forms, measured at the
  base (the Round 12 merge 1942cb39,
  its tree verified blob for blob
  against the API tree before any cell
  was trusted). The oracle pairing
  stands as UH-65 verifies it: the
  archive of the Python final commit
  03658d9, tree ffa48f27, verified blob
  for blob against the driven copy at
  this round's hand, and its own suite
  passing 28. The gate at the base and
  again on this round's final tree:
  cargo test 33 passed and 0 failed (4
  unit, 24 scoring, 5 shapes), clippy
  with warnings denied clean, fmt
  clean, from untruncated captures at
  both trees, and the two builds
  byte-identical. battery10 passes
  whole: ALL PASS, the realistic corpus
  at 235 rules, 24 tapes, 5,276
  events, and 42,941 observations over
  an 8,181,065-byte document.
  battery11 stands at its settled
  posture: 18 PASS, and exactly the two
  settled cells X989 and X990 failing
  against the Round 11 instrument's
  refusal expectation while both arms
  land the write by head-link
  replacement at exit 0, read as the
  settled mis-specification from PR
  13's landing, not a conviction;
  X991 and X992 stand in the crash
  class and the sequential-splice cell
  stands as contracted. battery12
  stands whole at UH-79's dispositions:
  eighteen PASS lines, FAILURES: 0.
  The differential harness stands at 15
  cases and 134 steps, running clean
  against this round's build (0
  failures, the build's sha256 verified
  identical at the harness's standing
  path), the harness file unmodified,
  and is unchanged this round: no case
  convicted, so no case was added. The
  Quickstart capture reproduces the
  README's first-screen document byte
  for byte at 1,687 bytes, exit 0, from
  the synthetic rules against the
  synthetic tape.
- UH-85. The rules file's byte surface,
  measured fresh against the Python
  oracle, a surface no earlier item
  contracts: the contract holds no
  count for a byte order mark, a
  carriage return, or a line ending,
  and Round 12's rules cells varied
  only the inventory's content at plain
  line-feed bytes. Both readers decode
  the rules file as utf-8-sig and split
  it under the interpreter's splitlines
  discipline, and the twins agreed in
  every cell: exit, stderr, the input
  tree untouched, and the document byte
  for byte once the port's version line
  is stripped. A leading byte order
  mark, carriage-return line endings,
  carriage-return-plus-line-feed
  endings, a missing final newline, and
  the mark and the paired endings
  together each reproduce the control's
  document byte for byte on both arms:
  the mark is stripped at decode, each
  ending is one boundary, and a missing
  final boundary leaves the last line
  whole. Bytes that are not valid UTF-8
  refuse on both arms at exit 1, with
  empty stdout and the one stderr line
  naming the cannot-read rules file. A
  file holding only the byte order mark
  decodes to the empty text and scores
  the minimal 176-byte document, zero
  scores and zero observations, the
  tape read and scored once. The family
  ran in stdout mode only and never
  named an output, so it does not touch
  the resolver walk. No conviction;
  nothing is repaired.
- UH-86. UH-73's text does not spell
  its output construction; this round's
  measurement family does not touch the
  resolver walk, so the observation is
  carried by name again: the
  head-spelling posture (a head spelling
  replaces the chain's head link itself
  and lands at exit 0 on both sides; a
  through-chain spelling refuses at the
  write; the guard resolves through 990
  splices and fails at the 991st under
  every construction) stands on the
  Round 11 reviews' reproduction,
  uncontracted by any item, and is owed
  to the next round whose family touches
  the walk.
- UH-87. README records 13 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in the
  standing form of UH-69, UH-77, and
  UH-82, and the record takes effect
  only when the owner merges after the
  reviews, fixes, and owner
  verification.

## Round 14. The read-back, the standing regression, and the tape file's byte surface (UH-88 to UH-92)

- UH-88. The owed read-back of PR 16's
  record, read back whole through the
  API. PR 16 stands merged at 68b1fe02,
  its single parent 1942cb39, the merge
  tree 9e58e1f9 identical to the
  reviewed closure head tree, and the
  README on main recording 13 of 100.
  The body stands in its final form at
  13,191 characters, the bot's
  release-notes appendix at 3,327 bytes
  from its marker, the owner completion
  record appended to it with the three
  reviews APPROVE on every axis. The
  census at this read-back: four issue
  comments, zero reviews, zero review
  comments. Three comments stood at
  landing (the Trunk merged note, the
  summary shell, and the lane's merge
  request); the fourth is the owner's
  landing comment 7046560634 itself,
  posted after the merge, and no
  CodeRabbit comment was posted after
  the landing, so nothing post-landing
  stands to disposition. The shell
  7043992738 in its true final form at
  this read-back is 11,311 characters,
  last edited 2026-11-20T16:19:39Z, its
  coverage naming the closure head
  00e4493d as both source and covered
  commit, pre-merge checks at 4 passed
  and none failed, Merge Risk at
  Minimal, Run ID
  cbd79e54-61be-4888-b9db-f8454dc177d7,
  and the sentence that no actionable
  comments were generated in the recent
  review. The build lane first recorded
  the shell at 11,309 characters in the
  body; the owner record states it
  re-rendered to 11,311 characters, two
  characters longer, with every
  disposition-bearing element
  unchanged, and this read-back
  confirms 11,311 as its true final
  form. Its walkthrough carries two
  en dash characters in its item
  ranges, in bot-generated text the
  lane cannot edit, as the owner record
  states. Its one style observation,
  the title generic in its framing with
  the verdict unchanged, stands
  recorded in the body. There is
  nothing to fix and nothing to
  dismiss, and the absence is the
  disposition. This item also contracts
  the two prose errata from Round 13's
  reviews by name, in the standing
  erratum shape the landing comment
  charges. First, the UH-83 summary
  sentence's runs-and-lines conflation:
  the body's summary sentence said N8
  and N11 each split into two runs for
  eighteen PASS lines. The exact split,
  as UH-83's own text states it: N8
  runs as a forward build, a reversed
  build, and a creation-order-free
  comparison, two twin runs plus a
  third PASS line, the comparison
  report; and N11 runs as an empty
  rules file and a headings-only rules
  file, two runs. Fifteen table entries
  plus two extra N8 lines and one extra
  N11 line give eighteen PASS lines
  across fifteen distinct cells, and
  the eighteen-line total matches the
  battery12 log line for line. Second,
  UH-85's novelty sentence scoped
  against UH-22: UH-85's sentence said
  the rules file's byte surface is a
  surface no earlier item contracts.
  Scoped exactly, UH-22 already
  contracts the decode discipline
  itself, that the rules file decodes
  as utf-8-sig and a read or decode
  failure is the named cannot-read
  refusal, and UH-85's cell R7
  re-measures that refusal twin-wise.
  UH-85's fresh part stands as its
  byte-surface counts, that the base
  contract held no count for a byte
  order mark, a carriage return, or a
  line ending, and its measured matter,
  mark stripping equivalence and the
  carriage-return, line-feed, and
  paired-ending boundary behavior,
  twin-measured. No measurement is
  wrong in either erratum; the
  corrections stand contracted here by
  name, and no tree change was owed.
- UH-89. The standing regression, in
  the contracted forms, measured at the
  base (the Round 13 merge 68b1fe02,
  its tree verified blob for blob
  against the API tree before any cell
  was trusted). The oracle pairing
  stands as UH-65 verifies it: the
  archive of the Python final commit
  03658d9, tree ffa48f27, verified blob
  for blob against the driven copy at
  this round's hand, and its own suite
  passing 28. The gate at the base and
  again on this round's final tree:
  cargo test 33 passed and 0 failed (4
  unit, 24 scoring, 5 shapes), clippy
  with warnings denied clean, fmt
  clean, from untruncated captures at
  both trees, and the two builds
  byte-identical. battery10 passes
  whole: ALL PASS, the realistic corpus
  at 235 rules, 24 tapes, 5,276
  events, and 42,941 observations over
  an 8,181,065-byte document.
  battery11 stands at its settled
  posture: 18 PASS, and exactly the two
  settled cells X989 and X990 failing
  against the Round 11 instrument's
  refusal expectation while both arms
  land the write by head-link
  replacement at exit 0, read as the
  settled mis-specification from PR
  13's landing, not a conviction;
  X991 and X992 stand in the crash
  class and the sequential-splice cell
  stands as contracted. battery12
  stands whole at UH-79's dispositions
  as contracted in UH-83: eighteen PASS
  lines, FAILURES: 0. battery13 stands
  whole at UH-85's dispositions: R1 to
  R8 all PASS with the variants-equal-
  control cross-check, FAILURES: 0. The
  differential harness stands at 15
  cases and 134 steps, running clean
  against this round's build (0
  failures, the build's sha256 verified
  identical at the harness's standing
  path), the harness file unmodified,
  and is unchanged this round: no case
  convicted, so no case was added. The
  Quickstart capture reproduces the
  README's first-screen document byte
  for byte at 1,687 bytes, exit 0, from
  the synthetic rules against the
  synthetic tape.
- UH-90. The tape file's byte surface,
  measured fresh against the Python
  oracle, a surface no earlier item
  contracts in its scoped form: the
  contract holds no byte-order-mark,
  carriage, or line-ending count for a
  tape file, Round 13 measured the
  rules file's byte surface, and Round
  12 measured the tape directory's
  candidacy and the degenerate inputs
  at plain shipped bytes. Both tape
  readers decode the tape file as plain
  utf-8, not utf-8-sig, and both feed
  the decoded text to a strict JSON
  reader for which a leading byte order
  mark is its own refusal before any
  scanning, while the JSON whitespace
  both readers skip is exactly space,
  tab, line feed, and carriage return.
  The twins agreed in every cell: exit,
  stderr, the input tree untouched, and
  the document byte for byte once the
  port's version line is stripped.
  Carriage-return-plus-line-feed
  endings, carriage-return endings
  alone, and a missing final newline
  each reproduce the control's document
  byte for byte on both arms, the tape
  read and scored once: each ending
  byte between JSON tokens is
  whitespace, and a missing final
  whitespace byte leaves the value
  whole. A leading byte order mark, the
  mark with paired endings together,
  and a file holding only the mark each
  exclude the tape on both arms at exit
  0 with the one reason that the file
  is not valid JSON and no defects, one
  tape read and zero scored, and the
  three documents equal each other on
  each arm and across arms. Bytes that
  are not valid UTF-8 exclude the tape
  on both arms at exit 0 with the one
  reason that the file is not valid
  UTF-8 and no defects, its document
  equal to the mark document except
  for the exclusion's reason alone. The
  family ran in stdout mode only and
  never named an output, so it does not
  touch the resolver walk. No
  conviction; nothing is repaired.
- UH-91. UH-73's text does not spell
  its output construction; this round's
  measurement family does not touch the
  resolver walk, so the observation is
  carried by name again: the
  head-spelling posture (a head spelling
  replaces the chain's head link itself
  and lands at exit 0 on both sides; a
  through-chain spelling refuses at the
  write; the guard resolves through 990
  splices and fails at the 991st under
  every construction) stands on the
  Round 11 reviews' reproduction,
  uncontracted by any item, and is owed
  to the next round whose family touches
  the walk.
- UH-92. README records 14 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in the
  standing form of UH-69, UH-77, UH-82,
  and UH-87, and the record takes
  effect only when the owner merges
  after the reviews, fixes, and owner
  verification.

## Round 15. The read-back, the standing regression, and the tape file's value-shape surface (UH-93 to UH-97)

- UH-93. The owed read-back of PR 18's
  record, read back whole through the
  API. PR 18 stands merged at
  2cfab7cf, its single parent
  68b1fe02, the merge tree 07648ecf
  identical to the reviewed closure
  head tree, and the README on main
  recording 14 of 100. The body stands
  in its final form at 12,280
  characters, the lane body at 9,773
  characters with the bot's
  release-notes appendix at 486 bytes
  from its marker, the owner
  completion record appended to it
  with the three reviews APPROVE on
  every axis. The census at this
  read-back: four issue comments, zero
  reviews, zero review comments. Three
  comments stood at landing (the Trunk
  merged note, the summary shell, and
  the lane's merge request); the
  fourth is the owner's landing
  comment 7048711044 itself, posted
  after the merge, and no CodeRabbit
  comment was posted after the
  landing, so nothing post-landing
  stands to disposition. The shell
  7046979655 in its true final form at
  this read-back is 11,312 characters,
  last edited 2026-11-20T18:11:35Z,
  its change assessment naming the
  closure head ecdd216f, pre-merge
  checks at 4 passed and none failed,
  Merge Risk at Minimal, Run ID
  4884b237-6eb8-488c-b17d-71dbdb38c309,
  and the sentence that no actionable
  comments were generated in the
  recent review. The build lane first
  recorded the shell at 11,310
  characters in the body; the owner
  record states it re-rendered to
  11,312 characters, two characters
  longer, with every
  disposition-bearing element
  unchanged, and this read-back
  confirms 11,312 as its true final
  form. Its one style observation, the
  title long at 146 characters and
  using an internal term with the
  verdict unchanged, stands recorded
  in the body. There is nothing to fix
  and nothing to dismiss, and the
  absence is the disposition. This
  item also contracts the citation
  repair from Round 14's spec review
  by name, in the standing erratum
  shape the landing comment charges.
  UH-90's novelty sentence scoped its
  claim by counts and by the Round 12
  and Round 13 distinctions but did
  not name UH-19. Scoped exactly,
  UH-19 already contracts the tape
  decode discipline itself, that tape
  bytes decode as strict UTF-8 with no
  signature stripping, a decode
  failure names the file not valid
  UTF-8, and an unreadable file cannot
  be read, and UH-90's cell T7
  re-measures that refusal twin-wise;
  UH-19 is the tape analogue of UH-22,
  whose scoping for the rules side
  UH-88 contracts. UH-90's fresh part
  stands as its byte-surface counts,
  that the base contract held no
  byte-order-mark, carriage, or
  line-ending count for a tape file,
  and its measured matter, the mark as
  a JSON refusal before any scanning
  and the carriage-return, line-feed,
  and paired-ending bytes as JSON
  whitespace between tokens,
  twin-measured. UH-90's scoped claim
  and figures stand as written; the
  citation alone was owed, no
  measurement is wrong, and no tree
  change was owed. - UH-94. The
  standing regression, in the
  contracted forms, measured at the
  base (the Round 14 merge 2cfab7cf,
  its tree verified blob for blob
  against the API tree before any cell
  was trusted). The oracle pairing
  stands as UH-65 verifies it: the
  archive of the Python final commit
  03658d9, tree ffa48f27, verified
  blob for blob against the driven
  copy at this round's hand, and its
  own suite passing 28. The gate at
  the base and again on this round's
  final tree: cargo test 33 passed and
  0 failed (4 unit, 24 scoring, 5
  shapes), clippy with warnings denied
  clean, fmt clean, from untruncated
  captures at both trees, and the two
  builds byte-identical. battery10
  passes whole: ALL PASS, the
  realistic corpus at 235 rules, 24
  tapes, 5,276 events, and 42,941
  observations over an 8,181,065-byte
  document. battery11 stands at its
  settled posture: 18 PASS, and
  exactly the two settled cells X989
  and X990 failing against the Round
  11 instrument's refusal expectation
  while both arms land the write by
  head-link replacement at exit 0,
  read as the settled
  mis-specification from PR 13's
  landing, not a conviction; X991 and
  X992 stand in the crash class and
  the sequential-splice cell stands as
  contracted. battery12 stands whole
  at UH-79's dispositions as
  contracted in UH-83: eighteen PASS
  lines, FAILURES: 0. battery13 stands
  whole at UH-85's dispositions: R1 to
  R8 all PASS with the
  variants-equal-control cross-check,
  FAILURES: 0. battery14 stands whole
  at UH-90's dispositions: T1 to T8
  all PASS with both cross-checks,
  FAILURES: 0. The differential
  harness stands at 15 cases and 134
  steps, running clean against this
  round's build (0 failures, the
  build's sha256 verified identical at
  the harness's standing path), the
  harness file unmodified, and is
  unchanged this round: no case
  convicted, so no case was added. The
  Quickstart capture reproduces the
  README's first-screen document byte
  for byte at 1,687 bytes, exit 0,
  from the synthetic rules against the
  synthetic tape. - UH-95. The tape
  file's value-shape surface, measured
  fresh against the Python oracle: how
  a file that is valid UTF-8 and valid
  JSON but not a valid tape is
  excluded. Its novelty stands scoped
  against its standing neighbours by
  name: UH-5a defines the tape subset
  the format checks enforce, as a
  definition; UH-19 contracts the
  decode and reader refusals before
  any defect is computed; UH-20
  contracts the defect computation's
  order and phrases, as the
  computation's statement; UH-79
  measured the tape directory's
  candidacy and the degenerate inputs,
  among them a zero-event tape, at
  plain shipped constructions. No
  earlier item contracts the
  twin-measured exclusion documents
  for this shape lattice, and the
  defect phrases and order below are
  UH-20's, re-measured, not new. Every
  varied file parses on both arms, so
  no UH-19 refusal fires; the twins
  agreed in every cell: exit, stderr,
  the input tree untouched, and the
  document byte for byte once the
  port's version line is stripped. A
  top-level array and a top-level
  string each exclude the tape on both
  arms at exit 0 with the one reason
  that the file is not a valid tape
  and the one defect that the tape is
  not an object, and the two documents
  equal each other on each arm and
  across arms. An empty object
  excludes with three defects in the
  checks' order, tape_version is not
  1.0, run is not an object, and
  events must hold at least one event.
  A version field holding the JSON
  number 1.0 excludes with the one
  defect that tape_version is not 1.0;
  a run field holding an array
  excludes with the one defect that
  run is not an object; an events
  array holding a number excludes with
  the one defect that events[0] is not
  an object. An event with seq 7 and a
  kind outside the format enum
  excludes with two defects in order,
  events[0].seq is not 0 and
  events[0].kind is not in the format
  enum. A tool result answering a call
  id no earlier tool call used, with
  is_error as a string, excludes with
  two defects in order, the call id
  answers no earlier tool_call with
  the id rendered as CPython renders
  it, and events[0].is_error must be a
  boolean. Every exclusion records one
  tape read and zero scored, and the
  exclusion documents differ from each
  other only in the exclusion's
  defects array. The family ran in
  stdout mode only and never named an
  output, so it does not touch the
  resolver walk. No conviction;
  nothing is repaired. - UH-96.
  UH-73's text does not spell its
  output construction; this round's
  measurement family does not touch
  the resolver walk, so the
  observation is carried by name
  again: the head-spelling posture (a
  head spelling replaces the chain's
  head link itself and lands at exit 0
  on both sides; a through-chain
  spelling refuses at the write; the
  guard resolves through 990 splices
  and fails at the 991st under every
  construction) stands on the Round 11
  reviews' reproduction, uncontracted
  by any item, and is owed to the next
  round whose family touches the walk.
  - UH-97. README records 15 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in the
  standing form of UH-69, UH-77,
  UH-82, UH-87, and UH-92, and the
  record takes effect only when the
  owner merges after the reviews,
  fixes, and owner verification.

## Round 16. The read-back, the standing regression, and the rules file's line-boundary surface (UH-98 to UH-102)

- UH-98. The owed read-back of PR 20's
  record, read back whole through the
  API. PR 20 stands merged at 5f1a7bc6,
  its single parent 2cfab7cf, the merge
  tree f7c1edb4 identical to the
  reviewed closure head tree, and the
  README on main recording 15 of 100.
  The body stands in its final form at
  12,949 characters, the lane body at
  10,107 characters with the bot's
  release-notes appendix at 588
  characters from its marker at char
  9,519, the owner completion record
  appended to it (2,842 characters with
  its leading blank lines, its heading
  at char 10,109) with the three
  reviews APPROVE on every axis. The
  census at this read-back: four issue
  comments, zero reviews, zero review
  comments. Three comments stood at the
  owner record (the Trunk merged note,
  the summary shell, and the lane's
  merge request); the fourth is the
  owner's landing comment 7053312318
  itself, posted after the merge, and
  no CodeRabbit comment was posted
  after the landing, so nothing
  post-landing stands to disposition.
  The shell 7051040137 in its true
  final form at this read-back is
  11,312 characters, last edited
  2026-11-20T21:31:28Z, its change
  assessment naming the closure head
  f24d8134, pre-merge checks at 4
  passed and none failed, Merge Risk
  at Minimal, Run ID
  e5ac1af9-e0f9-43ac-a99d-b0e67d37c1b8,
  and the sentence that no actionable
  comments were generated in the recent
  review. The build lane first recorded
  the shell at 11,310 characters in the
  body at its earlier edited stamp; the
  owner record states it at 11,312
  characters, two characters longer,
  with every disposition-bearing
  element unchanged, and this read-back
  confirms 11,312 as its true final
  form. Its one style observation, the
  title long and dense at 173
  characters with the verdict
  unchanged, stands recorded in the
  body. There is nothing to fix and
  nothing to dismiss, and the absence
  is the disposition.
- UH-99. The standing regression, in
  the contracted forms, measured at the
  base (the Round 15 merge 5f1a7bc6,
  its tree verified blob for blob
  against the API tree before any cell
  was trusted). The oracle pairing
  stands as UH-65 verifies it: the
  archive of the Python final commit
  03658d9, tree ffa48f27, verified blob
  for blob against the driven copy at
  this round's hand, and its own suite
  passing 28. The gate at the base and
  again on this round's final tree:
  cargo test 33 passed and 0 failed (4
  unit, 24 scoring, 5 shapes), clippy
  with warnings denied clean, fmt
  clean, from untruncated captures at
  both trees, and the two builds
  byte-identical. battery10 passes
  whole: ALL PASS, the realistic corpus
  at 235 rules, 24 tapes, 5,276
  events, and 42,941 observations over
  an 8,181,065-byte document.
  battery11 stands at its settled
  posture: 18 PASS, and exactly the two
  settled cells X989 and X990 failing
  against the Round 11 instrument's
  refusal expectation while both arms
  land the write by head-link
  replacement at exit 0, read as the
  settled mis-specification from PR
  13's landing, not a conviction;
  X991 and X992 stand in the crash
  class and the sequential-splice cell
  stands as contracted. battery12
  stands whole at UH-79's dispositions
  as contracted in UH-83: eighteen PASS
  lines, FAILURES: 0. battery13 stands
  whole at UH-85's dispositions: R1 to
  R8 all PASS with the variants-equal-
  control cross-check, FAILURES: 0.
  battery14 stands whole at UH-90's
  dispositions: T1 to T8 all PASS with
  both cross-checks, FAILURES: 0.
  battery15 stands whole at UH-95's
  dispositions: V1 to V9 all PASS with
  both cross-checks, FAILURES: 0. The
  differential harness stands at 15
  cases and 134 steps, running clean
  against this round's build (0
  failures, the build's sha256 verified
  identical at the harness's standing
  path), the harness file unmodified,
  and is unchanged this round: no case
  convicted, so no case was added. The
  Quickstart capture reproduces the
  README's first-screen document byte
  for byte at 1,687 bytes, exit 0, from
  the synthetic rules against the
  synthetic tape.
- UH-100. The rules file's
  line-boundary surface, measured fresh
  against the Python oracle, a surface
  no earlier item contracts in its
  scoped form: the contract holds no
  count for any line boundary beyond
  the LF, CR, and CRLF endings UH-85
  measured, Round 15 measured the tape
  file's value-shape surface, Round 14
  the tape file's byte surface, and
  Round 12 the tape directory's
  candidacy and the degenerate inputs
  at plain shipped constructions. Its
  novelty stands scoped against its
  standing neighbours by name: UH-22
  contracts the rules decode discipline
  itself, that the rules file decodes
  as utf-8-sig and a read or decode
  failure is the named cannot-read
  refusal, and UH-85's cell R7
  re-measures that refusal; UH-85's
  fresh part stands as its byte-surface
  counts and mark and ending behaviour
  at LF family bytes alone. Both rules
  readers split the decoded text under
  the interpreter's splitlines
  discipline, and the twins agreed in
  every cell: exit, stderr, the input
  tree untouched, and the document byte
  for byte once the port's version line
  is stripped. Every LF byte of the
  synthetic rules file replaced by a
  vertical tab, a form feed, a file
  separator, a group separator, a
  record separator, a next line, a line
  separator, or a paragraph separator
  each reproduces the control's
  document byte for byte on both arms,
  the tape read and scored once, six
  scores and one observation: each
  boundary scalar ends a line where the
  LF stood, and each varied file holds
  exactly the control's eight boundary
  encodings. The family ran in stdout
  mode only and never named an output,
  so it does not touch the resolver
  walk. No conviction; nothing is
  repaired.
- UH-101. UH-73's text does not spell
  its output construction; this round's
  measurement family does not touch the
  resolver walk, so the observation is
  carried by name again: the
  head-spelling posture (a head spelling
  replaces the chain's head link itself
  and lands at exit 0 on both sides; a
  through-chain spelling refuses at the
  write; the guard resolves through 990
  splices and fails at the 991st under
  every construction) stands on the
  Round 11 reviews' reproduction,
  uncontracted by any item, and is owed
  to the next round whose family touches
  the walk.
- UH-102. README records 16 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in the
  standing form of UH-69, UH-77, UH-82,
  UH-87, UH-92, and UH-97, and the
  record takes effect only when the
  owner merges after the reviews,
  fixes, and owner verification.

## Round 17. The read-back, the carried build reproduction discharged, the standing regression, and the rules file's structural surface (UH-103 to UH-108)

- UH-103. The owed read-back of PR 22's
  record, read back whole through the
  API. PR 22 stands merged at 042d4b29,
  its single parent 5f1a7bc6, the merge
  tree aa480376 identical to the
  reviewed closure head tree, and the
  README on main recording 16 of 100.
  The body stands in its final form at
  14,636 characters, the lane body at
  10,420 characters, the owner
  completion record appended to it
  (4,216 characters with its leading
  blank lines) with the three reviews
  APPROVE on every axis (spec one
  record-only Minor, standards one
  record-only Minor, red team no
  findings against the slice and one
  explicit limitation, the carried
  build reproduction this round
  discharges at UH-104). The census at
  this read-back: four issue comments,
  zero reviews, zero review comments.
  Three comments stood at the owner
  record (the Trunk merged note, the
  summary shell, and the lane's merge
  request); the fourth is the owner's
  landing comment 7057193223 itself,
  posted after the merge, and no
  CodeRabbit comment was posted after
  the landing, so nothing post-landing
  stands to disposition beyond the
  charges themselves. The shell
  7056728114 in its true final form at
  this read-back is 11,381 characters,
  last edited 2026-11-21T03:40:26Z, its
  change assessment naming the closure
  head cffdda6f, pre-merge checks at 4
  passed and none failed, Merge Risk
  at Minimal, Run ID
  a57de23f-c564-4ac4-bb2e-091b3b3b1ae8,
  and the sentence that no actionable
  comments were generated in the recent
  review. The build lane first recorded
  the shell at 11,379 characters in the
  body at its earlier edited stamp; the
  owner record states it at 11,381
  characters, two characters longer,
  with every disposition-bearing
  element unchanged, and this read-back
  confirms 11,381 as its true final
  form. Its one style observation, the
  title long at 135 characters with the
  verdict Passed, stands recorded in
  the body. There is nothing to fix and
  nothing to dismiss, and the absence
  is the disposition.
- UH-104. The carried independent-build
  reproduction, discharged by name
  against Round 16's red team
  limitation. The Round 16 red team's
  own builds never ran in its sitting:
  its ready runner probed for an open
  write window for 46 consecutive
  cycles and every probe file stood at
  zero bytes, so the independent-build
  confirmation that stood for five
  consecutive rounds was not counted
  for Round 16 and was carried to this
  round by name. At this round's hand
  the landed Round 16 tree, which is
  this round's base tree (042d4b29, its
  tree verified blob for blob against
  the API tree before any cell was
  trusted), is built fresh in the
  retry-hold shape inside this sitting
  under the build slot, at this lane's
  own target directory: cargo test 33
  passed and 0 failed (4 unit, 24
  scoring, 5 shapes), clippy with
  warnings denied clean, fmt clean,
  the build standing at sha256
  8ed5606c and 1,579,944 bytes, the
  same bytes the Round 16 lane built.
  battery16 runs whole at this hand
  beside battery15 and battery14 whole,
  each at its contracted dispositions
  with FAILURES: 0. The discharge
  stands on this build and these runs,
  at this hand, and on nothing carried
  from Round 16's logs.
- UH-105. The standing regression, in
  the contracted forms, measured at the
  base (the Round 16 merge 042d4b29,
  its tree verified blob for blob
  against the API tree before any cell
  was trusted). The oracle pairing
  stands as UH-65 verifies it: the
  archive of the Python final commit
  03658d9, tree ffa48f27, verified blob
  for blob against the driven copy at
  this round's hand, and its own suite
  passing 28. The gate at the base and
  again on this round's final tree:
  cargo test 33 passed and 0 failed (4
  unit, 24 scoring, 5 shapes), clippy
  with warnings denied clean, fmt
  clean, from untruncated captures at
  both trees, and the two builds
  byte-identical. battery10 passes
  whole: ALL PASS, the realistic corpus
  at 235 rules, 24 tapes, 5,276
  events, and 42,941 observations over
  an 8,181,065-byte document.
  battery11 stands at its settled
  posture: 18 PASS, and exactly the two
  settled cells X989 and X990 failing
  against the Round 11 instrument's
  refusal expectation while both arms
  land the write by head-link
  replacement at exit 0, read as the
  settled mis-specification from PR
  13's landing, not a conviction;
  X991 and X992 stand in the crash
  class and the sequential-splice cell
  stands as contracted. battery12
  stands whole at UH-79's dispositions
  as contracted in UH-83: eighteen PASS
  lines, FAILURES: 0. battery13 stands
  whole at UH-85's dispositions: R1 to
  R8 all PASS with the variants-equal-
  control cross-check, FAILURES: 0.
  battery14 stands whole at UH-90's
  dispositions: T1 to T8 all PASS with
  both cross-checks, FAILURES: 0.
  battery15 stands whole at UH-95's
  dispositions: V1 to V9 all PASS with
  both cross-checks, FAILURES: 0.
  battery16 stands whole at UH-100's
  dispositions: L1 to L9 all PASS with
  both cross-checks, FAILURES: 0. The
  differential harness stands at 15
  cases and 134 steps, running clean
  against this round's build (0
  failures, the build's sha256 verified
  identical at the harness's standing
  path), the harness file unmodified,
  and is unchanged this round: no case
  convicted, so no case was added. The
  Quickstart capture reproduces the
  README's first-screen document byte
  for byte at 1,687 bytes, exit 0, from
  the synthetic rules against the
  synthetic tape.
- UH-106. The rules file's structural
  surface, measured fresh against the
  Python oracle, a surface no earlier
  item contracts in its scoped form:
  the contract holds no count for a
  checkbox, a marker form, a fenced or
  comment block, a setext heading, a
  duplicate rule, or an unclosed fence
  at the score CLI, Round 16 measured
  the rules file's line-boundary
  surface, Round 15 the tape file's
  value-shape surface, Round 14 the
  tape file's byte surface, and Round
  13 the rules file's byte surface.
  Its novelty stands scoped against
  its standing neighbours by name:
  UH-22 contracts the rules decode
  discipline itself, that the rules
  file decodes as utf-8-sig and a read
  or decode failure is the named
  cannot-read refusal, and UH-5 the
  inventory discipline at the shared
  synthetic fixture, which carries
  none of these constructions. Both
  rules readers state the same
  inventory law, and the twins agreed
  in every cell: exit, stderr, the
  input tree untouched, and the
  document byte for byte once the
  port's version line is stripped. A
  rule spelled with a checkbox, every
  marker spelled as a numbered marker,
  the markers spelled as asterisks and
  plus signs, a fenced block holding a
  bullet line, a comment block holding
  a bullet line, and the heading
  spelled as a setext heading each
  reproduce the control's document
  byte for byte on both arms, the tape
  read and scored once, six scores and
  one observation. A repeated rule
  adds exactly one key, a duplicates
  array holding the one repeated text;
  an unclosed fence adds exactly one
  key, a warnings array holding the
  one warning that names its opening
  line at line 9 and states the rest
  of the file was read as code. The
  family ran in stdout mode only and
  never named an output, so it does
  not touch the resolver walk. No
  conviction; nothing is repaired.
- UH-107. UH-73's text does not spell
  its output construction; this round's
  measurement family does not touch the
  resolver walk, so the observation is
  carried by name again: the
  head-spelling posture (a head spelling
  replaces the chain's head link itself
  and lands at exit 0 on both sides; a
  through-chain spelling refuses at the
  write; the guard resolves through 990
  splices and fails at the 991st under
  every construction) stands on the
  Round 11 reviews' reproduction,
  uncontracted by any item, and is owed
  to the next round whose family touches
  the walk.
- UH-108. README records 17 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in the
  standing form of UH-69, UH-77, UH-82,
  UH-87, UH-92, UH-97, and UH-102, and
  the record takes effect only when the
  owner merges after the reviews,
  fixes, and owner verification.

## Round 18. The read-back, the standing regression, and the rules file's block-closure surface (UH-109 to UH-113)

- UH-109. The owed read-back of PR 24's
  record, read back whole through the
  API. PR 24 stands merged at 3d5b3431,
  its single parent 042d4b29, the merge
  tree 33cebe0c identical to the
  reviewed closure head tree, and the
  README on main recording 17 of 100.
  The body stands in its final form at
  17,961 characters, the lane body at
  12,865 characters, the owner
  completion record appended to it
  (5,096 characters with its leading
  blank lines) with the three reviews
  APPROVE on every axis (spec three
  record-only Minors, standards one
  record-only Minor, red team no
  findings against the slice, one
  record-only Minor shared with
  standards, and one explicit
  limitation, Round 17's red team's own
  builds never running in its sitting,
  charged with no new carry because the
  unbuilt code is byte-identical to the
  landed Round 16 tree's code and
  UH-104 built that code fresh in Round
  17). The census at this read-back:
  four issue comments, zero reviews,
  zero review comments. Three comments
  stood at the owner record (the Trunk
  merged note, the summary shell, and
  the lane's merge request); the fourth
  is the owner's landing comment
  7062504747 itself, posted after the
  merge, and no CodeRabbit comment was
  posted after the landing, so nothing
  post-landing stands to disposition
  beyond the charges themselves. The
  shell 7061695722 in its true final
  form at this read-back is 11,652
  characters, last edited
  2026-11-21T11:13:34Z, its change
  assessment naming the closure head
  8a53dde9, pre-merge checks at 4
  passed and none failed, Merge Risk
  at Minimal, Run ID
  9c7acee2-ed73-4143-b40c-2188a8c6c38e,
  and the sentence that no actionable
  comments were generated in the recent
  review. The build lane first recorded
  the shell at 11,650 characters in the
  body at its earlier edited stamp; the
  owner record states it at 11,652
  characters, two characters longer,
  with every disposition-bearing
  element unchanged, and this read-back
  confirms 11,652 as its true final
  form. Its one style observation, the
  title long at 140 characters with the
  verdict Passed, stands recorded in
  the body. There is nothing to fix and
  nothing to dismiss, and the absence
  is the disposition. This entry also
  contracts Round 16's red team
  limitation in full by name, as the
  owner record charges: the Round 16
  red team's own builds never ran in
  its sitting (the write wave stood
  closed for its whole sitting, its
  probes standing at zero bytes through
  46 probe cycles), its ready runner
  was stopped when its report finalized
  so nothing raced it, and the
  independent-build confirmation that
  stood for five consecutive rounds was
  not counted for Round 16 and stood
  carried until UH-104 discharged it in
  Round 17. The CONTRACT's UH-104 entry
  omitted the runner-stopped clause;
  this entry carries the limitation
  whole, that clause included.
- UH-110. The standing regression, in
  the contracted forms, measured at the
  base (the Round 17 merge 3d5b3431,
  its tree verified blob for blob
  against the API tree before any cell
  was trusted). The oracle pairing
  stands as UH-65 verifies it: the
  archive of the Python final commit
  03658d9, tree ffa48f27, verified blob
  for blob against the driven copy at
  this round's hand, and its own suite
  passing 28. The gate at the base and
  again on this round's final tree:
  cargo test 33 passed and 0 failed (4
  unit, 24 scoring, 5 shapes), clippy
  with warnings denied clean, fmt
  clean, from untruncated captures at
  both trees, and the two builds
  byte-identical at sha256 8ed5606c and
  1,579,944 bytes. battery10 passes
  whole: ALL PASS, the realistic corpus
  at 235 rules, 24 tapes, 5,276
  events, and 42,941 observations over
  an 8,181,065-byte document.
  battery11 stands at its settled
  posture: 18 PASS, and exactly the two
  settled cells X989 and X990 failing
  against the Round 11 instrument's
  refusal expectation while both arms
  land the write by head-link
  replacement at exit 0, read as the
  settled mis-specification from PR
  13's landing, not a conviction;
  X991 and X992 stand in the crash
  class and the sequential-splice cell
  stands as contracted. battery12
  stands whole at UH-79's dispositions
  as contracted in UH-83: eighteen PASS
  lines, FAILURES: 0. battery13 stands
  whole at UH-85's dispositions: R1 to
  R8 all PASS with the variants-equal-
  control cross-check, FAILURES: 0.
  battery14 stands whole at UH-90's
  dispositions: T1 to T8 all PASS with
  both cross-checks, FAILURES: 0.
  battery15 stands whole at UH-95's
  dispositions: V1 to V9 all PASS with
  both cross-checks, FAILURES: 0.
  battery16 stands whole at UH-100's
  dispositions: L1 to L9 all PASS with
  both cross-checks, FAILURES: 0.
  battery17 stands whole at UH-106's
  dispositions: K1 to K9 all PASS with
  both cross-checks, FAILURES: 0, the
  six alternate constructions
  reproducing the control byte for
  byte, the duplicate rule adding
  exactly the duplicates key, and the
  unclosed fence adding exactly the
  warnings key naming line 9. The
  differential harness stands at 15
  cases and 134 steps, running clean
  against this round's builds (0
  failures at the base build and again
  at the head build, exit 0 both
  times), the harness file unmodified
  at sha256 110042d6, and is unchanged
  this round: no case convicted, so no
  case was added. The Quickstart
  capture reproduces the README's
  first-screen document byte for byte
  at 1,687 bytes, exit 0, at both
  trees, from the synthetic rules
  against the synthetic tape.
- UH-111. The rules file's
  block-closure surface, measured fresh
  against the Python oracle, a surface
  no earlier item contracts in its
  scoped form: the contract holds no
  count for a tilde fence, a fence
  closed by a longer run of its own
  character, a fence whose closing run
  is too short, a fence addressed by
  the other character, a fence with an
  info string, an indented fence, a
  comment that opens and closes on one
  line, or a comment block that never
  closes, at the score CLI. Round 17
  measured the rules file's structural
  surface, Round 16 its line-boundary
  surface, Round 15 the tape file's
  value-shape surface, Round 14 the
  tape file's byte surface, and Round
  13 the rules file's byte surface.
  Its novelty stands scoped against
  its standing neighbours by name:
  UH-106 contracts the three block
  constructions of one fenced block
  closed by its own three backticks,
  one comment block closed on its own
  line, and one unclosed backtick
  fence. Both rules readers state the
  same closure law, and the twins
  agreed in every cell: exit, stderr,
  the input tree untouched, and the
  document byte for byte once the
  port's version line is stripped. A
  tilde fence block holding a bullet
  line, a backtick fence closed by a
  longer run, a fence opened with an
  info string, a comment that opens and
  closes on one line holding a bullet
  line, and an indented fence block
  each reproduce the control's document
  byte for byte on both arms, the tape
  read and scored once, six scores and
  one observation. A fence addressed by
  a run of the other character and a
  fence whose closing run is too short
  each add exactly one key, a warnings
  array holding the one warning that
  names the fence's opening line at
  line 9 and states the rest of the
  file was read as code. A comment
  block that never closes adds no key
  at all and reproduces the control
  byte for byte, where an unclosed
  fence warns. The family ran in stdout
  mode only and never named an output,
  so it does not touch the resolver
  walk. No conviction; nothing is
  repaired.
- UH-112. UH-73's text does not spell
  its output construction; this round's
  measurement family does not touch the
  resolver walk, so the observation is
  carried by name again: the
  head-spelling posture (a head spelling
  replaces the chain's head link itself
  and lands at exit 0 on both sides; a
  through-chain spelling refuses at the
  write; the guard resolves through 990
  splices and fails at the 991st under
  every construction) stands on the
  Round 11 reviews' reproduction,
  uncontracted by any item, and is owed
  to the next round whose family touches
  the walk.
- UH-113. README records 18 of 100
  only in the closure commit. The
  closure commit is written before
  this round's three reviews, in the
  standing form of UH-69, UH-77, UH-82,
  UH-87, UH-92, UH-97, UH-102, and
  UH-108, and the record takes effect
  only when the owner merges after the
  reviews, fixes, and owner verification.
