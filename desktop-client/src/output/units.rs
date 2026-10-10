//! Output value types: no bool, no float.

/// Whether a head is switched on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Enabled {
    /// On.
    On,
    /// Off.
    #[default]
    Off,
}

/// Whether a mode is the head's preferred one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preferred {
    /// The preferred mode.
    Yes,
    /// Not the preferred mode.
    #[default]
    No,
}

/// Adaptive sync (variable refresh) of a head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum AdaptiveSync {
    /// Off.
    #[default]
    Disabled,
    /// On.
    Enabled,
}

/// A head's rotation and flip (`wl_output.transform`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Transform {
    /// As drawn.
    #[default]
    Normal,
    /// Rotated 90 degrees counter-clockwise.
    Rotate90,
    /// Rotated 180 degrees.
    Rotate180,
    /// Rotated 270 degrees counter-clockwise.
    Rotate270,
    /// Flipped about the vertical axis.
    Flipped,
    /// Flipped, then rotated 90.
    Flipped90,
    /// Flipped, then rotated 180.
    Flipped180,
    /// Flipped, then rotated 270.
    Flipped270,
}

/// A scale factor in thousandths (1500 is 1.5): `wl_fixed` as an integer, so it is `Eq`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScaleMilli(u32);

impl ScaleMilli {
    /// Scale 1.
    pub const ONE: ScaleMilli = ScaleMilli(1000);

    /// The scale `milli` thousandths.
    pub const fn new(milli: u32) -> Self {
        Self(milli)
    }

    /// The thousandths.
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A refresh rate in millihertz (60000 is 60 Hz), as the protocol carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshMilliHz(u32);

impl RefreshMilliHz {
    /// The rate `milli_hz`.
    pub const fn new(milli_hz: u32) -> Self {
        Self(milli_hz)
    }

    /// The millihertz.
    pub const fn get(self) -> u32 {
        self.0
    }
}
