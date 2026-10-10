//! The heads known now.

use std::collections::BTreeMap;

use super::model::Head;
use crate::keys::HeadKey;
use crate::names::OutputName;

/// Every head, with the serial of the last `done`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Outputs {
    pub(crate) heads: BTreeMap<HeadKey, Head>,
    pub(crate) serial: Option<u32>,
}

impl Outputs {
    /// The heads, in creation order.
    pub fn heads(&self) -> impl Iterator<Item = &Head> {
        self.heads.values()
    }

    /// One head.
    pub fn get(&self, key: HeadKey) -> Option<&Head> {
        self.heads.get(&key)
    }

    /// The head named `name`.
    pub fn by_name(&self, name: &OutputName) -> Option<&Head> {
        self.heads.values().find(|h| &h.name == name)
    }

    /// The serial of the last `done`; a configuration quotes it. `None` before the first.
    pub fn serial(&self) -> Option<u32> {
        self.serial
    }
}
