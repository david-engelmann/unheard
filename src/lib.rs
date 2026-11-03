//! unheard: obedience scoring for standing
//! instructions. It takes a rules file and a
//! directory of recorded runs, and scores the
//! checkable rules by what the runs actually did:
//! counts of exercises and violations, never
//! impressions (UH-1 to UH-16). It measures the
//! hearing and never edits; the editing is the
//! sibling tool's work.
//!
//! Round 3 is the Rust port (UH-17 to UH-24). It
//! consumes no sibling crate: the tape reader and
//! the canonical digest stand in this crate, the
//! format consumed pinned by the family's golden
//! fixture (UH-18), and sha2 is the only external
//! dependency, for that digest.

pub mod cli;
pub mod digest;
pub mod emit;
pub mod failure;
pub mod helptext;
pub mod inventory;
pub mod json;
pub mod pyrepr;
pub mod repr_table;
pub mod scoring;
pub mod shapes;
pub mod tapes;
pub mod util;
pub mod value;

/// The command line entry, in the cli module.
pub fn run_cli() -> i32 {
    // A generous stack on a spawned thread, the
    // family precedent: parsed values nest as deep
    // as their files, and cloning or dropping a deep
    // value recurses with its depth.
    let child = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(crate::cli::run_cli)
        .expect("the CLI thread starts");
    child.join().unwrap_or(1)
}
