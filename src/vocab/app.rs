//! App vocabulary: `quire_desktop_app_v1`'s enums.

use super::code::wire_enum;

wire_enum! {
    /// Where an app is in its life (`quire_desktop_app_v1.app_life_kind`).
    AppLifeKind {
        /// Started, no window yet.
        Launching = 0,
        /// Running with windows.
        Running = 1,
        /// Running with no window.
        Windowless = 2,
        /// Asked to quit, still running.
        Quitting = 3,
        /// Not answering pings.
        NotResponding = 4,
    }
}

wire_enum! {
    /// Whether an app's windows are hidden (`quire_desktop_app_v1.app_visibility`).
    AppVisibility {
        /// Its windows are shown.
        Shown = 0,
        /// The app is hidden.
        Hidden = 1,
    }
}

wire_enum! {
    /// How much an app asks for the person (`quire_desktop_app_v1.attention`).
    Attention {
        /// Nothing.
        None = 0,
        /// Once.
        Informational = 1,
        /// Until the person responds.
        Critical = 2,
    }
}

wire_enum! {
    /// How the compositor knows who an app is (`quire_desktop_app_v1.proof_kind`).
    ProofKind {
        /// The app's systemd scope, made by our launcher.
        Scope = 0,
        /// The sandbox's `wp_security_context_v1` app id.
        SecurityContext = 1,
        /// Only what the app says about itself.
        Claimed = 2,
    }
}

wire_enum! {
    /// Where an app's global menu comes from (`quire_desktop_app_v1.menu_source_kind`).
    MenuSourceKind {
        /// No global menu.
        None = 0,
        /// `com.canonical.dbusmenu`.
        DbusMenu = 1,
        /// `org.gtk.Menus`.
        GtkMenus = 2,
    }
}
