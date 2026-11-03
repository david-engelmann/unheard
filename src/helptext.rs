//! The help and usage texts, pinned byte for byte
//! from the Python's argparse at COLUMNS=80 (the
//! family's standing pin; the Python wraps to the
//! terminal width, the port stays at 80).

pub const TOP_HELP: &str = "\
usage: unheard [-h] {score} ...

positional arguments:
  {score}
    score     score rules against tapes

options:
  -h, --help  show this help message and exit
";

pub const TOP_USAGE: &str = "usage: unheard [-h] {score} ...";

pub const SCORE_HELP: &str = "\
usage: unheard score [-h] [-o OUTPUT] rules tapes

positional arguments:
  rules
  tapes

options:
  -h, --help            show this help message and exit
  -o OUTPUT, --output OUTPUT
";

pub const SCORE_USAGE: &str = "usage: unheard score [-h] [-o OUTPUT] rules tapes";
