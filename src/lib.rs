//! The desktop's private Wayland protocols, `quire_*_v1`.
//!
//! The six XML files in `protocols/` are the interface. This crate carries them in three
//! layers:
//!
//! - [`vocab`] (always built, serde only): one Rust enum per XML enum, with the same codes, so
//!   pure crates (the compositor's model, the shell's model, the agent tier) name wire values
//!   without linking Wayland;
//! - `client` (feature `client`): wayland-scanner client bindings, one module per protocol;
//! - `server` (feature `server`): wayland-scanner server bindings, one module per protocol.
//!
//! The protocols build on wayland-protocols' `xdg_shell`, `ext-foreign-toplevel-list-v1` and
//! `ext-image-capture-source-v1`.

pub mod vocab;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
