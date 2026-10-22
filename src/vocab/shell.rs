//! Shell vocabulary: `quire_shell_v1`'s enums (its modifier bitfield is `MacMods`).

use super::code::{WireCode, wire_enum};
use std::collections::BTreeSet;

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
    /// What the bar does in full screen (`quire_shell_v1.full_screen_bar`, version 7).
    FullScreenBar {
        /// Hidden; it slides in when the pointer rests at the top edge.
        Reveal = 0,
        /// It stays up and the full-screen window is sized below it.
        Keep = 1,
    }
}

wire_enum! {
    /// How much motion the person allows (`quire_shell_v1.motion`, version 7).
    Motion {
        /// Slides and zooms.
        Full = 0,
        /// Cross-fades in place of slides.
        Reduced = 1,
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

wire_enum! {
    /// An entry of a window's menu, in menu order (`quire_shell_v1.window_entry`).
    WindowEntry {
        /// Enter Full Screen.
        EnterFullScreen = 0,
        /// Exit Full Screen.
        ExitFullScreen = 1,
        /// Move and resize: the left half.
        Left = 2,
        /// Move and resize: the right half.
        Right = 3,
        /// Move and resize: the top half.
        Top = 4,
        /// Move and resize: the bottom half.
        Bottom = 5,
        /// Move and resize: the top left quarter.
        TopLeft = 6,
        /// Move and resize: the top right quarter.
        TopRight = 7,
        /// Move and resize: the bottom left quarter.
        BottomLeft = 8,
        /// Move and resize: the bottom right quarter.
        BottomRight = 9,
        /// Move and resize: Return to Previous Size.
        ReturnToPreviousSize = 10,
        /// Full Screen Tile: Left of Screen.
        TileLeftOfScreen = 11,
        /// Full Screen Tile: Right of Screen.
        TileRightOfScreen = 12,
        /// Fill.
        Fill = 13,
        /// Center.
        Centre = 14,
    }
}

/// The entries of a window's menu that can be picked now (`quire_shell_v1.window_menu`'s
/// `enabled`, a bit per [`WindowEntry`] code on the wire).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WindowEntries(BTreeSet<WindowEntry>);

impl WindowEntries {
    /// The entries the wire bit set `bits` offers; a bit no entry has is dropped.
    ///
    /// ```
    /// use casement_protocols::vocab::{WindowEntries, WindowEntry};
    ///
    /// let offered = WindowEntries::from_bits(0b101);
    /// assert!(offered.contains(WindowEntry::EnterFullScreen));
    /// assert!(!offered.contains(WindowEntry::ExitFullScreen));
    /// assert!(offered.contains(WindowEntry::Left));
    /// ```
    pub fn from_bits(bits: u32) -> WindowEntries {
        WindowEntries(
            WindowEntry::ALL
                .iter()
                .copied()
                .filter(|entry| {
                    1_u32
                        .checked_shl(entry.code())
                        .is_some_and(|bit| bits & bit != 0)
                })
                .collect(),
        )
    }

    /// Whether `entry` can be picked.
    pub fn contains(&self, entry: WindowEntry) -> bool {
        self.0.contains(&entry)
    }
}

impl FromIterator<WindowEntry> for WindowEntries {
    fn from_iter<I: IntoIterator<Item = WindowEntry>>(entries: I) -> WindowEntries {
        WindowEntries(entries.into_iter().collect())
    }
}

wire_enum! {
    /// A half of the screen in a Split View (`quire_shell_v1.split_side`).
    SplitSide {
        /// The left half.
        Left = 0,
        /// The right half.
        Right = 1,
    }
}

wire_enum! {
    /// How a window drag in Mission Control ended (`quire_shell_v1.drag_outcome`).
    DragOutcome {
        /// Dropped on a Space of the Spaces bar.
        DroppedOnSpace = 0,
        /// Dropped on the new-desktop target.
        DroppedOnNew = 1,
        /// Dropped on nothing; the window flew back.
        Cancelled = 2,
    }
}
