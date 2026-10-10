//! The Quire arm's picture.

use std::collections::BTreeMap;

use super::app::QuireApp;
use super::shell::Shell;
use super::space::QuireSpace;
use super::window::QuireWindow;
use crate::keys::{AppKey, SpaceKey, ToplevelKey};
use crate::names::{AppId, OutputName, SpaceId};
use crate::toplevel::Announcement;
use casement_protocols::vocab::ActorKind;

/// Which app is active and which window is key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Active {
    /// The active app, if any.
    pub app: Option<AppKey>,
    /// The key window, if the active app has one.
    pub key: Option<ToplevelKey>,
    /// Who caused the last change.
    pub actor: ActorKind,
}

impl Default for Active {
    fn default() -> Self {
        Self {
            app: None,
            key: None,
            actor: ActorKind::Unknown,
        }
    }
}

/// Apps, windows, Spaces, the active pair and the shell's state.
///
/// Apps and Spaces are listed once announced (their first `done`); a window is reached by the
/// key of its toplevel.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Quire {
    pub(crate) apps: BTreeMap<AppKey, QuireApp>,
    pub(crate) windows: BTreeMap<ToplevelKey, QuireWindow>,
    pub(crate) spaces: BTreeMap<SpaceKey, QuireSpace>,
    pub(crate) active: Active,
    pub(crate) shell: Shell,
}

impl Quire {
    /// The announced apps, in creation order.
    pub fn apps(&self) -> impl Iterator<Item = &QuireApp> {
        self.apps
            .values()
            .filter(|a| a.announcement == Announcement::Announced)
    }

    /// One app, announced or not.
    pub fn app(&self, key: AppKey) -> Option<&QuireApp> {
        self.apps.get(&key)
    }

    /// The announced app with this key string.
    pub fn app_by_id(&self, id: &AppId) -> Option<&QuireApp> {
        self.apps().find(|a| a.id.as_ref() == Some(id))
    }

    /// The announced windows, in toplevel key order.
    pub fn windows(&self) -> impl Iterator<Item = &QuireWindow> {
        self.windows
            .values()
            .filter(|w| w.announcement == Announcement::Announced)
    }

    /// The window of a toplevel, announced or not.
    pub fn window(&self, key: ToplevelKey) -> Option<&QuireWindow> {
        self.windows.get(&key)
    }

    /// The announced windows of one app.
    pub fn windows_of(&self, app: AppKey) -> impl Iterator<Item = &QuireWindow> {
        self.windows().filter(move |w| w.app == Some(app))
    }

    /// The announced Spaces, in creation order.
    pub fn spaces(&self) -> impl Iterator<Item = &QuireSpace> {
        self.spaces
            .values()
            .filter(|s| s.announcement == Announcement::Announced)
    }

    /// One Space, announced or not.
    pub fn space(&self, key: SpaceKey) -> Option<&QuireSpace> {
        self.spaces.get(&key)
    }

    /// The announced Space with this id.
    pub fn space_by_id(&self, id: &SpaceId) -> Option<&QuireSpace> {
        self.spaces().find(|s| s.id.as_ref() == Some(id))
    }

    /// The announced Spaces of one output, left to right.
    pub fn spaces_on(&self, output: &OutputName) -> Vec<&QuireSpace> {
        let mut spaces: Vec<_> = self
            .spaces()
            .filter(|s| s.output.as_ref() == Some(output))
            .collect();
        spaces.sort_by_key(|s| s.position);
        spaces
    }

    /// The active app and key window.
    pub fn active(&self) -> &Active {
        &self.active
    }

    /// The shell's state.
    pub fn shell(&self) -> &Shell {
        &self.shell
    }

    /// Drops a window and, if it was key, the key.
    pub(crate) fn forget_window(&mut self, key: ToplevelKey) {
        self.windows.remove(&key);
        if self.active.key == Some(key) {
            self.active.key = None;
        }
        for window in self.windows.values_mut() {
            if window.parent == Some(key) {
                window.parent = None;
            }
        }
    }
}
