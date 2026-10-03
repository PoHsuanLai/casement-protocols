//! Sensitivity vocabulary: `quire_sensitive_surface_v1`'s level.

use super::code::wire_enum;

wire_enum! {
    /// Who may not see a surface's content (`quire_sensitive_surface_v1.sensitivity_level`).
    SensitivityLevel {
        /// Visible to every permitted capture.
        Normal = 0,
        /// Masked in agent captures.
        AgentHidden = 1,
        /// Masked in every capture.
        CaptureHidden = 2,
    }
}
