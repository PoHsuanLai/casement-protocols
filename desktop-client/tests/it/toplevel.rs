//! The toplevel reducer.

use desktop_client::toplevel::{
    Announcement, ToplevelChange, ToplevelEvent, ToplevelFlag, ToplevelFlags,
};
use desktop_client::{OutputName, ToplevelIdentifier};

use crate::support::{run, tk, top, top_change};

fn announced(n: u64) -> Vec<desktop_client::DesktopEvent> {
    vec![
        top(ToplevelEvent::Opened(tk(n))),
        top(ToplevelEvent::Done(tk(n))),
    ]
}

#[test]
fn opened_is_pending_until_done() {
    let state = run([top(ToplevelEvent::Opened(tk(1)))]);
    let t = state.toplevels.get(tk(1)).expect("record");
    assert_eq!(t.announcement, Announcement::Pending);
    assert_eq!(state.toplevels.announced().count(), 0);

    let state = run(announced(1));
    assert_eq!(state.toplevels.announced().count(), 1);
}

#[test]
fn reopening_does_not_reset() {
    let state = run([
        top(ToplevelEvent::Opened(tk(1))),
        top_change(tk(1), ToplevelChange::Title("a".into())),
        top(ToplevelEvent::Opened(tk(1))),
    ]);
    assert_eq!(state.toplevels.get(tk(1)).unwrap().title, "a");
}

#[test]
fn changes_apply() {
    let state = run([
        top(ToplevelEvent::Opened(tk(1))),
        top_change(tk(1), ToplevelChange::Title("T".into())),
        top_change(tk(1), ToplevelChange::AppId("org.example".into())),
        top_change(
            tk(1),
            ToplevelChange::Identifier(ToplevelIdentifier::new("id-1")),
        ),
        top_change(
            tk(1),
            ToplevelChange::OutputEntered(OutputName::new("DP-1")),
        ),
        top_change(
            tk(1),
            ToplevelChange::OutputEntered(OutputName::new("DP-1")),
        ),
        top_change(
            tk(1),
            ToplevelChange::OutputEntered(OutputName::new("DP-2")),
        ),
        top_change(tk(1), ToplevelChange::OutputLeft(OutputName::new("DP-1"))),
        top_change(
            tk(1),
            ToplevelChange::Flags(ToplevelFlags::of([ToplevelFlag::Activated])),
        ),
        top(ToplevelEvent::Done(tk(1))),
    ]);
    let t = state.toplevels.get(tk(1)).unwrap();
    assert_eq!(t.title, "T");
    assert_eq!(t.app_id, "org.example");
    assert_eq!(t.identifier, Some(ToplevelIdentifier::new("id-1")));
    assert_eq!(t.outputs, vec![OutputName::new("DP-2")]);
    assert!(t.flags.contains(ToplevelFlag::Activated));
    assert_eq!(state.toplevels.activated().map(|t| t.key), Some(tk(1)));
    assert_eq!(
        state
            .toplevels
            .by_identifier(&ToplevelIdentifier::new("id-1"))
            .map(|t| t.key),
        Some(tk(1))
    );
}

#[test]
fn flags_replace_the_whole_set() {
    let state = run([
        top(ToplevelEvent::Opened(tk(1))),
        top_change(
            tk(1),
            ToplevelChange::Flags(ToplevelFlags::of([
                ToplevelFlag::Maximized,
                ToplevelFlag::Activated,
            ])),
        ),
        top_change(
            tk(1),
            ToplevelChange::Flags(ToplevelFlags::of([ToplevelFlag::Minimized])),
        ),
    ]);
    let t = state.toplevels.get(tk(1)).unwrap();
    assert!(t.flags.contains(ToplevelFlag::Minimized));
    assert!(!t.flags.contains(ToplevelFlag::Maximized));
    assert!(!t.flags.contains(ToplevelFlag::Activated));
}

#[test]
fn unknown_key_is_ignored() {
    let state = run([
        top_change(tk(9), ToplevelChange::Title("x".into())),
        top(ToplevelEvent::Done(tk(9))),
        top(ToplevelEvent::Closed(tk(9))),
    ]);
    assert!(state.toplevels.get(tk(9)).is_none());
}

#[test]
fn closed_removes_and_clears_parent_links() {
    let state = run([
        top(ToplevelEvent::Opened(tk(1))),
        top(ToplevelEvent::Opened(tk(2))),
        top_change(tk(2), ToplevelChange::Parent(Some(tk(1)))),
        top(ToplevelEvent::Closed(tk(1))),
    ]);
    assert!(state.toplevels.get(tk(1)).is_none());
    assert_eq!(state.toplevels.get(tk(2)).unwrap().parent, None);
}

#[test]
fn merged_moves_the_identifier_onto_the_managed_record() {
    let state = run([
        top(ToplevelEvent::Opened(tk(1))),
        top_change(
            tk(1),
            ToplevelChange::Identifier(ToplevelIdentifier::new("id-1")),
        ),
        top(ToplevelEvent::Opened(tk(2))),
        top_change(tk(2), ToplevelChange::Title("managed".into())),
        top(ToplevelEvent::Opened(tk(3))),
        top_change(tk(3), ToplevelChange::Parent(Some(tk(1)))),
        top(ToplevelEvent::Merged {
            from: tk(1),
            into: tk(2),
        }),
    ]);
    assert!(state.toplevels.get(tk(1)).is_none());
    let merged = state.toplevels.get(tk(2)).unwrap();
    assert_eq!(merged.identifier, Some(ToplevelIdentifier::new("id-1")));
    assert_eq!(merged.title, "managed");
    assert_eq!(state.toplevels.get(tk(3)).unwrap().parent, Some(tk(2)));
}

#[test]
fn merged_needs_both_records() {
    let only_from = run([
        top(ToplevelEvent::Opened(tk(1))),
        top(ToplevelEvent::Merged {
            from: tk(1),
            into: tk(2),
        }),
    ]);
    assert!(only_from.toplevels.get(tk(1)).is_some());

    let only_into = run([
        top(ToplevelEvent::Opened(tk(2))),
        top(ToplevelEvent::Merged {
            from: tk(1),
            into: tk(2),
        }),
    ]);
    assert!(only_into.toplevels.get(tk(2)).is_some());
}

#[test]
fn announced_iterates_in_creation_order() {
    let mut events = announced(3);
    events.extend(announced(1));
    events.extend(announced(2));
    let state = run(events);
    let keys: Vec<_> = state.toplevels.announced().map(|t| t.key).collect();
    assert_eq!(keys, vec![tk(1), tk(2), tk(3)]);
}

#[test]
fn flag_codes() {
    let cases = [
        (0u32, Some(ToplevelFlag::Maximized)),
        (1, Some(ToplevelFlag::Minimized)),
        (2, Some(ToplevelFlag::Activated)),
        (3, Some(ToplevelFlag::Fullscreen)),
        (4, None),
    ];
    for (code, expected) in cases {
        assert_eq!(ToplevelFlag::from_code(code), expected, "code {code}");
    }
}

#[test]
fn flags_from_a_wire_array() {
    let mut bytes = Vec::new();
    for code in [2u32, 9, 0] {
        bytes.extend_from_slice(&code.to_ne_bytes());
    }
    bytes.push(0xff); // a partial trailing word
    let flags = ToplevelFlags::from_array(&bytes);
    assert_eq!(
        flags,
        ToplevelFlags::of([ToplevelFlag::Activated, ToplevelFlag::Maximized])
    );
    assert_eq!(ToplevelFlags::from_array(&[]), ToplevelFlags::none());
}
