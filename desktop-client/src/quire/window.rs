//! Windows: `quire_desktop_window_v1`.

use super::lifecycle::{Entity, Lifecycle, announced};
use crate::geometry::Rect;
use crate::keys::{AppKey, SpaceKey, ToplevelKey};
use crate::names::OutputName;
use crate::toplevel::Announcement;
use casement_protocols::vocab::{
    ActorKind, Emphasis, GlowCode, LeaseState, Modality, PlacementMark, Responsiveness,
    SensitivityLevel, WindowRole, WindowStateKind,
};

/// Which Spaces show a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum SpacePlace {
    /// Not reported yet.
    #[default]
    Unknown,
    /// One Space.
    On(SpaceKey),
    /// Every Space.
    AllSpaces,
}

/// A window's rectangle in global logical pixels, and its output's scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct WindowGeometry {
    /// The rectangle.
    pub rect: Rect,
    /// The scale of its output times 120.
    pub scale120: u32,
}

impl WindowGeometry {
    /// A geometry.
    pub const fn new(rect: Rect, scale120: u32) -> Self {
        Self { rect, scale120 }
    }
}

/// An agent's hold on a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Lease {
    /// Whether a lease is held and whether input flows.
    pub state: LeaseState,
    /// The run's glow; `Off` when `state` is `Free`.
    pub glow: GlowCode,
}

impl Lease {
    /// A lease state.
    pub const fn new(state: LeaseState, glow: GlowCode) -> Self {
        Self { state, glow }
    }

    /// No lease.
    pub const fn free() -> Self {
        Self::new(LeaseState::Free, GlowCode::Off)
    }
}

/// What the Quire arm knows of one window; its title and app id are on the toplevel with the
/// same key.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct QuireWindow {
    /// The key of its toplevel.
    pub key: ToplevelKey,
    /// The app it belongs to.
    pub app: Option<AppKey>,
    /// Its role.
    pub role: WindowRole,
    /// How it blocks others.
    pub modality: Modality,
    /// Its state.
    pub state: WindowStateKind,
    /// Who caused the last state change.
    pub actor: ActorKind,
    /// Which Spaces show it.
    pub place: SpacePlace,
    /// The output it is mostly on.
    pub output: Option<OutputName>,
    /// Its rectangle.
    pub geometry: Option<WindowGeometry>,
    /// Its parent window.
    pub parent: Option<ToplevelKey>,
    /// Key, main or inactive.
    pub emphasis: Emphasis,
    /// The strongest sensitivity any of its surfaces declares.
    pub sensitivity: SensitivityLevel,
    /// An agent's hold on it.
    pub lease: Lease,
    /// Whether its client answers pings.
    pub responsiveness: Responsiveness,
    /// What its zoom control shows as current.
    pub placement: PlacementMark,
    /// Its own menu's object path, when it has one.
    pub menu_path: Option<String>,
    /// Whether it is complete.
    pub announcement: Announcement,
}

impl QuireWindow {
    /// A window that has just appeared: nothing known yet.
    pub fn new(key: ToplevelKey) -> Self {
        Self {
            key,
            app: None,
            role: WindowRole::Document,
            modality: Modality::None,
            state: WindowStateKind::Floating,
            actor: ActorKind::Unknown,
            place: SpacePlace::Unknown,
            output: None,
            geometry: None,
            parent: None,
            emphasis: Emphasis::Inactive,
            sensitivity: SensitivityLevel::Normal,
            lease: Lease::free(),
            responsiveness: Responsiveness::Responding,
            placement: PlacementMark::Free,
            menu_path: None,
            announcement: Announcement::Pending,
        }
    }
}

/// One property of a window changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindowChange {
    /// The app it belongs to.
    App(AppKey),
    /// Its role and modality.
    Role {
        /// The role.
        role: WindowRole,
        /// The modality.
        modality: Modality,
    },
    /// Its state.
    State {
        /// The state.
        kind: WindowStateKind,
        /// Who caused it.
        actor: ActorKind,
    },
    /// Which Spaces show it.
    Place(SpacePlace),
    /// Its output.
    Output(OutputName),
    /// Its rectangle.
    Geometry(WindowGeometry),
    /// Its parent, or none.
    Parent(Option<ToplevelKey>),
    /// Its emphasis.
    Emphasis(Emphasis),
    /// Its sensitivity.
    Sensitivity(SensitivityLevel),
    /// An agent's hold.
    Lease(Lease),
    /// Whether its client answers.
    Responsiveness(Responsiveness),
    /// Its placement mark.
    Placement(PlacementMark),
    /// Its own menu's path; empty for the app's.
    MenuPath(String),
}

/// A window event (`Opened` is the window object being linked to its toplevel).
pub type WindowEvent = Lifecycle<ToplevelKey, WindowChange>;

impl Entity for QuireWindow {
    type Key = ToplevelKey;
    type Change = WindowChange;

    fn new(key: ToplevelKey) -> Self {
        QuireWindow::new(key)
    }

    fn apply(&mut self, change: WindowChange) {
        match change {
            WindowChange::App(app) => self.app = Some(app),
            WindowChange::Role { role, modality } => {
                self.role = role;
                self.modality = modality;
            }
            WindowChange::State { kind, actor } => {
                self.state = kind;
                self.actor = actor;
            }
            WindowChange::Place(place) => self.place = place,
            WindowChange::Output(name) => self.output = Some(name),
            WindowChange::Geometry(geometry) => self.geometry = Some(geometry),
            WindowChange::Parent(parent) => self.parent = parent,
            WindowChange::Emphasis(emphasis) => self.emphasis = emphasis,
            WindowChange::Sensitivity(level) => self.sensitivity = level,
            WindowChange::Lease(lease) => self.lease = lease,
            WindowChange::Responsiveness(state) => self.responsiveness = state,
            WindowChange::Placement(mark) => self.placement = mark,
            WindowChange::MenuPath(path) => {
                self.menu_path = Some(path).filter(|p| !p.is_empty());
            }
        }
    }

    fn announce(&mut self) {
        announced(&mut self.announcement);
    }
}
