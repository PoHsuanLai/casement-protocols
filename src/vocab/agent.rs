//! Agent vocabulary: `quire_agent_v1`'s enums, the lease state `quire_desktop_window_v1`
//! reports, and the press edge of input receipts.

use super::code::wire_enum;

wire_enum! {
    /// Where a run happens (`quire_agent_manager_v1.lease_mode`).
    RunMode {
        /// On the person's own seat, where the window is.
        InPlace = 0,
        /// In a hidden agent Space that keeps rendering.
        AgentWorkspace = 1,
        /// In a nested session.
        NestedSession = 2,
    }
}

wire_enum! {
    /// Why a lease ended without the agent asking (`quire_agent_lease_v1.revoke_reason`).
    RevokeReason {
        /// The hardware kill chord.
        KillChord = 0,
        /// The window was destroyed.
        WindowClosed = 1,
        /// The window left the run's Space.
        SpaceChanged = 2,
        /// The compositor withdrew or refused the lease.
        Compositor = 3,
    }
}

wire_enum! {
    /// Why an agent's input stops without ending the lease
    /// (`quire_agent_lease_v1.suspend_reason`).
    Suspend {
        /// The person took over.
        TakeOver = 0,
        /// A confirmation is showing.
        Consent = 1,
        /// The run is paused.
        Paused = 2,
    }
}

wire_enum! {
    /// What a run looks like from outside (`quire_agent_lease_v1.glow_state`).
    GlowCode {
        /// Nothing.
        Off = 0,
        /// Looking or thinking.
        Working = 1,
        /// Acting at a rectangle.
        Acting = 2,
        /// Needs the person.
        WaitingForYou = 3,
    }
}

wire_enum! {
    /// Why a captured region is masked (`quire_agent_lease_v1.mask_reason`).
    MaskReason {
        /// A password field.
        PasswordField = 0,
        /// The app declared the region sensitive.
        SurfaceDeclared = 1,
        /// An accessibility node marked secret.
        A11ySecret = 2,
        /// A sensitive window class.
        SensitiveWindow = 3,
        /// Content from another Space.
        OtherSpace = 4,
        /// An overlay that is not part of the leased window.
        OverlayOutsideLease = 5,
    }
}

wire_enum! {
    /// Whether an agent holds a window (`quire_desktop_window_v1.lease_state`).
    LeaseState {
        /// No lease.
        Free = 0,
        /// Leased, input flowing.
        Granted = 1,
        /// Leased, input stopped.
        Suspended = 2,
    }
}

wire_enum! {
    /// Whether a button or key went down or up, in an input receipt.
    PressEdge {
        /// Went down.
        Press = 0,
        /// Went up.
        Release = 1,
    }
}
