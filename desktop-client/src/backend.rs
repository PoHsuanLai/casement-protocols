//! Which arm carries the toplevels.

use crate::protocol::{Protocol, ProtocolSet};

/// The protocols that describe and act on toplevels, chosen from the offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DesktopBackend {
    /// `ext-foreign-toplevel-list` for identity plus `quire_desktop_v1` for state, places,
    /// apps and Spaces.
    Quire,
    /// `zwlr-foreign-toplevel-management` (paired with the ext list when that is offered).
    Wlr,
    /// The ext list alone: identity, title and app id, no state and no actions.
    Ext,
}

/// The arm for `offered`, from the globals alone (never from a compositor's name or the
/// environment): Quire when `quire_desktop_v1` and the ext list are offered, else wlr when its
/// manager is, else the ext list alone; `None` when no toplevel protocol is offered.
///
/// ```
/// use desktop_client::{DesktopBackend, Protocol, ProtocolSet, backend_for};
///
/// let wlr = ProtocolSet::empty().with(Protocol::WlrForeignToplevelManagement);
/// assert_eq!(backend_for(&wlr), Some(DesktopBackend::Wlr));
/// let quire = wlr
///     .with(Protocol::ExtForeignToplevelList)
///     .with(Protocol::QuireDesktop);
/// assert_eq!(backend_for(&quire), Some(DesktopBackend::Quire));
/// assert_eq!(backend_for(&ProtocolSet::empty()), None);
/// ```
pub fn backend_for(offered: &ProtocolSet) -> Option<DesktopBackend> {
    let has = |p| offered.contains(p);
    if has(Protocol::QuireDesktop) && has(Protocol::ExtForeignToplevelList) {
        Some(DesktopBackend::Quire)
    } else if has(Protocol::WlrForeignToplevelManagement) {
        Some(DesktopBackend::Wlr)
    } else if has(Protocol::ExtForeignToplevelList) {
        Some(DesktopBackend::Ext)
    } else {
        None
    }
}
