//! Pairing a list entry with its management handle.
//!
//! Neither protocol names the other's object, so the pairing is a heuristic over what both
//! describe: app id and title, in arrival order (FINDINGS). It is pure so it is testable.

use super::model::Toplevel;
use crate::keys::ToplevelKey;
use crate::names::ToplevelIdentifier;

/// A toplevel as `ext-foreign-toplevel-list-v1` describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Listed {
    /// The list-side record's key.
    pub key: ToplevelKey,
    /// Its identifier.
    pub identifier: ToplevelIdentifier,
    /// Its title.
    pub title: String,
    /// Its app id.
    pub app_id: String,
}

impl Listed {
    /// A list entry.
    pub fn new(
        key: ToplevelKey,
        identifier: ToplevelIdentifier,
        title: impl Into<String>,
        app_id: impl Into<String>,
    ) -> Self {
        Self {
            key,
            identifier,
            title: title.into(),
            app_id: app_id.into(),
        }
    }
}

/// A decided pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Pairing {
    /// The list-side key.
    pub listed: ToplevelKey,
    /// The management-side key.
    pub managed: ToplevelKey,
}

/// Pairs the not yet paired `listed` entries with `managed` toplevels: equal app id and title
/// first, one to one, earliest first; an entry with no match stays unpaired.
pub fn pair(listed: &[Listed], managed: &[&Toplevel]) -> Vec<Pairing> {
    let _ = (listed, managed);
    todo!("fill: match on (app_id, title) in arrival order, each side used once")
}
