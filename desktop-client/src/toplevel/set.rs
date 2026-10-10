//! The toplevels known now.

use std::collections::BTreeMap;

use super::flags::ToplevelFlag;
use super::model::{Announcement, Toplevel};
use crate::keys::ToplevelKey;
use crate::names::ToplevelIdentifier;

/// Every toplevel, keyed; iteration is in creation order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Toplevels(pub(crate) BTreeMap<ToplevelKey, Toplevel>);

impl Toplevels {
    /// One toplevel.
    pub fn get(&self, key: ToplevelKey) -> Option<&Toplevel> {
        self.0.get(&key)
    }

    /// The announced toplevels, in creation order.
    pub fn announced(&self) -> impl Iterator<Item = &Toplevel> {
        self.0
            .values()
            .filter(|t| t.announcement == Announcement::Announced)
    }

    /// The one with this identifier.
    pub fn by_identifier(&self, identifier: &ToplevelIdentifier) -> Option<&Toplevel> {
        self.announced()
            .find(|t| t.identifier.as_ref() == Some(identifier))
    }

    /// The activated toplevel, if any.
    pub fn activated(&self) -> Option<&Toplevel> {
        self.announced()
            .find(|t| t.flags.contains(ToplevelFlag::Activated))
    }
}
