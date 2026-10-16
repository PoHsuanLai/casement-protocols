//! Which protocols the compositor offers.

use std::collections::BTreeSet;
use std::fmt;

/// A global this crate can use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Protocol {
    /// `ext_foreign_toplevel_list_v1`: toplevel identity.
    ExtForeignToplevelList,
    /// `zwlr_foreign_toplevel_manager_v1`: toplevel state and actions.
    WlrForeignToplevelManagement,
    /// `ext_workspace_manager_v1`.
    ExtWorkspace,
    /// `zwlr_output_manager_v1`.
    WlrOutputManagement,
    /// `ext_image_copy_capture_manager_v1`.
    ExtImageCopyCapture,
    /// `ext_output_image_capture_source_manager_v1`: an output as a capture source.
    ExtOutputImageCaptureSource,
    /// `quire_desktop_manager_v1`: apps, windows and Spaces (the Quire arm).
    QuireDesktop,
    /// `quire_window_manager_v1`: an app's own say about its toplevels, and (version 2) the
    /// person's request that the app quit (`quit`). Recorded in the offer; this crate does not
    /// bind it (apps bind it for their own windows and listen for `quit` on the generated
    /// `client::quire_window_v1` binding).
    QuireWindow,
    /// `quire_shell_v1`: the shell's roles and system events, on Shell connections only.
    QuireShell,
}

impl Protocol {
    /// Every protocol, in declaration order.
    pub const ALL: [Protocol; 9] = [
        Protocol::ExtForeignToplevelList,
        Protocol::WlrForeignToplevelManagement,
        Protocol::ExtWorkspace,
        Protocol::WlrOutputManagement,
        Protocol::ExtImageCopyCapture,
        Protocol::ExtOutputImageCaptureSource,
        Protocol::QuireDesktop,
        Protocol::QuireWindow,
        Protocol::QuireShell,
    ];

    /// The global's interface name in the registry.
    pub const fn interface(self) -> &'static str {
        match self {
            Self::ExtForeignToplevelList => "ext_foreign_toplevel_list_v1",
            Self::WlrForeignToplevelManagement => "zwlr_foreign_toplevel_manager_v1",
            Self::ExtWorkspace => "ext_workspace_manager_v1",
            Self::WlrOutputManagement => "zwlr_output_manager_v1",
            Self::ExtImageCopyCapture => "ext_image_copy_capture_manager_v1",
            Self::ExtOutputImageCaptureSource => "ext_output_image_capture_source_manager_v1",
            Self::QuireDesktop => "quire_desktop_manager_v1",
            Self::QuireWindow => "quire_window_manager_v1",
            Self::QuireShell => "quire_shell_v1",
        }
    }

    /// The protocol a registry interface name belongs to, if this crate uses it.
    pub fn from_interface(interface: &str) -> Option<Protocol> {
        Self::ALL.into_iter().find(|p| p.interface() == interface)
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.interface())
    }
}

/// The protocols the compositor offered, from its registry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProtocolSet(BTreeSet<Protocol>);

impl ProtocolSet {
    /// Nothing offered.
    pub fn empty() -> Self {
        Self::default()
    }

    /// The set of the registry interface names `interfaces` that this crate uses; others are
    /// ignored.
    pub fn from_interfaces<'a>(interfaces: impl IntoIterator<Item = &'a str>) -> Self {
        Self(
            interfaces
                .into_iter()
                .filter_map(Protocol::from_interface)
                .collect(),
        )
    }

    /// This set and `protocol`.
    pub fn with(mut self, protocol: Protocol) -> Self {
        self.0.insert(protocol);
        self
    }

    /// Whether `protocol` is offered.
    pub fn contains(&self, protocol: Protocol) -> bool {
        self.0.contains(&protocol)
    }

    /// The offered protocols, in order.
    pub fn iter(&self) -> impl Iterator<Item = Protocol> + '_ {
        self.0.iter().copied()
    }
}
