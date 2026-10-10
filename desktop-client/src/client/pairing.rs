//! Which toplevel handles wait for their partner.

use std::collections::BTreeMap;

use crate::event::DesktopEvent;
use crate::keys::ToplevelKey;
use crate::state::DesktopState;
use crate::toplevel::{Listed, Pairing, ToplevelEvent, pair};

/// The pairing book: list and management handles that are complete but not yet matched, and the
/// matches made.
#[derive(Debug, Default)]
pub(super) struct Book {
    /// List-side keys whose `done` arrived, in arrival order.
    listed: Vec<ToplevelKey>,
    /// Management-side keys whose first `done` arrived, in arrival order.
    managed: Vec<ToplevelKey>,
    /// List-side key to the management-side key it was merged into.
    merged: BTreeMap<ToplevelKey, ToplevelKey>,
}

impl Book {
    /// A list handle is complete and waits for a partner.
    pub(super) fn add_listed(&mut self, key: ToplevelKey) {
        if !self.listed.contains(&key) {
            self.listed.push(key);
        }
    }

    /// A management handle is complete and waits for a partner, once.
    pub(super) fn add_managed(&mut self, key: ToplevelKey) {
        let known = self.managed.contains(&key) || self.merged.values().any(|m| *m == key);
        if !known {
            self.managed.push(key);
        }
    }

    /// Whether `key` (a list handle) has been merged into a management record.
    pub(super) fn is_merged(&self, key: ToplevelKey) -> bool {
        self.merged.contains_key(&key)
    }

    /// Forgets a list handle (it closed unmerged).
    pub(super) fn drop_listed(&mut self, key: ToplevelKey) {
        self.listed.retain(|k| *k != key);
    }

    /// Forgets a management handle; returns the list handle merged into it, if any.
    pub(super) fn drop_managed(&mut self, key: ToplevelKey) -> Option<ToplevelKey> {
        self.managed.retain(|k| *k != key);
        let listed = self
            .merged
            .iter()
            .find_map(|(l, m)| (*m == key).then_some(*l))?;
        self.merged.remove(&listed);
        Some(listed)
    }

    /// Matches what waits, with [`pair`], and returns the merge events to emit.
    pub(super) fn pair_up(&mut self, state: &DesktopState) -> Vec<DesktopEvent> {
        let listed: Vec<Listed> = self
            .listed
            .iter()
            .filter_map(|key| {
                let t = state.toplevels.get(*key)?;
                Some(Listed::new(
                    *key,
                    t.identifier.clone()?,
                    t.title.clone(),
                    t.app_id.clone(),
                ))
            })
            .collect();
        let managed: Vec<_> = self
            .managed
            .iter()
            .filter_map(|key| state.toplevels.get(*key))
            .collect();
        let pairs = pair(&listed, &managed);
        pairs
            .into_iter()
            .map(|Pairing { listed, managed }| {
                self.listed.retain(|k| *k != listed);
                self.managed.retain(|k| *k != managed);
                self.merged.insert(listed, managed);
                DesktopEvent::Toplevel(ToplevelEvent::Merged {
                    from: listed,
                    into: managed,
                })
            })
            .collect()
    }
}
