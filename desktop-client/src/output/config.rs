//! A configuration to apply or test.

use std::collections::BTreeSet;

use super::model::Head;
use super::set::Outputs;
use super::units::{AdaptiveSync, RefreshMilliHz, ScaleMilli, Transform};
use crate::geometry::{Point, Size};
use crate::keys::{HeadKey, ModeKey};

/// A full configuration: every head the client mentions. A head left out keeps its state
/// (the protocol's rule is that a configuration must mention every enabled head; the wire
/// layer fills in the unmentioned ones from the state).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct OutputConfig {
    /// The heads, each once.
    pub heads: Vec<HeadConfig>,
}

impl OutputConfig {
    /// A configuration of these heads.
    pub fn new(heads: Vec<HeadConfig>) -> Self {
        Self { heads }
    }

    /// Checks it against `outputs`: every head and mode exists, a mode belongs to its head, no
    /// head is named twice, a custom mode has a non-zero size and refresh, and the
    /// configuration is not empty.
    pub fn validate(&self, outputs: &Outputs) -> Result<(), ConfigError> {
        if self.heads.is_empty() {
            return Err(ConfigError::Empty);
        }
        let mut seen = BTreeSet::new();
        for part in &self.heads {
            let head = outputs
                .get(part.head)
                .ok_or(ConfigError::UnknownHead(part.head))?;
            if !seen.insert(part.head) {
                return Err(ConfigError::DuplicateHead(part.head));
            }
            if let HeadChangeRequest::Enable(settings) = &part.request
                && let Some(choice) = &settings.mode
            {
                check_mode(outputs, head, choice)?;
            }
        }
        Ok(())
    }
}

fn check_mode(outputs: &Outputs, head: &Head, choice: &ModeChoice) -> Result<(), ConfigError> {
    let head_key = head.key;
    match choice {
        ModeChoice::Existing(mode) if head.modes.iter().any(|m| m.key == *mode) => Ok(()),
        ModeChoice::Existing(mode) => {
            let elsewhere = outputs
                .heads()
                .any(|h| h.modes.iter().any(|m| m.key == *mode));
            if elsewhere {
                Err(ConfigError::ModeOfOtherHead {
                    head: head_key,
                    mode: *mode,
                })
            } else {
                Err(ConfigError::UnknownMode(*mode))
            }
        }
        ModeChoice::Custom { size, refresh } => {
            if size.w == 0 || size.h == 0 || refresh.get() == 0 {
                Err(ConfigError::InvalidCustomMode(head_key))
            } else {
                Ok(())
            }
        }
    }
}

/// One head's part of a configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct HeadConfig {
    /// The head.
    pub head: HeadKey,
    /// Switch it off or on with these settings.
    pub request: HeadChangeRequest,
}

impl HeadConfig {
    /// A head's part.
    pub fn new(head: HeadKey, request: HeadChangeRequest) -> Self {
        Self { head, request }
    }
}

/// Off, or on with settings.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeadChangeRequest {
    /// `disable_head`.
    Disable,
    /// `enable_head` and the settings.
    Enable(HeadSettings),
}

/// The settings of an enabled head; `None` leaves the head's current value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct HeadSettings {
    /// Which mode.
    pub mode: Option<ModeChoice>,
    /// Where.
    pub position: Option<Point>,
    /// Rotation and flip.
    pub transform: Option<Transform>,
    /// Scale.
    pub scale: Option<ScaleMilli>,
    /// Adaptive sync.
    pub adaptive_sync: Option<AdaptiveSync>,
}

impl HeadSettings {
    /// Settings that change nothing.
    pub fn keep() -> Self {
        Self::default()
    }

    /// These settings with `mode`.
    pub fn with_mode(mut self, mode: ModeChoice) -> Self {
        self.mode = Some(mode);
        self
    }

    /// These settings at `position`.
    pub fn at(mut self, position: Point) -> Self {
        self.position = Some(position);
        self
    }

    /// These settings with `scale`.
    pub fn scaled(mut self, scale: ScaleMilli) -> Self {
        self.scale = Some(scale);
        self
    }

    /// These settings with `transform`.
    pub fn transformed(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }
}

/// A mode the head has, or a custom one.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModeChoice {
    /// One of the head's own modes (`set_mode`).
    Existing(ModeKey),
    /// A size and refresh the head did not list (`set_custom_mode`).
    Custom {
        /// Pixels.
        size: Size,
        /// Refresh rate.
        refresh: RefreshMilliHz,
    },
}

/// Applied, or only tested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigPurpose {
    /// `apply`.
    Apply,
    /// `test`.
    Test,
}

/// How a configuration ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConfigOutcome {
    /// `succeeded`.
    Succeeded,
    /// `failed`: the compositor cannot do it.
    Failed,
    /// `cancelled`: the heads changed under it; build it again from the new state.
    Cancelled,
}

/// A configuration that does not fit the current heads.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// It names no head.
    #[error("the configuration names no head")]
    Empty,
    /// A head it names does not exist.
    #[error("no head {0:?}")]
    UnknownHead(HeadKey),
    /// A head is named twice.
    #[error("head {0:?} is named twice")]
    DuplicateHead(HeadKey),
    /// A mode does not exist.
    #[error("no mode {0:?}")]
    UnknownMode(ModeKey),
    /// A custom mode has a zero width, height or refresh rate.
    #[error("custom mode of head {0:?} has a zero size or refresh")]
    InvalidCustomMode(HeadKey),
    /// The compositor has not finished describing the heads (no serial to quote yet).
    #[error("the compositor has not finished describing the output heads")]
    NotSettled,
    /// A mode belongs to another head.
    #[error("mode {mode:?} is not a mode of head {head:?}")]
    ModeOfOtherHead {
        /// The head.
        head: HeadKey,
        /// The mode.
        mode: ModeKey,
    },
}
