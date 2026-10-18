//! The state the Wayland dispatchers write into.

use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Proxy, QueueHandle, WEnum};

use super::handles::Handles;
use super::pairing::Book;
use crate::config::DesktopConfig;
use crate::event::DesktopEvent;
use crate::keys::Mint;
use crate::names::OutputName;
use crate::quire::Linker;
use crate::state::DesktopState;
use casement_protocols::vocab::WireCode;

/// Everything the dispatch impls touch: the reduced state, the events not yet drained and the
/// proxies behind the keys.
#[derive(Debug)]
pub(super) struct Wire {
    pub(super) state: DesktopState,
    pub(super) events: Vec<DesktopEvent>,
    pub(super) handles: Handles,
    pub(super) book: Book,
    /// Quire window objects and the ext handles they belong to.
    pub(super) linker: Linker,
    pub(super) config: DesktopConfig,
    pub(super) qh: QueueHandle<Wire>,
    mint: Mint,
}

impl Wire {
    /// An empty wire on the queue `qh`.
    pub(super) fn new(qh: QueueHandle<Wire>, config: DesktopConfig) -> Self {
        Self {
            state: DesktopState::default(),
            events: Vec::new(),
            handles: Handles::default(),
            book: Book::default(),
            linker: Linker::default(),
            config,
            qh,
            mint: Mint::default(),
        }
    }

    /// Reduces `event` into the state and keeps it for draining.
    pub(super) fn emit(&mut self, event: DesktopEvent) {
        let state = std::mem::take(&mut self.state);
        self.state = crate::reduce(state, event.clone());
        self.events.push(event);
    }

    /// The output's name, when it has announced one.
    pub(super) fn output_name(&self, output: &WlOutput) -> Option<OutputName> {
        self.handles
            .output_names
            .get(&output.id().protocol_id())
            .cloned()
    }

    /// A number no earlier call returned, for the ids this client makes (configurations,
    /// captures, surfaces).
    pub(super) fn mint(&mut self) -> u64 {
        self.mint.fresh()
    }
}

/// The key number of a compositor-made object: its protocol id, unique among live objects. A
/// record is removed from the state when its object goes, so a reused id never meets a stale
/// record.
pub(super) fn number_of(proxy: &impl Proxy) -> u64 {
    u64::from(proxy.id().protocol_id())
}

/// The wire code of an enum or bitfield argument, known to the bindings or not.
pub(super) fn raw<T: Into<u32>>(value: WEnum<T>) -> u32 {
    match value {
        WEnum::Value(known) => known.into(),
        WEnum::Unknown(code) => code,
    }
}

/// The vocab value of an enum argument, `None` for a code the vocab does not know (a newer
/// compositor): the event is then skipped.
pub(super) fn vocab<V: WireCode, T: Into<u32>>(value: WEnum<T>) -> Option<V> {
    V::from_code(raw(value)).ok()
}
