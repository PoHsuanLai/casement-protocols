//! Pairing a list entry with its management handle.
//!
//! Neither protocol names the other's object, so the pairing is a heuristic over what both
//! describe: app id and title, in arrival order (FINDINGS). It is pure so it is testable.

use std::collections::BTreeSet;

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
///
/// Both slices are in arrival order and hold only entries not paired yet. Identical windows
/// (same app id and title) pair first-to-first, second-to-second, so the result is
/// deterministic but can swap two windows opened together (FINDINGS).
///
/// ```
/// use desktop_client::toplevel::{Listed, Toplevel, pair};
/// use desktop_client::{ToplevelIdentifier, ToplevelKey};
///
/// let listed = [Listed::new(ToplevelKey::new(1), ToplevelIdentifier::new("a"), "T", "app")];
/// let mut managed = Toplevel::new(ToplevelKey::new(2));
/// managed.title = "T".into();
/// managed.app_id = "app".into();
/// let pairs = pair(&listed, &[&managed]);
/// assert_eq!(pairs.len(), 1);
/// assert_eq!(pairs[0].managed, ToplevelKey::new(2));
/// ```
pub fn pair(listed: &[Listed], managed: &[&Toplevel]) -> Vec<Pairing> {
    let mut taken: BTreeSet<ToplevelKey> = BTreeSet::new();
    let mut pairs = Vec::new();
    for entry in listed {
        let found = managed.iter().find(|m| {
            !taken.contains(&m.key) && m.app_id == entry.app_id && m.title == entry.title
        });
        if let Some(m) = found {
            taken.insert(m.key);
            pairs.push(Pairing {
                listed: entry.key,
                managed: m.key,
            });
        }
    }
    pairs
}
