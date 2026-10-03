//! The modifier keys the person holds, as pressed (`quire_shell_v1.mac_mods`, a bitfield).

use serde::{Deserialize, Serialize};

/// A set of modifier keys, as pressed and before any per-app remapping: Command is the
/// Command key whatever the focused app sees.
///
/// The bits are `quire_shell_v1.mac_mods`'s values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MacMods(pub u32);

impl MacMods {
    /// No modifier.
    pub const NONE: MacMods = MacMods(0);
    /// Shift.
    pub const SHIFT: MacMods = MacMods(1);
    /// Control.
    pub const CONTROL: MacMods = MacMods(2);
    /// Option.
    pub const OPTION: MacMods = MacMods(4);
    /// Command.
    pub const COMMAND: MacMods = MacMods(8);
    /// Caps Lock is on.
    pub const CAPS_LOCK: MacMods = MacMods(16);
    /// Fn.
    pub const FUNCTION: MacMods = MacMods(32);

    /// Whether every key of `required` is held (any others may be held too).
    pub fn holds(self, required: MacMods) -> bool {
        self.0 & required.0 == required.0
    }

    /// This set with `other`'s keys added.
    pub fn with(self, other: MacMods) -> MacMods {
        MacMods(self.0 | other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::MacMods;

    /// (case, held, required, holds)
    const HOLDS: &[(&str, MacMods, MacMods, bool)] = &[
        (
            "nothing required is always held",
            MacMods::NONE,
            MacMods::NONE,
            true,
        ),
        ("the exact key", MacMods::COMMAND, MacMods::COMMAND, true),
        ("a missing key", MacMods::SHIFT, MacMods::COMMAND, false),
        (
            "extra keys do not matter",
            MacMods(8 | 1),
            MacMods::COMMAND,
            true,
        ),
        ("every key of a chord", MacMods(8 | 1), MacMods(8 | 1), true),
        (
            "half a chord is not enough",
            MacMods::COMMAND,
            MacMods(8 | 1),
            false,
        ),
    ];

    #[test]
    fn holds_needs_every_required_key() {
        for (case, held, required, want) in HOLDS {
            assert_eq!(held.holds(*required), *want, "{case}");
        }
    }

    #[test]
    fn with_adds_keys_and_keeps_the_ones_held() {
        let chord = MacMods::COMMAND.with(MacMods::SHIFT);
        assert_eq!(chord, MacMods(8 | 1));
        assert_eq!(chord.with(MacMods::SHIFT), chord);
        assert_eq!(MacMods::NONE.with(MacMods::OPTION), MacMods::OPTION);
    }

    #[test]
    fn every_key_has_its_own_bit() {
        let keys = [
            MacMods::SHIFT,
            MacMods::CONTROL,
            MacMods::OPTION,
            MacMods::COMMAND,
            MacMods::CAPS_LOCK,
            MacMods::FUNCTION,
        ];
        for key in keys {
            assert_eq!(key.0.count_ones(), 1, "{key:?}");
        }
        let all = keys.iter().fold(MacMods::NONE, |set, key| set.with(*key));
        assert_eq!(all.0.count_ones(), 6);
    }
}
