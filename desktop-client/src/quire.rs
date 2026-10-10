//! The place for the Quire arm (`quire_desktop_v1`, `quire_window_v1`).
//!
//! P2 fills this: a compositor that offers the private protocols reports Spaces, placement
//! and emphasis, and accepts placement commands, which wlr cannot express (FINDINGS). Until
//! then nothing produces a [`QuireEvent`] and no command targets it. The enum is
//! `non_exhaustive` so adding variants breaks no consumer.

/// An event from the Quire protocols. Reserved: no variant yet.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum QuireEvent {}
