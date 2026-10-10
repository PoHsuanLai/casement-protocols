//! The Quire transition.

use super::app::{self, AppEvent, QuireApp};
use super::event::QuireEvent;
use super::set::{Active, Quire};
use super::shell::reduce_shell;
use super::space::{self, QuireSpace, SpaceEvent};
use super::window::{self, QuireWindow, WindowEvent};
use crate::toplevel::Announcement;

/// `quire` after `event`. Pure and total: a change to something unknown is ignored (a window
/// is created by its `Opened`), and a `Done` marks its object announced.
pub(crate) fn reduce_quire(mut quire: Quire, event: QuireEvent) -> Quire {
    match event {
        QuireEvent::App(e) => reduce_app(&mut quire, e),
        QuireEvent::Window(e) => reduce_window(&mut quire, e),
        QuireEvent::Space(e) => reduce_space(&mut quire, e),
        QuireEvent::Active { app, key, actor } => quire.active = Active { app, key, actor },
        QuireEvent::Shell(e) => quire.shell = reduce_shell(quire.shell, e),
    }
    quire
}

fn reduce_app(quire: &mut Quire, event: AppEvent) {
    match event {
        AppEvent::Opened(key) => {
            quire.apps.entry(key).or_insert_with(|| QuireApp::new(key));
        }
        AppEvent::Changed { key, change } => {
            if let Some(a) = quire.apps.get_mut(&key) {
                app::apply(a, change);
            }
        }
        AppEvent::Done(key) => {
            if let Some(a) = quire.apps.get_mut(&key) {
                a.announcement = Announcement::Announced;
            }
        }
        AppEvent::Closed(key) => {
            quire.apps.remove(&key);
            if quire.active.app == Some(key) {
                quire.active.app = None;
            }
            for w in quire.windows.values_mut() {
                if w.app == Some(key) {
                    w.app = None;
                }
            }
        }
    }
}

fn reduce_window(quire: &mut Quire, event: WindowEvent) {
    match event {
        WindowEvent::Opened(key) => {
            quire
                .windows
                .entry(key)
                .or_insert_with(|| QuireWindow::new(key));
        }
        WindowEvent::Changed { key, change } => {
            if let Some(w) = quire.windows.get_mut(&key) {
                window::apply(w, change);
            }
        }
        WindowEvent::Done(key) => {
            if let Some(w) = quire.windows.get_mut(&key) {
                w.announcement = Announcement::Announced;
            }
        }
        WindowEvent::Closed(key) => quire.forget_window(key),
    }
}

fn reduce_space(quire: &mut Quire, event: SpaceEvent) {
    match event {
        SpaceEvent::Opened(key) => {
            quire
                .spaces
                .entry(key)
                .or_insert_with(|| QuireSpace::new(key));
        }
        SpaceEvent::Changed { key, change } => {
            if let Some(s) = quire.spaces.get_mut(&key) {
                space::apply(s, change);
            }
        }
        SpaceEvent::Done(key) => {
            if let Some(s) = quire.spaces.get_mut(&key) {
                s.announcement = Announcement::Announced;
            }
        }
        SpaceEvent::Removed(key) => {
            quire.spaces.remove(&key);
            for w in quire.windows.values_mut() {
                if w.place == super::window::SpacePlace::On(key) {
                    w.place = super::window::SpacePlace::Unknown;
                }
            }
        }
    }
}
