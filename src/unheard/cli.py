"""UH-11. The score command. Unheard never writes to
the rules file or the tapes."""
from __future__ import annotations

import argparse
import json
import os
import sys
import tempfile
from pathlib import Path

from .inventory import InventoryError, parse_rules
from .scoring import score
from .tapes import FAILED, load_tape_file


def _write_atomically(path: Path, text: str) -> None:
    fd, tmp = tempfile.mkstemp(dir=str(path.parent),
                               prefix=path.name + ".")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise


def _score(args: argparse.Namespace) -> int:
    rules_path = Path(args.rules)
    tapes_dir = Path(args.tapes)
    out = Path(args.output) if args.output else None
    if not rules_path.is_file():
        print(f"error, no such rules file, {rules_path}",
              file=sys.stderr)
        return 1
    if not tapes_dir.is_dir():
        print(f"error, no such tapes directory, {tapes_dir}",
              file=sys.stderr)
        return 1
    if out is not None and (
            out.resolve() == rules_path.resolve()
            or (tapes_dir in out.resolve().parents)):
        print("error, unheard never writes to its inputs",
              file=sys.stderr)
        return 1
    try:
        rule_texts = [rule["text"] for rule in
                      parse_rules(rules_path.read_bytes().decode(
                          "utf-8-sig"))[0]]
    except (OSError, UnicodeDecodeError) as exc:
        print(f"error, cannot read the rules file, {rules_path}",
              file=sys.stderr)
        return 1
    except InventoryError as exc:
        print(f"error, {exc}", file=sys.stderr)
        return 1
    tapes, excluded = [], []
    for name in sorted(os.listdir(tapes_dir)):
        path = tapes_dir / name
        if not path.is_file():
            continue
        tape, reason, defects = load_tape_file(path)
        if tape is FAILED:
            excluded.append({"file": name, "reason": reason,
                             "defects": defects})
        else:
            tapes.append(tape)
    document = score(rule_texts, tapes, excluded)
    text = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
    try:
        if out is None:
            sys.stdout.write(text)
            sys.stdout.flush()
        else:
            _write_atomically(out, text)
    except (OSError, UnicodeEncodeError):
        if out is not None:
            print(f"error, cannot write the observations file, "
                  f"{out}", file=sys.stderr)
        else:
            print("error, cannot write the observations to stdout",
                  file=sys.stderr)
            # The broken stream would fail again at interpreter
            # shutdown and mask this refusal with exit 120 and
            # an ignored-exception dump, so it is replaced
            # with a sink before returning.
            try:
                sys.stdout.close()
            except OSError:
                pass
            sys.stdout = open(os.devnull, "w", encoding="utf-8")
        return 1
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="unheard")
    sub = parser.add_subparsers(dest="command", required=True)
    cmd = sub.add_parser("score", help="score rules against tapes")
    cmd.add_argument("rules")
    cmd.add_argument("tapes")
    cmd.add_argument("-o", "--output")
    args = parser.parse_args(argv)
    if args.command == "score":
        return _score(args)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
