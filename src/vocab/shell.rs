//! Shell vocabulary: `quire_shell_v1`'s enums (its modifier bitfield is `MacMods`).

use super::code::wire_enum;

wire_enum! {
    /// What a shell surface is (`quire_shell_v1.shell_role`).
    ShellRole {
        /// The menu bar.
        Bar = 0,
        /// The dock.
        Dock = 1,
        /// The launcher.
        Launcher = 2,
        /// The app switcher.
        Switcher = 3,
        /// The overview's chrome.
        Overview = 4,
        /// Notification banners and centre.
        Notifications = 5,
        /// On-screen displays.
        Osd = 6,
        /// The control centre.
        ControlCentre = 7,
        /// The desktop picture.
        Wallpaper = 8,
        /// Desktop widgets.
        Widget = 9,
    }
}

wire_enum! {
    /// A modifier gesture the compositor recognised (`quire_shell_v1.key_gesture`).
    KeyGesture {
        /// A double tap of Command.
        Summon = 0,
        /// Command held alone past the hold time.
        HoldStart = 1,
        /// The held Command was released.
        HoldEnd = 2,
        /// The hold became a chord.
        HoldCancel = 3,
    }
}

wire_enum! {
    /// A screen edge (`quire_shell_v1.screen_edge`).
    ScreenEdge {
        /// Top.
        Top = 0,
        /// Bottom.
        Bottom = 1,
        /// Left.
        Left = 2,
        /// Right.
        Right = 3,
    }
}

wire_enum! {
    /// A screen corner (`quire_shell_v1.corner`).
    Corner {
        /// Top left.
        TopLeft = 0,
        /// Top right.
        TopRight = 1,
        /// Bottom left.
        BottomLeft = 2,
        /// Bottom right.
        BottomRight = 3,
    }
}

wire_enum! {
    /// Whether something is on screen (`quire_shell_v1.shown`).
    Shown {
        /// Hidden.
        Hidden = 0,
        /// Shown.
        Shown = 1,
    }
}

wire_enum! {
    /// Which overview (`quire_shell_v1.overview_mode`).
    OverviewMode {
        /// Every window of every app.
        MissionControl = 0,
        /// The windows of the active app.
        AppExpose = 1,
        /// Every window moved aside.
        Desktop = 2,
    }
}

wire_enum! {
    /// Where the overview is in its life (`quire_shell_v1.overview_state`).
    OverviewState {
        /// Not shown.
        Closed = 0,
        /// Animating in.
        Entering = 1,
        /// Shown.
        Open = 2,
        /// Animating out.
        Leaving = 3,
        /// Following a gesture.
        Tracking = 4,
    }
}
