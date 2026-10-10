//! Outputs: `zwlr-output-management-v1`.
//!
//! Heads and modes are state; a configuration is built as a value ([`OutputConfig`]), checked
//! against the state ([`OutputConfig::validate`]) and sent with `DesktopCommand::ApplyOutputs`
//! or `TestOutputs`.

mod config;
mod event;
mod model;
mod reduce;
mod set;
mod units;

pub use config::{
    ConfigError, ConfigOutcome, ConfigPurpose, HeadChangeRequest, HeadConfig, HeadSettings,
    ModeChoice, OutputConfig,
};
pub use event::{HeadChange, ModeChange, OutputEvent};
pub use model::{Head, Mode};
pub(crate) use reduce::reduce_outputs;
pub use set::Outputs;
pub use units::{AdaptiveSync, Enabled, Preferred, RefreshMilliHz, ScaleMilli, Transform};
