//! The output transition.

use super::event::OutputEvent;
use super::set::Outputs;

/// `outputs` after `event`. A change to an unknown key is ignored; `Done` records the serial;
/// `ConfigResolved` does not touch the heads (the compositor sends the new head state next).
pub(crate) fn reduce_outputs(outputs: Outputs, event: OutputEvent) -> Outputs {
    let _ = (outputs, event);
    todo!("fill: edit heads and modes; Done stores the serial; HeadFinished and Finished drop")
}
