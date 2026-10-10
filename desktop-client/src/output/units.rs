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

impl Transform {
    /// The transform with this `wl_output.transform` code; `None` outside 0 to 7.
    pub const fn from_code(code: u32) -> Option<Self> {
        match code {
            0 => Some(Self::Normal),
            1 => Some(Self::Rotate90),
            2 => Some(Self::Rotate180),
            3 => Some(Self::Rotate270),
            4 => Some(Self::Flipped),
            5 => Some(Self::Flipped90),
            6 => Some(Self::Flipped180),
            7 => Some(Self::Flipped270),
            _ => None,
        }
    }

    /// The `wl_output.transform` code.
    pub const fn code(self) -> u32 {
        match self {
            Self::Normal => 0,
            Self::Rotate90 => 1,
            Self::Rotate180 => 2,
            Self::Rotate270 => 3,
            Self::Flipped => 4,
            Self::Flipped90 => 5,
            Self::Flipped180 => 6,
            Self::Flipped270 => 7,
        }
    }
}

impl Enabled {
    /// `On` for a non-zero wire value.
    pub const fn from_wire(value: i32) -> Self {
        if value == 0 { Self::Off } else { Self::On }
    }
}

impl AdaptiveSync {
    /// `Enabled` for the wire value 1.
    pub const fn from_code(code: u32) -> Self {
        if code == 1 {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }

    /// The wire value.
    pub const fn code(self) -> u32 {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
        }
    }
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

    /// The scale nearest `value`, a `wl_fixed` already read as a float; negative and NaN
    /// become 0.
    pub fn from_f64(value: f64) -> Self {
        Self((value * 1000.0).round().max(0.0) as u32)
    }

    /// The scale as the float `wl_fixed` carries.
    pub fn to_f64(self) -> f64 {
        f64::from(self.0) / 1000.0
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
