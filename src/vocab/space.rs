//! Space vocabulary: `quire_desktop_space_v1`'s enums.

use super::code::wire_enum;

wire_enum! {
    /// What a Space holds (`quire_desktop_space_v1.space_kind`).
    SpaceKind {
        /// An ordinary desktop.
        Desktop = 0,
        /// One full-screen window.
        FullScreen = 1,
        /// Two windows side by side.
        SplitView = 2,
        /// An agent's task.
        Agent = 3,
    }
}

wire_enum! {
    /// What is happening on a Space (`quire_desktop_space_v1.space_activity`).
    SpaceActivity {
        /// Nothing to show.
        None = 0,
        /// An agent run is working there.
        AgentRunning = 1,
    }
}
