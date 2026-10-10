//! The shell's notes: `quire_shell_v1` events.
//!
//! Most are moments (a hot corner, a bell, the kill chord) and change no state; the modifiers,
//! the switcher, the overview layout and the reveals are state the shell draws from.

use std::collections::BTreeMap;

use crate::geometry::Rect;
use crate::keys::SurfaceKey;
use crate::names::{AppId, SpaceId, ToplevelIdentifier};
use casement_protocols::vocab::{
    Corner, KeyGesture, MacMods, OverviewMode, OverviewState, ScreenEdge, Shown,
};

/// One event of `quire_shell_v1`.
///
/// Apps and windows are named by string (an app key, an ext-foreign-toplevel identifier),
/// because the protocol is bound on a different connection from the toplevel handles.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShellEvent {
    /// The modifiers held over a shell surface, on entry and on every change. The surface is the
    /// caller's registered key, `None` for a surface it never registered.
    Modifiers {
        /// The shell surface under the pointer.
        surface: Option<SurfaceKey>,
        /// The keys held, before any per-app remapping.
        mods: MacMods,
    },
    /// A modifier gesture was recognised (summon, hold start, hold end, hold cancel).
    KeyGesture(KeyGesture),
    /// One app of the switcher, in order, before `SwitcherShown`.
    SwitcherApp {
        /// The position the compositor gave; `SwitcherPick` returns it.
        index: u32,
        /// The app.
        app: AppId,
    },
    /// The switcher appeared with this app selected.
    SwitcherShown {
        /// The selected index.
        selected: u32,
    },
    /// The selection moved.
    SwitcherSelected(u32),
    /// The switcher went away.
    SwitcherHidden,
    /// The overview's state and progress (thousandths).
    Overview {
        /// Which overview.
        mode: OverviewMode,
        /// Where it is in its life.
        state: OverviewState,
        /// 0 closed, 1000 open.
        progress: u32,
    },
    /// Where a window sits in the overview; complete at `OverviewDone`.
    OverviewSlot {
        /// The window's ext-foreign-toplevel identifier.
        window: ToplevelIdentifier,
        /// Its slot, in global logical pixels.
        rect: Rect,
    },
    /// Where a Space thumbnail sits in the overview; complete at `OverviewDone`.
    OverviewSpace {
        /// The Space.
        space: SpaceId,
        /// Its thumbnail, in global logical pixels.
        rect: Rect,
    },
    /// The overview layout is complete.
    OverviewDone,
    /// The pointer triggered a hot corner (a moment).
    HotCorner(Corner),
    /// A full-screen edge reveal changed.
    Reveal {
        /// The edge.
        edge: ScreenEdge,
        /// Slid in or away.
        shown: Shown,
    },
    /// A key equivalent ran a menu item; flash the menu title (a moment).
    MenuFlash {
        /// The app.
        app: AppId,
        /// The index of the top-level menu.
        item: u32,
    },
    /// An app rang the bell (a moment).
    Bell(AppId),
    /// An app was activated with no windows; the shell decides what reopening means (a moment).
    Reopen(AppId),
    /// The kill chord was pressed; every agent lease has ended (a moment).
    Killed,
}

/// The switcher: the apps in order and the selection while it is shown.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Switcher {
    pub(crate) apps: Vec<AppId>,
    pub(crate) selected: Option<u32>,
}

impl Switcher {
    /// The apps, in the compositor's order.
    pub fn apps(&self) -> &[AppId] {
        &self.apps
    }

    /// The selected index while the switcher is shown.
    pub fn selected(&self) -> Option<u32> {
        self.selected
    }

    /// Whether the switcher is on screen.
    pub fn is_shown(&self) -> bool {
        self.selected.is_some()
    }
}

/// A window's slot in the overview.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct OverviewSlot {
    /// The window's identifier.
    pub window: ToplevelIdentifier,
    /// Its slot.
    pub rect: Rect,
}

/// A Space thumbnail's slot in the overview.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct OverviewSpaceSlot {
    /// The Space.
    pub space: SpaceId,
    /// Its thumbnail.
    pub rect: Rect,
}

/// The overview: its state, and the last complete layout (a batch is applied at
/// `OverviewDone`, so a reader never sees half a layout).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overview {
    pub(crate) mode: OverviewMode,
    pub(crate) state: OverviewState,
    pub(crate) progress: u32,
    pub(crate) slots: Vec<OverviewSlot>,
    pub(crate) spaces: Vec<OverviewSpaceSlot>,
    pub(crate) pending_slots: Vec<OverviewSlot>,
    pub(crate) pending_spaces: Vec<OverviewSpaceSlot>,
}

impl Default for Overview {
    fn default() -> Self {
        Self {
            mode: OverviewMode::MissionControl,
            state: OverviewState::Closed,
            progress: 0,
            slots: Vec::new(),
            spaces: Vec::new(),
            pending_slots: Vec::new(),
            pending_spaces: Vec::new(),
        }
    }
}

impl Overview {
    /// Which overview the last state event named.
    pub fn mode(&self) -> OverviewMode {
        self.mode
    }

    /// Where the overview is in its life.
    pub fn state(&self) -> OverviewState {
        self.state
    }

    /// Progress in thousandths: 0 closed, 1000 open.
    pub fn progress(&self) -> u32 {
        self.progress
    }

    /// The window slots of the last complete layout.
    pub fn slots(&self) -> &[OverviewSlot] {
        &self.slots
    }

    /// The Space thumbnails of the last complete layout.
    pub fn spaces(&self) -> &[OverviewSpaceSlot] {
        &self.spaces
    }
}

/// Which edges have their bar or dock revealed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Reveals(pub(crate) BTreeMap<ScreenEdge, Shown>);

impl Reveals {
    /// Whether `edge` is revealed; hidden until the compositor says otherwise.
    pub fn at(&self, edge: ScreenEdge) -> Shown {
        self.0.get(&edge).copied().unwrap_or(Shown::Hidden)
    }
}

/// The shell's state.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Shell {
    /// The modifiers held over a shell surface at the last report.
    pub mods: MacMods,
    /// The surface that report was for.
    pub mods_surface: Option<SurfaceKey>,
    /// The app switcher.
    pub switcher: Switcher,
    /// The overview.
    pub overview: Overview,
    /// The edge reveals.
    pub reveals: Reveals,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            mods: MacMods::NONE,
            mods_surface: None,
            switcher: Switcher::default(),
            overview: Overview::default(),
            reveals: Reveals::default(),
        }
    }
}

/// `shell` after `event`. Moments change nothing.
pub(super) fn reduce_shell(mut shell: Shell, event: ShellEvent) -> Shell {
    match event {
        ShellEvent::Modifiers { surface, mods } => {
            shell.mods = mods;
            shell.mods_surface = surface;
        }
        ShellEvent::SwitcherApp { index, app } => {
            shell.switcher.apps.truncate(index as usize);
            shell.switcher.apps.push(app);
        }
        ShellEvent::SwitcherShown { selected } | ShellEvent::SwitcherSelected(selected) => {
            shell.switcher.selected = Some(selected);
        }
        ShellEvent::SwitcherHidden => shell.switcher = Switcher::default(),
        ShellEvent::Overview {
            mode,
            state,
            progress,
        } => {
            shell.overview.mode = mode;
            shell.overview.state = state;
            shell.overview.progress = progress;
        }
        ShellEvent::OverviewSlot { window, rect } => {
            shell
                .overview
                .pending_slots
                .push(OverviewSlot { window, rect });
        }
        ShellEvent::OverviewSpace { space, rect } => {
            shell
                .overview
                .pending_spaces
                .push(OverviewSpaceSlot { space, rect });
        }
        ShellEvent::OverviewDone => {
            shell.overview.slots = std::mem::take(&mut shell.overview.pending_slots);
            shell.overview.spaces = std::mem::take(&mut shell.overview.pending_spaces);
        }
        ShellEvent::Reveal { edge, shown } => {
            shell.reveals.0.insert(edge, shown);
        }
        ShellEvent::KeyGesture(_)
        | ShellEvent::HotCorner(_)
        | ShellEvent::MenuFlash { .. }
        | ShellEvent::Bell(_)
        | ShellEvent::Reopen(_)
        | ShellEvent::Killed => {}
    }
    shell
}
