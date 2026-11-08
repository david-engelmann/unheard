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

```sh
unheard score rules.md tapes -o observations.json
```

The document is the one bluepencil consumes. Its observations come first, then the scores with their denominators, exercised of total and violations of exercised, never a bare rate. A rule never exercised is a different state from a rule never measured, and the document keeps the two apart in the unexercised and unmeasured lists.

The rules it measures are the ones written in one of six checkable shapes. A named tool prohibited. A named path prohibited. A required ordering of two named tools. A required tool for files of a named kind. A numeric cap on one tool's calls in a session. The tool a session must open with. Every other rule is reported unmeasured, with the count published, and nothing is scored by guesswork.

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

Closed perfection rounds, 7 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed.

## License

MIT. See LICENSE.
