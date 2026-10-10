//! A head and its modes.

use super::units::{AdaptiveSync, Enabled, Preferred, RefreshMilliHz, ScaleMilli, Transform};
use crate::geometry::{Point, Size};
use crate::keys::{HeadKey, ModeKey};
use crate::names::OutputName;

/// A mode a head can run.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Mode {
    /// This client's key for it.
    pub key: ModeKey,
    /// Pixels.
    pub size: Size,
    /// Refresh rate.
    pub refresh: RefreshMilliHz,
    /// Whether the head prefers it.
    pub preferred: Preferred,
}

impl Mode {
    /// A mode that has just appeared.
    pub fn new(key: ModeKey) -> Self {
        Self {
            key,
            size: Size::default(),
            refresh: RefreshMilliHz::new(0),
            preferred: Preferred::No,
        }
    }
}

/// One output head (a connector with a display behind it).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Head {
    /// This client's key for it.
    pub key: HeadKey,
    /// Its name (`DP-1`).
    pub name: OutputName,
    /// A description a person reads.
    pub description: String,
    /// Physical size in millimetres, when known.
    pub physical_size_mm: Option<Size>,
    /// Manufacturer, when known.
    pub make: Option<String>,
    /// Model, when known.
    pub model: Option<String>,
    /// Serial number, when known.
    pub serial_number: Option<String>,
    /// On or off.
    pub enabled: Enabled,
    /// Every mode, in the order announced.
    pub modes: Vec<Mode>,
    /// The mode in use; `None` while off.
    pub current_mode: Option<ModeKey>,
    /// Position in the global compositor space.
    pub position: Point,
    /// Rotation and flip.
    pub transform: Transform,
    /// Scale.
    pub scale: ScaleMilli,
    /// Adaptive sync; `None` when the compositor does not say (protocol version 4).
    pub adaptive_sync: Option<AdaptiveSync>,
}

impl Head {
    /// A head that has just appeared.
    pub fn new(key: HeadKey) -> Self {
        Self {
            key,
            name: OutputName::new(""),
            description: String::new(),
            physical_size_mm: None,
            make: None,
            model: None,
            serial_number: None,
            enabled: Enabled::Off,
            modes: Vec::new(),
            current_mode: None,
            position: Point::default(),
            transform: Transform::Normal,
            scale: ScaleMilli::ONE,
            adaptive_sync: None,
        }
    }

    /// The mode in use.
    pub fn current(&self) -> Option<&Mode> {
        let key = self.current_mode?;
        self.modes.iter().find(|m| m.key == key)
    }
}
