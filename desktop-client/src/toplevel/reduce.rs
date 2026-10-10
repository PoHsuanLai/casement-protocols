//! The toplevel transition.

use super::event::{ToplevelChange, ToplevelEvent};
use super::model::{Announcement, Toplevel};
use super::set::Toplevels;
use crate::keys::ToplevelKey;

/// `toplevels` after `event`. A change to an unknown key is ignored; `Merged` keeps the
/// managed record's state and takes the identifier.
pub(crate) fn reduce_toplevels(mut toplevels: Toplevels, event: ToplevelEvent) -> Toplevels {
    match event {
        ToplevelEvent::Opened(key) => {
            toplevels.0.entry(key).or_insert_with(|| Toplevel::new(key));
        }
        ToplevelEvent::Changed { key, change } => {
            if let Some(toplevel) = toplevels.0.get_mut(&key) {
                apply(toplevel, change);
            }
        }
        ToplevelEvent::Done(key) => {
            if let Some(toplevel) = toplevels.0.get_mut(&key) {
                toplevel.announcement = Announcement::Announced;
            }
        }
        ToplevelEvent::Merged { from, into } => merge(&mut toplevels, from, into),
        ToplevelEvent::Closed(key) => {
            toplevels.0.remove(&key);
            forget_parent(&mut toplevels, key, None);
        }
    }
    toplevels
}

fn apply(toplevel: &mut Toplevel, change: ToplevelChange) {
    match change {
        ToplevelChange::Identifier(identifier) => toplevel.identifier = Some(identifier),
        ToplevelChange::Title(title) => toplevel.title = title,
        ToplevelChange::AppId(app_id) => toplevel.app_id = app_id,
        ToplevelChange::OutputEntered(name) => {
            if !toplevel.outputs.contains(&name) {
                toplevel.outputs.push(name);
            }
        }
        ToplevelChange::OutputLeft(name) => toplevel.outputs.retain(|o| o != &name),
        ToplevelChange::Flags(flags) => toplevel.flags = flags,
        ToplevelChange::Parent(parent) => toplevel.parent = parent,
    }
}

/// `from` and `into` are one window. Nothing changes unless both exist.
fn merge(toplevels: &mut Toplevels, from: ToplevelKey, into: ToplevelKey) {
    if from == into || !toplevels.0.contains_key(&into) {
        return;
    }
    let Some(listed) = toplevels.0.remove(&from) else {
        return;
    };
    if let Some(target) = toplevels.0.get_mut(&into) {
        target.identifier = listed.identifier;
    }
    forget_parent(toplevels, from, Some(into));
}

/// Points every parent link at `old` to `new` (or nowhere).
fn forget_parent(toplevels: &mut Toplevels, old: ToplevelKey, new: Option<ToplevelKey>) {
    for toplevel in toplevels.0.values_mut() {
        if toplevel.parent == Some(old) {
            toplevel.parent = new;
        }
    }
}
