//! The workspace reducer: pending edits, atomic commit at done.

use std::collections::BTreeSet;

use desktop_client::OutputName;
use desktop_client::workspace::{
    GroupAbility, GroupChange, WorkspaceAbility, WorkspaceChange, WorkspaceEvent, WorkspaceFlag,
    coordinates_from_array,
};

use crate::support::{gk, run, wk, ws, ws_change};

fn set<T: Ord>(items: impl IntoIterator<Item = T>) -> BTreeSet<T> {
    items.into_iter().collect()
}

#[test]
fn nothing_is_visible_before_done() {
    let before = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::Name("one".into())),
    ]);
    assert_eq!(before.workspaces.workspaces().count(), 0);

    let after = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::Name("one".into())),
        ws(WorkspaceEvent::Done),
    ]);
    assert_eq!(after.workspaces.get(wk(1)).unwrap().name, "one");
}

#[test]
fn the_commit_is_atomic() {
    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::Name("one".into())),
        ws(WorkspaceEvent::Done),
        ws_change(wk(1), WorkspaceChange::Name("renamed".into())),
        ws(WorkspaceEvent::Opened(wk(2))),
    ]);
    // Edits since the last done are not visible yet.
    assert_eq!(state.workspaces.get(wk(1)).unwrap().name, "one");
    assert!(state.workspaces.get(wk(2)).is_none());

    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::Name("one".into())),
        ws(WorkspaceEvent::Done),
        ws_change(wk(1), WorkspaceChange::Name("renamed".into())),
        ws(WorkspaceEvent::Opened(wk(2))),
        ws(WorkspaceEvent::Done),
    ]);
    assert_eq!(state.workspaces.get(wk(1)).unwrap().name, "renamed");
    assert!(state.workspaces.get(wk(2)).is_some());
}

#[test]
fn properties_apply() {
    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::Id("ws-1".into())),
        ws_change(wk(1), WorkspaceChange::Coordinates(vec![1, 2])),
        ws_change(
            wk(1),
            WorkspaceChange::Flags(set([WorkspaceFlag::Active, WorkspaceFlag::Urgent])),
        ),
        ws_change(
            wk(1),
            WorkspaceChange::Abilities(set([WorkspaceAbility::Activate, WorkspaceAbility::Remove])),
        ),
        ws(WorkspaceEvent::Done),
    ]);
    let w = state.workspaces.get(wk(1)).unwrap();
    assert_eq!(w.id.as_deref(), Some("ws-1"));
    assert_eq!(w.coordinates, vec![1, 2]);
    assert_eq!(w.flags, set([WorkspaceFlag::Active, WorkspaceFlag::Urgent]));
    assert_eq!(
        w.abilities,
        set([WorkspaceAbility::Activate, WorkspaceAbility::Remove])
    );
    assert_eq!(state.workspaces.active().count(), 1);
}

#[test]
fn removed_drops_the_workspace_at_done() {
    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws(WorkspaceEvent::Done),
        ws(WorkspaceEvent::Removed(wk(1))),
    ]);
    assert!(state.workspaces.get(wk(1)).is_some());
    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws(WorkspaceEvent::Done),
        ws(WorkspaceEvent::Removed(wk(1))),
        ws(WorkspaceEvent::Done),
    ]);
    assert!(state.workspaces.get(wk(1)).is_none());
}

#[test]
fn groups_hold_outputs_and_members() {
    let state = run([
        ws(WorkspaceEvent::GroupOpened(gk(7))),
        ws(WorkspaceEvent::Group {
            key: gk(7),
            change: GroupChange::OutputEntered(OutputName::new("DP-1")),
        }),
        ws(WorkspaceEvent::Group {
            key: gk(7),
            change: GroupChange::OutputEntered(OutputName::new("DP-1")),
        }),
        ws(WorkspaceEvent::Group {
            key: gk(7),
            change: GroupChange::OutputEntered(OutputName::new("DP-2")),
        }),
        ws(WorkspaceEvent::Group {
            key: gk(7),
            change: GroupChange::OutputLeft(OutputName::new("DP-2")),
        }),
        ws(WorkspaceEvent::Group {
            key: gk(7),
            change: GroupChange::Abilities(set([GroupAbility::CreateWorkspace])),
        }),
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::GroupEntered(gk(7))),
        ws(WorkspaceEvent::Done),
    ]);
    let group = state.workspaces.groups().next().unwrap();
    assert_eq!(group.outputs, vec![OutputName::new("DP-1")]);
    assert_eq!(group.abilities, set([GroupAbility::CreateWorkspace]));
    assert_eq!(state.workspaces.get(wk(1)).unwrap().group, Some(gk(7)));
}

#[test]
fn leaving_another_group_changes_nothing() {
    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::GroupEntered(gk(1))),
        ws_change(wk(1), WorkspaceChange::GroupLeft(gk(2))),
        ws(WorkspaceEvent::Done),
    ]);
    assert_eq!(state.workspaces.get(wk(1)).unwrap().group, Some(gk(1)));

    let state = run([
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::GroupEntered(gk(1))),
        ws_change(wk(1), WorkspaceChange::GroupLeft(gk(1))),
        ws(WorkspaceEvent::Done),
    ]);
    assert_eq!(state.workspaces.get(wk(1)).unwrap().group, None);
}

#[test]
fn a_removed_group_unlinks_its_workspaces() {
    let state = run([
        ws(WorkspaceEvent::GroupOpened(gk(7))),
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::GroupEntered(gk(7))),
        ws(WorkspaceEvent::Done),
        ws(WorkspaceEvent::GroupRemoved(gk(7))),
        ws(WorkspaceEvent::Done),
    ]);
    assert_eq!(state.workspaces.groups().count(), 0);
    assert_eq!(state.workspaces.get(wk(1)).unwrap().group, None);
}

#[test]
fn unknown_keys_are_ignored_and_finished_changes_nothing() {
    let state = run([
        ws_change(wk(5), WorkspaceChange::Name("x".into())),
        ws(WorkspaceEvent::Group {
            key: gk(5),
            change: GroupChange::OutputLeft(OutputName::new("DP-1")),
        }),
        ws(WorkspaceEvent::Removed(wk(5))),
        ws(WorkspaceEvent::Opened(wk(1))),
        ws(WorkspaceEvent::Done),
        ws(WorkspaceEvent::Finished),
    ]);
    assert!(state.workspaces.get(wk(5)).is_none());
    assert!(state.workspaces.get(wk(1)).is_some());
}

#[test]
fn bitfields_decode() {
    let flags = [
        (0, set([])),
        (1, set([WorkspaceFlag::Active])),
        (5, set([WorkspaceFlag::Active, WorkspaceFlag::Hidden])),
        (0x8000_0002, set([WorkspaceFlag::Urgent])),
    ];
    for (bits, expected) in flags {
        assert_eq!(
            WorkspaceFlag::set_from_bits(bits),
            expected,
            "flags {bits:#x}"
        );
    }
    assert_eq!(
        WorkspaceAbility::set_from_bits(0b1010),
        set([WorkspaceAbility::Deactivate, WorkspaceAbility::Assign])
    );
    assert_eq!(
        GroupAbility::set_from_bits(1),
        set([GroupAbility::CreateWorkspace])
    );
    assert_eq!(GroupAbility::set_from_bits(2), set([]));
}

#[test]
fn coordinates_decode() {
    let mut bytes = Vec::new();
    for n in [3u32, 4] {
        bytes.extend_from_slice(&n.to_ne_bytes());
    }
    bytes.push(1);
    assert_eq!(coordinates_from_array(&bytes), vec![3, 4]);
    assert!(coordinates_from_array(&[]).is_empty());
}
