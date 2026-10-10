//! The backend choice, from globals alone.

use desktop_client::{DesktopBackend, DesktopState, Protocol, ProtocolSet, backend_for};

fn offering(protocols: &[Protocol]) -> ProtocolSet {
    protocols
        .iter()
        .fold(ProtocolSet::empty(), |set, p| set.with(*p))
}

/// (case, offered, backend)
fn cases() -> Vec<(&'static str, ProtocolSet, Option<DesktopBackend>)> {
    use DesktopBackend::*;
    use Protocol::*;
    vec![
        ("nothing", offering(&[]), None),
        (
            "ext list alone",
            offering(&[ExtForeignToplevelList]),
            Some(Ext),
        ),
        (
            "wlr alone",
            offering(&[WlrForeignToplevelManagement]),
            Some(Wlr),
        ),
        (
            "ext plus wlr",
            offering(&[ExtForeignToplevelList, WlrForeignToplevelManagement]),
            Some(Wlr),
        ),
        (
            "quire desktop without the ext list",
            offering(&[QuireDesktop]),
            None,
        ),
        (
            "quire desktop and wlr without the ext list",
            offering(&[QuireDesktop, WlrForeignToplevelManagement]),
            Some(Wlr),
        ),
        (
            "quire desktop and the ext list",
            offering(&[QuireDesktop, ExtForeignToplevelList]),
            Some(Quire),
        ),
        (
            "quire wins over wlr",
            offering(&[
                QuireDesktop,
                ExtForeignToplevelList,
                WlrForeignToplevelManagement,
            ]),
            Some(Quire),
        ),
        (
            "the shell global alone chooses nothing",
            offering(&[QuireShell]),
            None,
        ),
        (
            "the window manager global does not choose the arm",
            offering(&[QuireWindow, ExtForeignToplevelList]),
            Some(Ext),
        ),
    ]
}

#[test]
fn the_backend_is_chosen_from_the_offered_globals() {
    for (case, offered, want) in cases() {
        assert_eq!(backend_for(&offered), want, "{case}");
        assert_eq!(DesktopState::new(offered).backend(), want, "{case}");
    }
}

#[test]
fn the_choice_reads_interface_names_from_the_registry() {
    let offered = ProtocolSet::from_interfaces([
        "wl_compositor",
        "ext_foreign_toplevel_list_v1",
        "quire_desktop_manager_v1",
        "quire_shell_v1",
    ]);
    assert_eq!(backend_for(&offered), Some(DesktopBackend::Quire));
    assert!(offered.contains(Protocol::QuireShell));
}
