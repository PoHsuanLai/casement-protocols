//! Keys: the client's own numbers for protocol objects.
//!
//! The core holds no proxy. The wire layer hands out a key when an object appears and keeps
//! the key-to-proxy map, so state is plain data and a command names its target by key.

macro_rules! key {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(u64);

        impl $name {
            /// The key with this number.
            pub const fn new(number: u64) -> Self {
                Self(number)
            }

            /// The number.
            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

key! {
    /// One toplevel for as long as it lives (the pairing of its two protocol handles).
    ToplevelKey
}
key! {
    /// One workspace (`ext_workspace_handle_v1`).
    WorkspaceKey
}
key! {
    /// One workspace group (`ext_workspace_group_handle_v1`).
    GroupKey
}
key! {
    /// One output head (`zwlr_output_head_v1`).
    HeadKey
}
key! {
    /// One output mode (`zwlr_output_mode_v1`); unique across heads.
    ModeKey
}
key! {
    /// A `wl_surface` the caller registered, named so a command can point at it (the
    /// minimise target is a surface).
    SurfaceKey
}
key! {
    /// One output capture, from the request to its frame.
    CaptureId
}
key! {
    /// One output configuration being applied or tested.
    ConfigId
}
