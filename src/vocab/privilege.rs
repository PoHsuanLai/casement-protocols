//! Consent vocabulary: the powers a program must be allowed, who asks and what the person said
//! (`quire_shell_v1`'s `permission_ask`, `permission_withdraw` and `permission_answer`).
//!
//! The words a person reads for a power live here, once, so the sheet that asks and the page in
//! Settings that lists the answers say the same thing.

use super::code::wire_enum;

wire_enum! {
    /// One thing a program needs the person's consent for (`quire_shell_v1.power`).
    Power {
        /// See what is on the screen, including other apps' windows.
        Screen = 0,
        /// Press keys and buttons as if the person did.
        Keys = 1,
        /// See what is typed, to offer suggestions and other languages.
        TypingHelp = 2,
        /// Read and replace what the person copies.
        Clipboard = 3,
        /// Cover the screen and ask the person to sign in.
        Lock = 4,
        /// Turn screens off and change their size, position and colours.
        Displays = 5,
        /// See the names of other apps' windows and move, raise or close them.
        Windows = 6,
    }
}

impl Power {
    /// The section title in Settings, under Privacy.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Screen => "Screen",
            Self::Keys => "Typing and clicking",
            Self::TypingHelp => "Typing help",
            Self::Clipboard => "Copy and paste",
            Self::Lock => "Locking the screen",
            Self::Displays => "Displays",
            Self::Windows => "Windows",
        }
    }

    /// Plain words on what the power lets a program do, for the question and for the
    /// section in Settings.
    pub const fn blurb(self) -> &'static str {
        match self {
            Self::Screen => {
                "It will be able to see everything that is shown, including your other apps' windows."
            }
            Self::Keys => "It will be able to press keys and buttons as if you did, in any app.",
            Self::TypingHelp => {
                "It will be able to see what you type, to offer suggestions and other languages."
            }
            Self::Clipboard => {
                "It will be able to read everything you copy, such as passwords and messages, and replace it."
            }
            Self::Lock => {
                "It will cover the screen and ask you to sign in. Only allow a program you trust: it will see the password you type."
            }
            Self::Displays => {
                "It will be able to turn screens off and change their size, position and colours."
            }
            Self::Windows => {
                "It will be able to see the names of your open windows and bring them forward, move them or close them."
            }
        }
    }
}

wire_enum! {
    /// How well the compositor knows which program is asking (`quire_shell_v1.subject_kind`).
    SubjectKind {
        /// A program whose image the system owns; the subject is its path.
        Program = 0,
        /// A sandboxed app; the subject is its app id.
        SandboxedApp = 1,
        /// A program the compositor cannot vouch for; the subject is its path, or empty.
        Unrecognised = 2,
    }
}

wire_enum! {
    /// What an answer will be kept as (`quire_shell_v1.memory`).
    Memory {
        /// The answer is kept; the program is not asked again.
        Remembered = 0,
        /// Nothing is kept; an allow lasts while the program stays connected.
        ThisTimeOnly = 1,
    }
}

wire_enum! {
    /// What the person said (`quire_shell_v1.answer`).
    Answer {
        /// Allow, or Allow Once when the ask's memory is `ThisTimeOnly`.
        Allow = 0,
        /// Don't Allow.
        Deny = 1,
        /// The question was closed without an answer.
        Dismissed = 2,
    }
}

/// The id of one question to the person, unique within a compositor run and never reused
/// (`permission_ask`'s `ask`).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct AskId(u32);

impl AskId {
    /// The id the wire carries as `value`.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// The number on the wire.
    pub const fn get(self) -> u32 {
        self.0
    }
}
