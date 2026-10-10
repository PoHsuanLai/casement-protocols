//! Names the compositor gives things.

/// An output's name as the compositor says it (`DP-1`): `wl_output.name` and
/// `zwlr_output_head_v1.name` carry the same string, which is how a toplevel's output, a
/// workspace group's output and a head are the same output.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OutputName(String);

impl OutputName {
    /// The name `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// The text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The compositor's stable identifier of a toplevel (`ext_foreign_toplevel_handle_v1.identifier`):
/// unique for the compositor's lifetime, safe to store and to send to another process.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToplevelIdentifier(String);

impl ToplevelIdentifier {
    /// The identifier `identifier`.
    pub fn new(identifier: impl Into<String>) -> Self {
        Self(identifier.into())
    }

    /// The text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An app's key as the desktop names it (`quire_desktop_app_v1.identity`'s `key`): the string
/// the rest of the desktop uses for the app, safe to store and to send to another process.
/// [`crate::AppKey`] is this client's number for the same app's protocol object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AppId(String);

impl AppId {
    /// The app key `id`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A Space's stable id (`quire_desktop_space_v1.id`), also the `id` of its ext-workspace
/// workspace. [`crate::SpaceKey`] is this client's number for the Space's protocol object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpaceId(String);

impl SpaceId {
    /// The Space id `id`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
