//! [`ShellVerb`], its command line and the way back.

/// One thing the compositor asks the shell to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellVerb {
    /// Lock the screen.
    Lock,
    /// Raise the front app's next window.
    NextWindow,
    /// Flip the dock's auto-hide.
    DockToggleHide,
    /// Show the launcher, or hide it when it is up.
    LauncherToggle,
    /// Show the power menu. Open, not toggle: like the Mac it closes by Escape or a button.
    PowerMenuOpen,
    /// Show the Force Quit panel, or hide it when it is up.
    ForceQuitToggle,
    /// Capture the whole screen.
    ScreenshotScreen,
    /// Capture a chosen area.
    ScreenshotArea,
    /// Capture a chosen window.
    ScreenshotWindow,
    /// The app switcher steps to the next app.
    SwitcherNext,
    /// The app switcher steps to the previous app.
    SwitcherPrev,
    /// The app switcher picks the app it is on.
    SwitcherCommit,
    /// The app switcher closes without picking.
    SwitcherCancel,
    /// Show the control centre, or hide it when it is up.
    ControlCenterToggle,
    /// Show the notification centre, or hide it when it is up.
    NotificationCenterToggle,
}

/// Every verb, in the order the table above names them.
pub const ALL: &[ShellVerb] = &[
    ShellVerb::Lock,
    ShellVerb::NextWindow,
    ShellVerb::DockToggleHide,
    ShellVerb::LauncherToggle,
    ShellVerb::PowerMenuOpen,
    ShellVerb::ForceQuitToggle,
    ShellVerb::ScreenshotScreen,
    ShellVerb::ScreenshotArea,
    ShellVerb::ScreenshotWindow,
    ShellVerb::SwitcherNext,
    ShellVerb::SwitcherPrev,
    ShellVerb::SwitcherCommit,
    ShellVerb::SwitcherCancel,
    ShellVerb::ControlCenterToggle,
    ShellVerb::NotificationCenterToggle,
];

impl ShellVerb {
    /// The words after the shell's program name that spell this verb on its command line.
    pub fn argv(self) -> &'static [&'static str] {
        match self {
            Self::Lock => &["lock"],
            Self::NextWindow => &["next-window"],
            Self::DockToggleHide => &["dock", "toggle-hide"],
            Self::LauncherToggle => &["launcher", "toggle"],
            Self::PowerMenuOpen => &["power-menu", "open"],
            Self::ForceQuitToggle => &["force-quit", "toggle"],
            Self::ScreenshotScreen => &["screenshot", "screen"],
            Self::ScreenshotArea => &["screenshot", "area"],
            Self::ScreenshotWindow => &["screenshot", "window"],
            Self::SwitcherNext => &["switcher", "next"],
            Self::SwitcherPrev => &["switcher", "prev"],
            Self::SwitcherCommit => &["switcher", "commit"],
            Self::SwitcherCancel => &["switcher", "cancel"],
            Self::ControlCenterToggle => &["control-center", "toggle"],
            Self::NotificationCenterToggle => &["notifications", "toggle"],
        }
    }

    /// The verb `args` spell, the exact inverse of [`argv`](Self::argv): any other words, or
    /// extra words after a verb's own, are `None`.
    pub fn parse(args: &[&str]) -> Option<Self> {
        ALL.iter().copied().find(|verb| verb.argv() == args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A match with no wildcard: a new variant fails to compile here until `ALL` is revisited.
    fn position(verb: ShellVerb) -> usize {
        match verb {
            ShellVerb::Lock => 0,
            ShellVerb::NextWindow => 1,
            ShellVerb::DockToggleHide => 2,
            ShellVerb::LauncherToggle => 3,
            ShellVerb::PowerMenuOpen => 4,
            ShellVerb::ForceQuitToggle => 5,
            ShellVerb::ScreenshotScreen => 6,
            ShellVerb::ScreenshotArea => 7,
            ShellVerb::ScreenshotWindow => 8,
            ShellVerb::SwitcherNext => 9,
            ShellVerb::SwitcherPrev => 10,
            ShellVerb::SwitcherCommit => 11,
            ShellVerb::SwitcherCancel => 12,
            ShellVerb::ControlCenterToggle => 13,
            ShellVerb::NotificationCenterToggle => 14,
        }
    }

    #[test]
    fn all_names_every_variant_once_in_order() {
        let positions: Vec<usize> = ALL.iter().copied().map(position).collect();
        assert_eq!(positions, (0..positions.len()).collect::<Vec<_>>());
        // The match above has one arm per variant, so the last position is the variant count.
        assert_eq!(position(ShellVerb::NotificationCenterToggle) + 1, ALL.len());
    }

    #[test]
    fn every_verb_round_trips_through_its_argv() {
        for verb in ALL {
            assert_eq!(ShellVerb::parse(verb.argv()), Some(*verb), "{verb:?}");
        }
    }

    #[test]
    fn no_two_verbs_share_an_argv() {
        let mut lines: Vec<&[&str]> = ALL.iter().map(|verb| verb.argv()).collect();
        lines.sort();
        let before = lines.len();
        lines.dedup();
        assert_eq!(lines.len(), before);
    }

    #[test]
    fn unknown_missing_or_extra_words_are_no_verb() {
        let cases: &[&[&str]] = &[
            &[],
            &["dock"],
            &["launcher", "open"],
            &["lock", "now"],
            &["screenshot", "screen", "extra"],
            &["Lock"],
        ];
        for args in cases {
            assert_eq!(ShellVerb::parse(args), None, "{args:?}");
        }
    }
}
