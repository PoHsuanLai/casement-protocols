//! Apps: `quire_desktop_app_v1`.

use crate::keys::AppKey;
use crate::names::{AppId, AppSubject};
use crate::toplevel::Announcement;
use casement_protocols::vocab::{AppLifeKind, AppVisibility, Attention, MenuSourceKind, ProofKind};

/// Where to read an app's global menu (`quire_desktop_app_v1.menu_source`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MenuSource {
    /// Which menu protocol.
    pub kind: MenuSourceKind,
    /// The D-Bus name.
    pub bus: String,
    /// The app's object path; empty when the source has none.
    pub app_path: String,
    /// The menubar's object path; empty when the source has none.
    pub menubar_path: String,
}

impl MenuSource {
    /// A menu source.
    pub fn new(
        kind: MenuSourceKind,
        bus: impl Into<String>,
        app_path: impl Into<String>,
        menubar_path: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            bus: bus.into(),
            app_path: app_path.into(),
            menubar_path: menubar_path.into(),
        }
    }

    /// No global menu.
    pub fn none() -> Self {
        Self::new(MenuSourceKind::None, "", "", "")
    }
}

/// One running app.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct QuireApp {
    /// This client's key for it.
    pub key: AppKey,
    /// The key the rest of the desktop names it by; `None` before `identity`.
    pub id: Option<AppId>,
    /// How the compositor knows who it is; `None` before `identity`.
    pub proof: Option<ProofKind>,
    /// What the app says about itself (may differ from `id` unless the proof is `Claimed`).
    pub claimed_id: String,
    /// The program behind it as the compositor established it; `None` when it did not say
    /// (a compositor older than protocol version 2, or a program it cannot name).
    pub subject: Option<AppSubject>,
    /// Where it is in its life.
    pub life: AppLifeKind,
    /// Whether its windows are hidden.
    pub visibility: AppVisibility,
    /// Its global menu.
    pub menu: MenuSource,
    /// How much it asks for the person (the dock bounce, the badge).
    pub attention: Attention,
    /// Its place in recent use, 0 the most recent; `None` before the first `mru`.
    pub mru: Option<u32>,
    /// Whether it is complete.
    pub announcement: Announcement,
}

impl QuireApp {
    /// An app that has just appeared: nothing known yet.
    pub fn new(key: AppKey) -> Self {
        Self {
            key,
            id: None,
            proof: None,
            claimed_id: String::new(),
            subject: None,
            life: AppLifeKind::Launching,
            visibility: AppVisibility::Shown,
            menu: MenuSource::none(),
            attention: Attention::None,
            mru: None,
            announcement: Announcement::Pending,
        }
    }
}

/// One property of an app changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AppChange {
    /// Who the app is.
    Identity {
        /// The app key.
        id: AppId,
        /// How it is known.
        proof: ProofKind,
        /// What the app claims.
        claimed_id: String,
    },
    /// The program behind the app, as the compositor established it.
    Subject(AppSubject),
    /// Its life changed.
    Life(AppLifeKind),
    /// It was hidden or shown.
    Visibility(AppVisibility),
    /// Where its menu is.
    Menu(MenuSource),
    /// Its request for attention changed.
    Attention(Attention),
    /// Its place in recent use changed.
    Mru(u32),
}

/// An app event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AppEvent {
    /// An app appeared.
    Opened(AppKey),
    /// A property changed.
    Changed {
        /// The app.
        key: AppKey,
        /// What changed.
        change: AppChange,
    },
    /// The app's burst is complete.
    Done(AppKey),
    /// The app ended.
    Closed(AppKey),
}

/// Applies `change` to `app`.
pub(super) fn apply(app: &mut QuireApp, change: AppChange) {
    match change {
        AppChange::Identity {
            id,
            proof,
            claimed_id,
        } => {
            app.id = Some(id);
            app.proof = Some(proof);
            app.claimed_id = claimed_id;
        }
        AppChange::Subject(subject) => app.subject = Some(subject),
        AppChange::Life(life) => app.life = life,
        AppChange::Visibility(visibility) => app.visibility = visibility,
        AppChange::Menu(menu) => app.menu = menu,
        AppChange::Attention(attention) => app.attention = attention,
        AppChange::Mru(rank) => app.mru = Some(rank),
    }
}
