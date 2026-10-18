//! The Quire transition.

use super::app::AppEvent;
use super::event::QuireEvent;
use super::lifecycle::reduce_entity;
use super::set::{Active, Quire};
use super::shell::reduce_shell;
use super::space::SpaceEvent;
use super::window::{SpacePlace, WindowEvent};

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
    if let Some(key) = reduce_entity(&mut quire.apps, event) {
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

fn reduce_window(quire: &mut Quire, event: WindowEvent) {
    if let Some(key) = reduce_entity(&mut quire.windows, event) {
        quire.forget_window(key);
    }
}

fn reduce_space(quire: &mut Quire, event: SpaceEvent) {
    if let Some(key) = reduce_entity(&mut quire.spaces, event) {
        for w in quire.windows.values_mut() {
            if w.place == SpacePlace::On(key) {
                w.place = SpacePlace::Unknown;
            }
        }
    }
}
