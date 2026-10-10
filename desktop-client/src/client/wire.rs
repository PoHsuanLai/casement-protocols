//! The state the Wayland dispatchers write into.

use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Proxy, QueueHandle, WEnum};

use super::handles::Handles;
use super::pairing::Book;
use crate::event::DesktopEvent;
use crate::names::OutputName;
use crate::state::DesktopState;

/// Everything the dispatch impls touch: the reduced state, the events not yet drained and the
/// proxies behind the keys.
#[derive(Debug)]
pub(super) struct Wire {
    pub(super) state: DesktopState,
    pub(super) events: Vec<DesktopEvent>,
    pub(super) handles: Handles,
    pub(super) book: Book,
    pub(super) qh: QueueHandle<Wire>,
    next: u64,
}

impl Wire {
    /// An empty wire on the queue `qh`.
    pub(super) fn new(qh: QueueHandle<Wire>) -> Self {
        Self {
            state: DesktopState::default(),
            events: Vec::new(),
            handles: Handles::default(),
            book: Book::default(),
            qh,
            next: 1,
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
        let number = self.next;
        self.next += 1;
        number
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
