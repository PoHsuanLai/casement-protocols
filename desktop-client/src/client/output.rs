//! Output dispatch: the manager, heads, modes and configurations (`zwlr-output-management-v1`).

use wayland_protocols_wlr::output_management::v1::client::{
    zwlr_output_configuration_v1, zwlr_output_head_v1, zwlr_output_manager_v1, zwlr_output_mode_v1,
};

use super::wire::Wire;
use crate::keys::{ConfigId, HeadKey, ModeKey};
use crate::output::{ConfigPurpose, OutputConfig};

// The fill lane adds the `Dispatch` impls (manager `()`, head `HeadKey`, mode `ModeKey`,
// configuration `(ConfigId, ConfigPurpose)`; event_created_child for `head`, and for a head's
// `mode`). wl_fixed scale and position become `ScaleMilli` and `Point`.

/// The manager's event.
pub(super) fn on_manager(wire: &mut Wire, event: zwlr_output_manager_v1::Event) {
    let _ = (wire, event);
    todo!("fill")
}

/// A head's event.
pub(super) fn on_head(wire: &mut Wire, key: HeadKey, event: zwlr_output_head_v1::Event) {
    let _ = (wire, key, event);
    todo!("fill")
}

/// A mode's event.
pub(super) fn on_mode(wire: &mut Wire, key: ModeKey, event: zwlr_output_mode_v1::Event) {
    let _ = (wire, key, event);
    todo!("fill")
}

/// A configuration's answer.
pub(super) fn on_configuration(
    wire: &mut Wire,
    id: ConfigId,
    purpose: ConfigPurpose,
    event: zwlr_output_configuration_v1::Event,
) {
    let _ = (wire, id, purpose, event);
    todo!("fill")
}

/// Builds and sends a `create_configuration` for `config` quoting the state's serial, mentioning
/// every enabled head the config leaves out, and applies or tests it.
pub(super) fn submit(
    wire: &mut Wire,
    config: &OutputConfig,
    purpose: ConfigPurpose,
) -> Result<ConfigId, super::error::WireError> {
    let _ = (wire, config, purpose);
    todo!("fill")
}
