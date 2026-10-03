//! Wire codes: the one trait every vocab enum implements, and the macro that writes them.

/// A closed set whose values travel as `uint` codes on the wire.
///
/// Every implementation is written by `wire_enum!`, so a code is stated once, beside its
/// variant.
pub trait WireCode: Sized + Copy + 'static {
    /// Every value, in code order.
    const ALL: &'static [Self];
    /// The value's code on the wire.
    fn code(self) -> u32;
    /// The value a wire code names.
    fn from_code(code: u32) -> Result<Self, CodeError>;
}

/// A wire code no value of the set carries.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CodeError {
    /// `code` is outside the set `what`.
    #[error("{what}: no value has code {code}")]
    Unknown {
        /// The set's name.
        what: &'static str,
        /// The code that arrived.
        code: u32,
    },
}

/// Writes a vocab enum, its [`WireCode`] impl and its `ALL`.
///
/// Every variant carries its doc and its code; the serde form is the XML entry name
/// (`snake_case`), so a stored value reads the same as the protocol.
macro_rules! wire_enum {
    (
        $(#[$doc:meta])*
        $name:ident {
            $( $(#[$vdoc:meta])* $variant:ident = $code:literal, )+
        }
    ) => {
        $(#[$doc])*
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(rename_all = "snake_case")]
        #[repr(u32)]
        pub enum $name {
            $( $(#[$vdoc])* $variant = $code, )+
        }

        impl $crate::vocab::WireCode for $name {
            const ALL: &'static [Self] = &[$(Self::$variant),+];

            fn code(self) -> u32 {
                self as u32
            }

            fn from_code(code: u32) -> Result<Self, $crate::vocab::CodeError> {
                match code {
                    $( $code => Ok(Self::$variant), )+
                    _ => Err($crate::vocab::CodeError::Unknown { what: stringify!($name), code }),
                }
            }
        }
    };
}
pub(crate) use wire_enum;
