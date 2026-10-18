//! Linking a window object to its ext toplevel handle.
//!
//! `quire_desktop_v1` announces every window as its own object, and `ext-foreign-toplevel-list`
//! announces the same window as a handle; both carry the same identifier string. [`Linker`]
//! matches them whichever arrives first and holds a window's events until its handle is known,
//! so the reducer only ever sees a window under its toplevel's key. It is pure: the wire layer
//! feeds it and emits what it returns.

use std::collections::BTreeMap;

use super::event::QuireEvent;
use super::window::{WindowChange, WindowEvent};
use crate::keys::{ToplevelKey, key};
use crate::names::ToplevelIdentifier;

key! {
    /// The client's number for a window object (its protocol id).
    WindowObject
}

/// An event held back until the window's handle is known.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Held {
    Change(WindowChange),
    Done,
}

/// Where one window object stands.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Standing {
    /// No handle yet: the identifier it announced (if it has), and what it said so far.
    Waiting {
        identifier: Option<ToplevelIdentifier>,
        held: Vec<Held>,
    },
    /// Linked to this toplevel.
    Linked(ToplevelKey),
}

/// Matches window objects with ext handles by identifier.
///
/// ```
/// use desktop_client::quire::{Linker, WindowObject};
/// use desktop_client::{ToplevelIdentifier, ToplevelKey};
///
/// let mut linker = Linker::default();
/// let object = WindowObject::new(9);
/// let id = ToplevelIdentifier::new("abc");
/// assert!(linker.identified(object, id.clone()).is_empty()); // no handle yet
/// let events = linker.listed(ToplevelKey::new(1), id); // the handle arrives
/// assert_eq!(events.len(), 1); // Opened
/// assert_eq!(linker.key_of(object), Some(ToplevelKey::new(1)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Linker {
    listed: BTreeMap<ToplevelIdentifier, ToplevelKey>,
    objects: BTreeMap<WindowObject, Standing>,
}

impl Linker {
    /// The toplevel a window object is linked to.
    pub fn key_of(&self, object: WindowObject) -> Option<ToplevelKey> {
        match self.objects.get(&object)? {
            Standing::Linked(key) => Some(*key),
            Standing::Waiting { .. } => None,
        }
    }

    /// The window object linked to a toplevel.
    pub fn object_of(&self, key: ToplevelKey) -> Option<WindowObject> {
        self.objects
            .iter()
            .find_map(|(object, standing)| match standing {
                Standing::Linked(k) if *k == key => Some(*object),
                _ => None,
            })
    }

    /// An ext handle announced its identifier. Returns the events of a window that was waiting
    /// for it.
    pub fn listed(&mut self, key: ToplevelKey, identifier: ToplevelIdentifier) -> Vec<QuireEvent> {
        self.listed.insert(identifier.clone(), key);
        let waiting = self
            .objects
            .iter()
            .find_map(|(object, standing)| match standing {
                Standing::Waiting {
                    identifier: Some(id),
                    ..
                } if *id == identifier => Some(*object),
                _ => None,
            });
        match waiting {
            Some(object) => self.link(object, key),
            None => Vec::new(),
        }
    }

    /// An ext handle closed: forget its identifier.
    pub fn unlisted(&mut self, key: ToplevelKey) {
        self.listed.retain(|_, k| *k != key);
    }

    /// A window object announced its identifier. Returns its events if the handle is known.
    pub fn identified(
        &mut self,
        object: WindowObject,
        identifier: ToplevelIdentifier,
    ) -> Vec<QuireEvent> {
        if matches!(self.objects.get(&object), Some(Standing::Linked(_))) {
            return Vec::new();
        }
        let key = self.listed.get(&identifier).copied();
        let held = match self.objects.remove(&object) {
            Some(Standing::Waiting { held, .. }) => held,
            _ => Vec::new(),
        };
        self.objects.insert(
            object,
            Standing::Waiting {
                identifier: Some(identifier),
                held,
            },
        );
        match key {
            Some(key) => self.link(object, key),
            None => Vec::new(),
        }
    }

    /// A property of a window object changed.
    pub fn change(&mut self, object: WindowObject, change: WindowChange) -> Vec<QuireEvent> {
        match self.objects.get_mut(&object) {
            Some(Standing::Linked(key)) => {
                vec![window_event(WindowEvent::Changed { key: *key, change })]
            }
            Some(Standing::Waiting { held, .. }) => {
                held.push(Held::Change(change));
                Vec::new()
            }
            None => {
                self.hold(object, Held::Change(change));
                Vec::new()
            }
        }
    }

    /// A window object's burst is complete.
    pub fn done(&mut self, object: WindowObject) -> Vec<QuireEvent> {
        match self.objects.get_mut(&object) {
            Some(Standing::Linked(key)) => vec![window_event(WindowEvent::Done(*key))],
            Some(Standing::Waiting { held, .. }) => {
                held.push(Held::Done);
                Vec::new()
            }
            None => {
                self.hold(object, Held::Done);
                Vec::new()
            }
        }
    }

    /// A window object closed (its last event).
    pub fn closed(&mut self, object: WindowObject) -> Vec<QuireEvent> {
        match self.objects.remove(&object) {
            Some(Standing::Linked(key)) => vec![window_event(WindowEvent::Closed(key))],
            _ => Vec::new(),
        }
    }

    fn hold(&mut self, object: WindowObject, event: Held) {
        self.objects.insert(
            object,
            Standing::Waiting {
                identifier: None,
                held: vec![event],
            },
        );
    }

    /// Links `object` to `key` and returns `Opened` and everything held, in order.
    fn link(&mut self, object: WindowObject, key: ToplevelKey) -> Vec<QuireEvent> {
        let held = match self.objects.insert(object, Standing::Linked(key)) {
            Some(Standing::Waiting { held, .. }) => held,
            _ => Vec::new(),
        };
        let mut events = vec![window_event(WindowEvent::Opened(key))];
        events.extend(held.into_iter().map(|h| {
            window_event(match h {
                Held::Change(change) => WindowEvent::Changed { key, change },
                Held::Done => WindowEvent::Done(key),
            })
        }));
        events
    }
}

fn window_event(event: WindowEvent) -> QuireEvent {
    QuireEvent::Window(event)
}
