//! The one failure shape every module returns: a
//! named refusal, the Python's error classes all
//! printing as one error line at the CLI, or the
//! crash class, where the Python dies with an
//! uncaught exception and the port exits 1 in
//! silence (a named deviation in the dossier).

use crate::util::Msg;

#[derive(Debug)]
pub enum Failure {
    Named(Msg),
    Crash,
}

impl Failure {
    pub fn named(text: &str) -> Failure {
        Failure::Named(Msg::str(text))
    }

    pub fn named_msg(message: Msg) -> Failure {
        Failure::Named(message)
    }
}
