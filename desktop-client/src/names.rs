//! Names the compositor gives things.

macro_rules! name {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// The value with this text.
            pub fn new(text: impl Into<String>) -> Self {
                Self(text.into())
            }

            /// The text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

name! {
    /// An output's name as the compositor says it (`DP-1`): `wl_output.name` and
    /// `zwlr_output_head_v1.name` carry the same string, which is how a toplevel's output, a
    /// workspace group's output and a head are the same output.
    OutputName
}

name! {
    /// The compositor's stable identifier of a toplevel (`ext_foreign_toplevel_handle_v1.identifier`):
    /// unique for the compositor's lifetime, safe to store and to send to another process.
    ToplevelIdentifier
}

name! {
    /// An app's key as the desktop names it (`quire_desktop_app_v1.identity`'s `key`): the string
    /// the rest of the desktop uses for the app, safe to store and to send to another process.
    /// [`crate::AppKey`] is this client's number for the same app's protocol object.
    AppId
}

name! {
    /// The program behind an app as the compositor established it (`quire_desktop_app_v1.subject`):
    /// the image path of a native process, or the app id of a sandboxed one. A fact, not the app's
    /// claim, so it can be compared with the standing of another process (the owner of the app's
    /// menu on the session bus).
    AppSubject
}

name! {
    /// A Space's stable id (`quire_desktop_space_v1.id`), also the `id` of its ext-workspace
    /// workspace. [`crate::SpaceKey`] is this client's number for the Space's protocol object.
    SpaceId
}
