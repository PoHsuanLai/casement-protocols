//! A question from the compositor to the person (`quire_shell_v1`, version 5): may this program
//! use this power?

use casement_protocols::vocab::{AskId, Memory, Power, SubjectKind};

/// One question, as `permission_ask` carries it.
///
/// The shell puts a sheet on screen from it and answers with
/// [`DesktopCommand::AnswerPermission`](crate::DesktopCommand::AnswerPermission). The compositor
/// sends every ask that is still open again when a shell binds, with the same [`AskId`], so a
/// shell keeps its queue keyed by id and treats a repeat as the same question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PermissionAsk {
    /// The question's id; the answer and the withdrawal name it.
    pub ask: AskId,
    /// What the program wants to do.
    pub power: Power,
    /// How well the compositor knows which program it is.
    pub subject_kind: SubjectKind,
    /// The program's path or the sandboxed app's id, at most 4096 bytes; may be empty for an
    /// unrecognised program.
    pub subject: String,
    /// What the answer will be kept as; `ThisTimeOnly` means offer Allow Once.
    pub memory: Memory,
}

impl PermissionAsk {
    /// A question.
    pub fn new(
        ask: AskId,
        power: Power,
        subject_kind: SubjectKind,
        subject: impl Into<String>,
        memory: Memory,
    ) -> Self {
        Self {
            ask,
            power,
            subject_kind,
            subject: subject.into(),
            memory,
        }
    }
}
