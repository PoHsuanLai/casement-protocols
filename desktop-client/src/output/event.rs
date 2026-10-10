//! What `zwlr-output-management-v1` reports.

use super::config::{ConfigOutcome, ConfigPurpose};
use super::units::{AdaptiveSync, Enabled, Preferred, RefreshMilliHz, ScaleMilli, Transform};
use crate::geometry::{Point, Size};
use crate::keys::{ConfigId, HeadKey, ModeKey};
use crate::names::OutputName;

/// One property of a head changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeadChange {
    /// The name.
    Name(OutputName),
    /// The description.
    Description(String),
    /// Physical size in millimetres.
    PhysicalSize(Size),
    /// Manufacturer.
    Make(String),
    /// Model.
    Model(String),
    /// Serial number.
    SerialNumber(String),
    /// On or off.
    Enabled(Enabled),
    /// The mode in use.
    CurrentMode(ModeKey),
    /// Position.
    Position(Point),
    /// Rotation and flip.
    Transform(Transform),
    /// Scale.
    Scale(ScaleMilli),
    /// Adaptive sync.
    AdaptiveSync(AdaptiveSync),
}

/// One property of a mode changing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModeChange {
    /// Pixels.
    Size(Size),
    /// Refresh rate.
    Refresh(RefreshMilliHz),
    /// Preferred or not.
    Preferred(Preferred),
    /// The mode is gone.
    Finished,
}

/// An output event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutputEvent {
    /// A head appeared.
    HeadOpened(HeadKey),
    /// A head changed.
    Head {
        /// The head.
        key: HeadKey,
        /// What changed.
        change: HeadChange,
    },
    /// A head is gone.
    HeadFinished(HeadKey),
    /// A mode appeared on a head.
    ModeOpened {
        /// The head.
        head: HeadKey,
        /// The mode.
        mode: ModeKey,
    },
    /// A mode changed.
    Mode {
        /// The mode.
        mode: ModeKey,
        /// What changed.
        change: ModeChange,
    },
    /// The batch is complete; `serial` is what a configuration must quote (`done`).
    Done {
        /// The serial.
        serial: u32,
    },
    /// The manager is finished (`finished`).
    Finished,
    /// A configuration the client sent has an answer.
    ConfigResolved {
        /// Which one.
        id: ConfigId,
        /// Applied or only tested.
        purpose: ConfigPurpose,
        /// How it went.
        outcome: ConfigOutcome,
    },
}
