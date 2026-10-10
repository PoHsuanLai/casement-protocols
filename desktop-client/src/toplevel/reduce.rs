//! The toplevel transition.

use super::event::ToplevelEvent;
use super::set::Toplevels;

/// `toplevels` after `event`. A change to an unknown key is ignored; `Merged` keeps the
/// managed record's state and takes the identifier.
pub(crate) fn reduce_toplevels(toplevels: Toplevels, event: ToplevelEvent) -> Toplevels {
    let _ = (toplevels, event);
    todo!(
        "fill: Opened inserts Pending; Changed edits; Done marks Announced; Merged; Closed removes"
    )
}
