//! Trusted-surface vocabulary: `quire_trusted_surface_v1`'s enums.

use super::code::wire_enum;

wire_enum! {
    /// What a trusted surface asks of the person (`quire_trusted_manager_v1.trusted_kind`).
    TrustedKind {
        /// A question the person answers.
        Confirm = 0,
        /// A notice the person acknowledges.
        Notice = 1,
    }
}

wire_enum! {
    /// Where a trusted surface appears (`quire_trusted_manager_v1.trusted_anchor`).
    TrustedAnchor {
        /// Attached to a window, as a sheet.
        Window = 0,
        /// Centred on the output with the pointer.
        Centre = 1,
    }
}

wire_enum! {
    /// Why the compositor took a trusted surface down (`quire_trusted_surface_v1.withdrawn`).
    Withdrawn {
        /// The session locked.
        Lock = 0,
        /// The anchoring window closed.
        TargetGone = 1,
        /// The kill chord was pressed.
        Kill = 2,
    }
}
