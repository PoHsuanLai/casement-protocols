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
