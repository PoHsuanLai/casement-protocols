//! The life every entity of the Quire arm shares: it appears, changes, completes its burst and
//! ends.

use crate::toplevel::Announcement;
use std::collections::BTreeMap;

/// What happens to one entity (an app, a window, a Space) of key `K` whose properties change by
/// `C`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Lifecycle<K, C> {
    /// It appeared.
    Opened(K),
    /// A property changed.
    Changed {
        /// The entity.
        key: K,
        /// What changed.
        change: C,
    },
    /// Its burst is complete.
    Done(K),
    /// It ended.
    Closed(K),
}

/// An entity the Quire arm keeps: what a [`Lifecycle`] acts on.
pub(super) trait Entity {
    /// Its key.
    type Key: Ord + Copy;
    /// One property changing.
    type Change;

    /// One that has just appeared: nothing known yet.
    fn new(key: Self::Key) -> Self;

    /// Applies `change`.
    fn apply(&mut self, change: Self::Change);

    /// Marks it complete.
    fn announce(&mut self);
}

/// `entities` after `event`. A change to something unknown is ignored (an entity is created by
/// its `Opened`). The key of an entity that ended is returned, for what refers to it to let go.
pub(super) fn reduce_entity<E: Entity>(
    entities: &mut BTreeMap<E::Key, E>,
    event: Lifecycle<E::Key, E::Change>,
) -> Option<E::Key> {
    match event {
        Lifecycle::Opened(key) => {
            entities.entry(key).or_insert_with(|| E::new(key));
            None
        }
        Lifecycle::Changed { key, change } => {
            if let Some(entity) = entities.get_mut(&key) {
                entity.apply(change);
            }
            None
        }
        Lifecycle::Done(key) => {
            if let Some(entity) = entities.get_mut(&key) {
                entity.announce();
            }
            None
        }
        Lifecycle::Closed(key) => {
            entities.remove(&key);
            Some(key)
        }
    }
}

/// An entity's `announcement` set to announced.
pub(super) fn announced(announcement: &mut Announcement) {
    *announcement = Announcement::Announced;
}
