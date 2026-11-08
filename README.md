# unheard

Scores every instruction by observed obedience across hundreds of runs and names the ones agents systematically ignore, then proposes rewording, relocation, or deletion.

Unheard scores a file of standing instructions against a directory of recorded runs. Each run is a flight tape in the flighttape format. For every rule it can measure it reports how many tapes exercised the rule and how many events violated it, and it prints an observation for each instance it finds, followed, ignored, or misread, citing the tape by digest and the event by its sequence number. A rule it cannot measure is named as unmeasured and counted, never scored by guesswork. Unheard measures the hearing and never edits. Bluepencil does the editing.

![ci](https://github.com/david-engelmann/unheard/actions/workflows/ci.yml/badge.svg)

```json
{
  "observations": [
    {
      "rule": "Use Editor for .py files",
      "kind": "misreading",
      "quote": "tape cc9c4e01fca418dcc33114caf75fae1f6bf0d401f297b5d61f6776903fa955a3 seq 2 used Read"
    }
  ],
  "scores": [
    {
      "rule": "Never use Grep",
      "shape": "prohibit-tool",
      "tapes_exercised": 0,
      "tapes_total": 1,
      "events_exercised": 0,
      "violations": 0
    },
    {
      "rule": "Do not edit secrets/prod.env",
      "shape": "prohibit-path",
      "tapes_exercised": 0,
      "tapes_total": 1,
      "events_exercised": 0,
      "violations": 0
    },
    {
      "rule": "Always run Lint before Push",
      "shape": "ordering",
      "tapes_exercised": 0,
      "tapes_total": 1,
      "events_exercised": 0,
      "violations": 0
    },
    {
      "rule": "Use Editor for .py files",
      "shape": "required-tool",
      "tapes_exercised": 1,
      "tapes_total": 1,
      "events_exercised": 1,
      "violations": 1
    },
    {
      "rule": "Keep the standup notes brief",
      "shape": "unmeasured",
      "tapes_exercised": 0,
      "tapes_total": 1,
      "events_exercised": 0,
      "violations": 0
    },
    {
      "rule": "Never call Teleport",
      "shape": "prohibit-tool",
      "tapes_exercised": 0,
      "tapes_total": 1,
      "events_exercised": 0,
      "violations": 0
    }
  ],
  "unmeasured": [
    "Keep the standup notes brief"
  ],
  "unmeasured_count": 1,
  "unexercised": [
    "Never use Grep",
    "Do not edit secrets/prod.env",
    "Always run Lint before Push",
    "Never call Teleport"
  ],
  "tapes": {
    "read": 1,
    "scored": 1,
    "excluded": []
  }
}
```

## Install

The install needs Git and a Rust toolchain.

```sh
git clone https://github.com/david-engelmann/unheard.git
cd unheard
export CARGO_HOME="$PWD/.cargo-home"
export PATH="$CARGO_HOME/bin:$PATH"
cargo install --path .
```

## Quickstart

Score the sample rules against the sample tape that ships in the test fixtures. The output is the document the first screen shows.

```sh
cp tests/fixtures/rules_synthetic.md rules.md
mkdir tapes
cp tests/fixtures/tape_synthetic.json tapes/
unheard score rules.md tapes
```

## Usage

The one command is score. It takes a rules file and a directory of tapes, and it writes the observations document to stdout, or to a file with -o. Writing to a file prints nothing.

The rules file is read as lists under headings. Each rule stands as a bullet item or a numbered item under a heading. A list item before any heading fails with an error naming its line, and a marker with no text after it fails the same way. Prose standing on its own is never inventoried as a rule, and a file of prose alone scores no rules at all.

```sh
unheard score rules.md tapes -o observations.json
```

The document is the one bluepencil consumes. Its observations come first, then the scores with their denominators, exercised of total and violations of exercised, never a bare rate. A rule never exercised is a different state from a rule never measured, and the document keeps the two apart in the unexercised and unmeasured lists.

The rules it measures are the ones written in one of six checkable shapes, and a shape matches only when the rule's whole text follows its accepted wording, with one period allowed at the very end. A tool prohibition opens with Never use, Do not use, Never call, or Do not call, followed by the tool's name. A path prohibition opens with Never edit, Do not edit, Never modify, Do not modify, Never touch, or Do not touch, followed by the path written as one run with no spaces in it. An ordering opens with Run, Always run, Call, or Always call, followed by one tool's name, the word before, and the other tool's name. A required tool opens with Use or Always use and the tool's name, and ends with for and an extension with its dot and the word files, or with for files ending in and an extension with its dot. A cap opens with Never call or Do not call, the tool's name, more than, a count, and times, or with Call or Use, the tool's name, at most, a count, and times, or with At most, a count, calls to, and the tool's name. A first step opens with Start with, Always start with, Begin with, Always begin with, or The first call must be, followed by the tool's name. Every other rule is reported unmeasured, with the count published, and nothing is scored by guesswork. Do not ever use Grep states the same prohibition in other words and stands unmeasured.

A path prohibition is exercised by any call whose input strings contain the path, whatever tool made the call, so a call that only reads the path counts as a violation.

Unheard never writes to its inputs, and it says so when asked to.

```sh
unheard score rules.md tapes -o rules.md
```

```text
error, unheard never writes to its inputs
```

A missing input is named the same way.

```sh
unheard score missing.md tapes
```

```text
error, no such rules file, missing.md
```

The help text is the whole command surface.

```sh
unheard score --help
```

```text
usage: unheard score [-h] [-o OUTPUT] rules tapes

positional arguments:
  rules
  tapes

options:
  -h, --help            show this help message and exit
  -o OUTPUT, --output OUTPUT
```

## Status

The score command is built and measures all six shapes. It reads flight tapes in the flighttape format, version 1.0, and writes the observations document to stdout or to a file. No other command is built, and no other run format is read.

Closed perfection rounds, 8 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.

## License

MIT. See LICENSE.
