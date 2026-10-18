//! The Quire arm's reducer: apps, windows, Spaces, the active pair, the shell, the projection
//! onto toplevels and the linker.

use std::collections::BTreeSet;

use casement_protocols::vocab::{
    ActorKind, AppLifeKind, AppVisibility, AskId, Attention, Corner, DragOutcome, Emphasis,
    GlowCode, KeyGesture, LeaseState, MacMods, Memory, MenuSourceKind, Modality, OverviewMode,
    OverviewState, PlacementMark, Power, ProofKind, Responsiveness, ScreenEdge, SensitivityLevel,
    Shown, SpaceActivity, SpaceKind, SplitSide, SubjectKind, TrialId, WindowRole, WindowStateKind,
};
use desktop_client::quire::{
    AppChange, AppEvent, DisplayTrial, Lease, Linker, MenuSource, PermissionAsk, QuireEvent,
    ShellEvent, SpaceChange, SpaceEvent, SpacePlace, WindowChange, WindowEvent, WindowGeometry,
    WindowObject,
};
use desktop_client::toplevel::{Announcement, ToplevelChange, ToplevelEvent, ToplevelFlag};
use desktop_client::workspace::{WorkspaceChange, WorkspaceEvent, WorkspaceFlag};
use desktop_client::{
    AppId, AppSubject, DesktopEvent, DesktopState, OutputName, Rect, SpaceId, SurfaceKey,
    ToplevelIdentifier, reduce,
};

use crate::support::{ak, quire, run, sk, tk, top, top_change, window_change, wk, ws, ws_change};

fn app(event: AppEvent) -> DesktopEvent {
    quire(QuireEvent::App(event))
}

fn app_change(n: u64, change: AppChange) -> DesktopEvent {
    app(AppEvent::Changed { key: ak(n), change })
}

fn space(event: SpaceEvent) -> DesktopEvent {
    quire(QuireEvent::Space(event))
}

fn space_change(n: u64, change: SpaceChange) -> DesktopEvent {
    space(SpaceEvent::Changed { key: sk(n), change })
}

fn window(event: WindowEvent) -> DesktopEvent {
    quire(QuireEvent::Window(event))
}

fn shell(event: ShellEvent) -> DesktopEvent {
    quire(QuireEvent::Shell(event))
}

/// A toplevel with a key, announced.
fn toplevel(n: u64) -> Vec<DesktopEvent> {
    vec![
        top(ToplevelEvent::Opened(tk(n))),
        top_change(tk(n), ToplevelChange::Title("T".into())),
        top(ToplevelEvent::Done(tk(n))),
    ]
}

/// A toplevel and its Quire window, opened.
fn with_window(n: u64) -> Vec<DesktopEvent> {
    let mut events = toplevel(n);
    events.push(window(WindowEvent::Opened(tk(n))));
    events.push(window(WindowEvent::Done(tk(n))));
    events
}

#[test]
fn an_app_is_listed_after_its_done_with_what_its_burst_said() {
    let mut events = vec![
        app(AppEvent::Opened(ak(1))),
        app_change(
            1,
            AppChange::Identity {
                id: AppId::new("org.example.Notes"),
                proof: ProofKind::Scope,
                claimed_id: "notes".into(),
            },
        ),
        app_change(1, AppChange::Subject(AppSubject::new("/usr/bin/notes"))),
        app_change(1, AppChange::Life(AppLifeKind::Running)),
        app_change(1, AppChange::Visibility(AppVisibility::Hidden)),
        app_change(
            1,
            AppChange::Menu(MenuSource::new(
                MenuSourceKind::DbusMenu,
                ":1.5",
                "/a",
                "/m",
            )),
        ),
        app_change(1, AppChange::Attention(Attention::Critical)),
        app_change(1, AppChange::Mru(2)),
    ];
    let before = run(events.clone());
    assert_eq!(before.quire.apps().count(), 0, "pending until done");
    events.push(app(AppEvent::Done(ak(1))));
    let state = run(events);
    let a = state.quire.app(ak(1)).unwrap();
    assert_eq!(a.id, Some(AppId::new("org.example.Notes")));
    assert_eq!(a.proof, Some(ProofKind::Scope));
    assert_eq!(a.claimed_id, "notes");
    assert_eq!(a.subject, Some(AppSubject::new("/usr/bin/notes")));
    assert_eq!(a.life, AppLifeKind::Running);
    assert_eq!(a.visibility, AppVisibility::Hidden);
    assert_eq!(a.menu.kind, MenuSourceKind::DbusMenu);
    assert_eq!(a.menu.bus, ":1.5");
    assert_eq!(a.attention, Attention::Critical);
    assert_eq!(a.mru, Some(2));
    assert_eq!(a.announcement, Announcement::Announced);
    assert_eq!(
        state
            .quire
            .app_by_id(&AppId::new("org.example.Notes"))
            .map(|a| a.key),
        Some(ak(1))
    );
    assert!(state.quire.app_by_id(&AppId::new("other")).is_none());
}

#[test]
fn a_new_app_starts_launching_with_nothing_asked() {
    let state = run([app(AppEvent::Opened(ak(1)))]);
    let a = state.quire.app(ak(1)).unwrap();
    assert_eq!(a.life, AppLifeKind::Launching);
    assert_eq!(a.attention, Attention::None);
    assert_eq!(a.menu, MenuSource::none());
    assert_eq!(a.mru, None);
    assert_eq!(a.subject, None, "no subject until the compositor says one");
}

#[test]
fn a_change_to_something_unknown_changes_nothing() {
    let events = [
        app_change(9, AppChange::Mru(0)),
        app(AppEvent::Done(ak(9))),
        space_change(9, SpaceChange::Position(1)),
        space(SpaceEvent::Done(sk(9))),
        window_change(tk(9), WindowChange::Emphasis(Emphasis::Key)),
        window(WindowEvent::Done(tk(9))),
    ];
    assert_eq!(run(events), DesktopState::default());
}

#[test]
fn a_closed_app_leaves_the_active_pair_and_its_windows() {
    let mut events = with_window(1);
    events.extend([
        app(AppEvent::Opened(ak(5))),
        app(AppEvent::Done(ak(5))),
        window_change(tk(1), WindowChange::App(ak(5))),
        quire(QuireEvent::Active {
            app: Some(ak(5)),
            key: Some(tk(1)),
            actor: ActorKind::User,
        }),
    ]);
    let open = run(events.clone());
    assert_eq!(open.quire.windows_of(ak(5)).count(), 1);
    assert_eq!(open.quire.active().app, Some(ak(5)));
    events.push(app(AppEvent::Closed(ak(5))));
    let closed = run(events);
    assert!(closed.quire.app(ak(5)).is_none());
    assert_eq!(closed.quire.active().app, None);
    assert_eq!(
        closed.quire.active().key,
        Some(tk(1)),
        "the key window stays"
    );
    assert_eq!(closed.quire.window(tk(1)).unwrap().app, None);
}

/// (case, change, check)
type WindowCheck = fn(&desktop_client::quire::QuireWindow) -> bool;

fn row(case: &'static str, change: WindowChange, check: WindowCheck) -> Row {
    (case, change, check)
}

type Row = (&'static str, WindowChange, WindowCheck);

fn window_rows() -> Vec<Row> {
    vec![
        row("app", WindowChange::App(ak(3)), |w| w.app == Some(ak(3))),
        row(
            "role and modality",
            WindowChange::Role {
                role: WindowRole::Utility,
                modality: Modality::WindowModal,
            },
            |w| w.role == WindowRole::Utility && w.modality == Modality::WindowModal,
        ),
        row(
            "state and actor",
            WindowChange::State {
                kind: WindowStateKind::Tiled,
                actor: ActorKind::Companion,
            },
            |w| w.state == WindowStateKind::Tiled && w.actor == ActorKind::Companion,
        ),
        row(
            "on a space",
            WindowChange::Place(SpacePlace::On(sk(2))),
            |w| w.place == SpacePlace::On(sk(2)),
        ),
        row(
            "on all spaces",
            WindowChange::Place(SpacePlace::AllSpaces),
            |w| w.place == SpacePlace::AllSpaces,
        ),
        row(
            "output",
            WindowChange::Output(OutputName::new("DP-1")),
            |w| w.output == Some(OutputName::new("DP-1")),
        ),
        row(
            "geometry",
            WindowChange::Geometry(WindowGeometry::new(Rect::new(-5, 6, 700, 500), 180)),
            |w| w.geometry == Some(WindowGeometry::new(Rect::new(-5, 6, 700, 500), 180)),
        ),
        row("parent", WindowChange::Parent(Some(tk(1))), |w| {
            w.parent == Some(tk(1))
        }),
        row("emphasis", WindowChange::Emphasis(Emphasis::Main), |w| {
            w.emphasis == Emphasis::Main
        }),
        row(
            "sensitivity",
            WindowChange::Sensitivity(SensitivityLevel::CaptureHidden),
            |w| w.sensitivity == SensitivityLevel::CaptureHidden,
        ),
        row(
            "lease",
            WindowChange::Lease(Lease::new(LeaseState::Granted, GlowCode::Acting)),
            |w| w.lease == Lease::new(LeaseState::Granted, GlowCode::Acting),
        ),
        row(
            "responsiveness",
            WindowChange::Responsiveness(Responsiveness::NotResponding),
            |w| w.responsiveness == Responsiveness::NotResponding,
        ),
        row(
            "placement",
            WindowChange::Placement(PlacementMark::TopLeft),
            |w| w.placement == PlacementMark::TopLeft,
        ),
        row(
            "own menu path",
            WindowChange::MenuPath("/w/1".into()),
            |w| w.menu_path.as_deref() == Some("/w/1"),
        ),
        row(
            "an empty menu path means the app's",
            WindowChange::MenuPath(String::new()),
            |w| w.menu_path.is_none(),
        ),
    ]
}

#[test]
fn every_window_change_lands_on_its_field() {
    for (case, change, check) in window_rows() {
        let mut events = with_window(1);
        events.push(window_change(tk(1), change));
        let state = run(events);
        assert!(check(state.quire.window(tk(1)).unwrap()), "{case}");
    }
}

#[test]
fn a_new_window_has_quiet_defaults() {
    let w = run(with_window(1)).quire.window(tk(1)).unwrap().clone();
    assert_eq!(w.role, WindowRole::Document);
    assert_eq!(w.state, WindowStateKind::Floating);
    assert_eq!(w.emphasis, Emphasis::Inactive);
    assert_eq!(w.place, SpacePlace::Unknown);
    assert_eq!(w.lease, Lease::free());
    assert_eq!(w.announcement, Announcement::Announced);
}

#[test]
fn a_window_is_listed_after_its_done_and_goes_with_its_toplevel() {
    let mut events = toplevel(1);
    events.push(window(WindowEvent::Opened(tk(1))));
    assert_eq!(run(events.clone()).quire.windows().count(), 0);
    events.push(window(WindowEvent::Done(tk(1))));
    assert_eq!(run(events.clone()).quire.windows().count(), 1);
    events.push(top(ToplevelEvent::Closed(tk(1))));
    let state = run(events);
    assert!(state.quire.window(tk(1)).is_none());
    assert!(state.toplevels.get(tk(1)).is_none());
}

#[test]
fn a_closed_window_object_leaves_the_key_and_the_parent_links() {
    let mut events = with_window(1);
    events.extend(with_window(2));
    events.extend([
        window_change(tk(2), WindowChange::Parent(Some(tk(1)))),
        quire(QuireEvent::Active {
            app: None,
            key: Some(tk(1)),
            actor: ActorKind::User,
        }),
        window(WindowEvent::Closed(tk(1))),
    ]);
    let state = run(events);
    assert!(state.quire.window(tk(1)).is_none());
    assert_eq!(state.quire.active().key, None);
    assert_eq!(state.quire.window(tk(2)).unwrap().parent, None);
}

/// (case, window state, is the key window, expected flags)
type FlagRow = (&'static str, WindowStateKind, bool, Vec<ToplevelFlag>);

fn flag_row(
    case: &'static str,
    kind: WindowStateKind,
    key: bool,
    want: &[ToplevelFlag],
) -> FlagRow {
    (case, kind, key, want.to_vec())
}

#[test]
fn quire_state_is_projected_onto_the_toplevel_flags() {
    use ToplevelFlag::*;
    let rows = [
        flag_row("floating", WindowStateKind::Floating, false, &[]),
        flag_row("tiled", WindowStateKind::Tiled, false, &[]),
        flag_row(
            "filled is maximised",
            WindowStateKind::Filled,
            false,
            &[Maximized],
        ),
        flag_row(
            "full screen",
            WindowStateKind::FullScreen,
            false,
            &[Fullscreen],
        ),
        flag_row("minimised", WindowStateKind::Minimised, false, &[Minimized]),
        flag_row(
            "the key window",
            WindowStateKind::Floating,
            true,
            &[Activated],
        ),
        flag_row(
            "key and full screen",
            WindowStateKind::FullScreen,
            true,
            &[Activated, Fullscreen],
        ),
    ];
    for (case, kind, key, want) in rows {
        let mut events = with_window(1);
        events.push(window_change(
            tk(1),
            WindowChange::State {
                kind,
                actor: ActorKind::User,
            },
        ));
        if key {
            events.push(quire(QuireEvent::Active {
                app: None,
                key: Some(tk(1)),
                actor: ActorKind::User,
            }));
        }
        let state = run(events);
        let flags: BTreeSet<_> = state.toplevels.get(tk(1)).unwrap().flags.iter().collect();
        assert_eq!(flags, want.into_iter().collect(), "{case}");
    }
}

#[test]
fn the_key_moves_between_windows_and_the_flags_follow() {
    let mut events = with_window(1);
    events.extend(with_window(2));
    for key in [tk(1), tk(2)] {
        events.push(quire(QuireEvent::Active {
            app: None,
            key: Some(key),
            actor: ActorKind::User,
        }));
    }
    let state = run(events);
    assert_eq!(state.toplevels.activated().map(|t| t.key), Some(tk(2)));
    assert!(
        !state
            .toplevels
            .get(tk(1))
            .unwrap()
            .flags
            .contains(ToplevelFlag::Activated)
    );
}

#[test]
fn parent_and_output_are_projected_onto_the_toplevel() {
    let mut events = with_window(1);
    events.extend(with_window(2));
    events.extend([
        window_change(tk(2), WindowChange::Parent(Some(tk(1)))),
        window_change(tk(2), WindowChange::Output(OutputName::new("DP-2"))),
    ]);
    let t = run(events).toplevels.get(tk(2)).unwrap().clone();
    assert_eq!(t.parent, Some(tk(1)));
    assert_eq!(t.outputs, vec![OutputName::new("DP-2")]);
}

#[test]
fn a_toplevel_without_a_quire_window_keeps_its_own_flags() {
    let mut events = toplevel(1);
    events.push(top_change(
        tk(1),
        ToplevelChange::Flags(desktop_client::toplevel::ToplevelFlags::of([
            ToplevelFlag::Maximized,
        ])),
    ));
    events.extend(with_window(2));
    events.push(window_change(
        tk(2),
        WindowChange::State {
            kind: WindowStateKind::Minimised,
            actor: ActorKind::User,
        },
    ));
    let state = run(events);
    assert!(
        state
            .toplevels
            .get(tk(1))
            .unwrap()
            .flags
            .contains(ToplevelFlag::Maximized)
    );
}

#[test]
fn spaces_keep_their_order_per_output_and_a_removed_space_unplaces_its_windows() {
    let dp1 = OutputName::new("DP-1");
    let mut events = with_window(1);
    for (n, id, position, output) in [
        (1, "b", 1, "DP-1"),
        (2, "a", 0, "DP-1"),
        (3, "c", 0, "DP-2"),
    ] {
        events.extend([
            space(SpaceEvent::Opened(sk(n))),
            space_change(n, SpaceChange::Id(SpaceId::new(id))),
            space_change(n, SpaceChange::Position(position)),
            space_change(n, SpaceChange::Output(OutputName::new(output))),
            space(SpaceEvent::Done(sk(n))),
        ]);
    }
    events.push(window_change(
        tk(1),
        WindowChange::Place(SpacePlace::On(sk(1))),
    ));
    let state = run(events.clone());
    let order: Vec<_> = state
        .quire
        .spaces_on(&dp1)
        .iter()
        .map(|s| s.id.clone().unwrap())
        .collect();
    assert_eq!(order, vec![SpaceId::new("a"), SpaceId::new("b")]);
    assert_eq!(
        state.quire.space_by_id(&SpaceId::new("c")).map(|s| s.key),
        Some(sk(3))
    );
    events.push(space(SpaceEvent::Closed(sk(1))));
    let after = run(events);
    assert!(after.quire.space(sk(1)).is_none());
    assert_eq!(
        after.quire.window(tk(1)).unwrap().place,
        SpacePlace::Unknown
    );
}

#[test]
fn space_fields_land_and_an_empty_occupant_is_none() {
    let state = run([
        space(SpaceEvent::Opened(sk(1))),
        space_change(1, SpaceChange::Kind(SpaceKind::Agent)),
        space_change(1, SpaceChange::Occupant("task-7".into())),
        space_change(1, SpaceChange::Activity(SpaceActivity::AgentRunning)),
        space(SpaceEvent::Done(sk(1))),
        space(SpaceEvent::Opened(sk(2))),
        space_change(2, SpaceChange::Occupant(String::new())),
        space(SpaceEvent::Done(sk(2))),
    ]);
    let agent = state.quire.space(sk(1)).unwrap();
    assert_eq!(agent.kind, SpaceKind::Agent);
    assert_eq!(agent.occupant.as_deref(), Some("task-7"));
    assert_eq!(agent.activity, SpaceActivity::AgentRunning);
    assert_eq!(state.quire.space(sk(2)).unwrap().occupant, None);
}

#[test]
fn a_space_is_shown_when_its_workspace_is_active() {
    let state = run([
        space(SpaceEvent::Opened(sk(1))),
        space_change(1, SpaceChange::Id(SpaceId::new("sp-1"))),
        space(SpaceEvent::Done(sk(1))),
        space(SpaceEvent::Opened(sk(2))),
        space_change(2, SpaceChange::Id(SpaceId::new("sp-2"))),
        space(SpaceEvent::Done(sk(2))),
        ws(WorkspaceEvent::Opened(wk(1))),
        ws_change(wk(1), WorkspaceChange::Id("sp-1".into())),
        ws_change(
            wk(1),
            WorkspaceChange::Flags(BTreeSet::from([WorkspaceFlag::Active])),
        ),
        ws(WorkspaceEvent::Opened(wk(2))),
        ws_change(wk(2), WorkspaceChange::Id("sp-2".into())),
        ws(WorkspaceEvent::Done),
    ]);
    assert_eq!(state.space_shown(sk(1)), Shown::Shown);
    assert_eq!(state.space_shown(sk(2)), Shown::Hidden);
    assert_eq!(state.space_shown(sk(9)), Shown::Hidden);
}

#[test]
fn the_active_pair_is_replaced_whole() {
    let state = run([
        quire(QuireEvent::Active {
            app: Some(ak(1)),
            key: Some(tk(1)),
            actor: ActorKind::User,
        }),
        quire(QuireEvent::Active {
            app: None,
            key: None,
            actor: ActorKind::Cli,
        }),
    ]);
    let active = state.quire.active();
    assert_eq!(
        (active.app, active.key, active.actor),
        (None, None, ActorKind::Cli)
    );
}

#[test]
fn modifiers_are_kept_with_the_surface_they_were_reported_for() {
    let state = run([shell(ShellEvent::Modifiers {
        surface: Some(SurfaceKey::new(4)),
        mods: MacMods::COMMAND.with(MacMods::SHIFT),
    })]);
    let s = state.quire.shell();
    assert!(s.mods.holds(MacMods::COMMAND));
    assert_eq!(s.mods_surface, Some(SurfaceKey::new(4)));
    let released = reduce(
        state,
        shell(ShellEvent::Modifiers {
            surface: None,
            mods: MacMods::NONE,
        }),
    );
    assert_eq!(released.quire.shell().mods, MacMods::NONE);
}

#[test]
fn the_switcher_collects_its_apps_then_shows_and_clears() {
    let apps = [("a", 0), ("b", 1), ("c", 2)].map(|(id, index)| {
        shell(ShellEvent::SwitcherApp {
            index,
            app: AppId::new(id),
        })
    });
    let mut events: Vec<_> = apps.into();
    assert!(!run(events.clone()).quire.shell().switcher.is_shown());
    events.push(shell(ShellEvent::SwitcherShown { selected: 1 }));
    let shown = run(events.clone());
    let switcher = &shown.quire.shell().switcher;
    assert_eq!(switcher.apps().len(), 3);
    assert_eq!(switcher.selected(), Some(1));
    events.push(shell(ShellEvent::SwitcherSelected(2)));
    assert_eq!(
        run(events.clone()).quire.shell().switcher.selected(),
        Some(2)
    );
    events.push(shell(ShellEvent::SwitcherHidden));
    let hidden = run(events);
    assert!(hidden.quire.shell().switcher.apps().is_empty());
    assert!(!hidden.quire.shell().switcher.is_shown());
}

#[test]
fn a_second_switcher_list_replaces_the_first() {
    let state = run([
        shell(ShellEvent::SwitcherApp {
            index: 0,
            app: AppId::new("a"),
        }),
        shell(ShellEvent::SwitcherApp {
            index: 1,
            app: AppId::new("b"),
        }),
        shell(ShellEvent::SwitcherApp {
            index: 0,
            app: AppId::new("z"),
        }),
    ]);
    assert_eq!(state.quire.shell().switcher.apps(), &[AppId::new("z")]);
}

#[test]
fn a_split_choice_collects_its_candidates_and_ends_with_its_first_window() {
    let id = ToplevelIdentifier::new;
    let choose = shell(ShellEvent::SplitChoose {
        first: id("w-1"),
        side: SplitSide::Left,
    });
    let candidate = |name: &str| shell(ShellEvent::SplitCandidate(ToplevelIdentifier::new(name)));
    let events = vec![choose, candidate("w-2"), candidate("w-3")];
    let offered = run(events.clone());
    let choice = offered
        .quire
        .shell()
        .split_choice
        .as_ref()
        .expect("offered");
    assert_eq!(choice.first(), &id("w-1"));
    assert_eq!(choice.side(), SplitSide::Left);
    assert_eq!(choice.candidates(), &[id("w-2"), id("w-3")]);

    // The end of another window's choosing leaves this one alone; its own takes it away.
    let mut other = events.clone();
    other.push(shell(ShellEvent::SplitChoiceEnd(id("w-9"))));
    assert!(run(other).quire.shell().split_choice.is_some());
    let mut ended = events;
    ended.push(shell(ShellEvent::SplitChoiceEnd(id("w-1"))));
    assert!(run(ended).quire.shell().split_choice.is_none());
}

#[test]
fn mission_control_is_shown_on_the_outputs_the_compositor_names() {
    let output = |name: &str| OutputName::new(name);
    let shown = |name: &str, shown| {
        shell(ShellEvent::OverviewOutput {
            output: output(name),
            shown,
        })
    };
    let open = shell(ShellEvent::Overview {
        mode: OverviewMode::MissionControl,
        state: OverviewState::Open,
        progress: 1000,
    });
    // A version 1 compositor names no output: every output counts while it is not closed.
    let old = run([open.clone()]);
    assert!(old.quire.shell().overview.shown_on(&output("DP-1")));
    let state = run([
        shown("DP-1", Shown::Shown),
        shown("DP-2", Shown::Hidden),
        open,
    ]);
    let overview = &state.quire.shell().overview;
    assert!(overview.shown_on(&output("DP-1")));
    assert!(!overview.shown_on(&output("DP-2")));
}

#[test]
fn the_drop_target_follows_the_dragged_window_and_ends_empty() {
    let target = |space: Option<&str>| {
        shell(ShellEvent::DropTarget {
            output: OutputName::new("DP-1"),
            space: space.map(SpaceId::new),
        })
    };
    let over = run([target(Some("s-2"))]);
    assert_eq!(
        over.quire
            .shell()
            .overview
            .drop_target(&OutputName::new("DP-1")),
        Some(&SpaceId::new("s-2"))
    );
    let after = run([target(Some("s-2")), target(None)]);
    assert!(
        after
            .quire
            .shell()
            .overview
            .drop_target(&OutputName::new("DP-1"))
            .is_none()
    );
}

#[test]
fn the_new_space_target_is_lit_while_the_dragged_window_is_over_it() {
    let over = |over| {
        shell(ShellEvent::DropTargetNew {
            output: OutputName::new("DP-1"),
            over,
        })
    };
    let lit = run([over(Shown::Shown)]);
    let dp1 = OutputName::new("DP-1");
    assert!(lit.quire.shell().overview.over_new_space(&dp1));
    assert!(
        !lit.quire
            .shell()
            .overview
            .over_new_space(&OutputName::new("DP-2"))
    );
    let after = run([over(Shown::Shown), over(Shown::Hidden)]);
    assert!(!after.quire.shell().overview.over_new_space(&dp1));
}

#[test]
fn a_window_drag_is_under_way_from_its_begin_to_its_end_and_remembers_the_outcome() {
    let begin = shell(ShellEvent::WindowDragBegin {
        output: OutputName::new("DP-1"),
        window: ToplevelIdentifier::new("w-1"),
    });
    let end = |outcome| {
        shell(ShellEvent::WindowDragEnd {
            output: OutputName::new("DP-1"),
            window: ToplevelIdentifier::new("w-1"),
            outcome,
        })
    };
    let dp1 = OutputName::new("DP-1");
    let during = run([begin.clone()]);
    let overview = &during.quire.shell().overview;
    assert_eq!(
        overview.dragged_window(&dp1),
        Some(&ToplevelIdentifier::new("w-1"))
    );
    assert!(overview.dragged_window(&OutputName::new("DP-2")).is_none());
    assert_eq!(overview.last_drag_outcome(&dp1), None);
    let after = run([begin.clone(), end(DragOutcome::DroppedOnNew)]);
    let overview = &after.quire.shell().overview;
    assert!(overview.dragged_window(&dp1).is_none());
    assert_eq!(
        overview.last_drag_outcome(&dp1),
        Some(DragOutcome::DroppedOnNew)
    );
    let again = run([begin.clone(), end(DragOutcome::Cancelled), begin]);
    assert_eq!(again.quire.shell().overview.last_drag_outcome(&dp1), None);
}

#[test]
fn a_candidate_with_no_choosing_is_dropped() {
    let state = run([shell(ShellEvent::SplitCandidate(ToplevelIdentifier::new(
        "w-2",
    )))]);
    assert!(state.quire.shell().split_choice.is_none());
}

#[test]
fn the_overview_layout_is_applied_at_its_done() {
    let slot = |id: &str, x: i32| ShellEvent::OverviewSlot {
        window: ToplevelIdentifier::new(id),
        rect: Rect::new(x, 0, 100, 80),
    };
    let mut events = vec![
        shell(ShellEvent::Overview {
            mode: OverviewMode::AppExpose,
            state: OverviewState::Entering,
            progress: 400,
        }),
        shell(slot("w1", 0)),
        shell(slot("w2", 110)),
        shell(ShellEvent::OverviewSpace {
            space: SpaceId::new("sp-1"),
            rect: Rect::new(0, 0, 50, 30),
        }),
    ];
    let half = run(events.clone());
    let overview = &half.quire.shell().overview;
    assert_eq!(overview.state(), OverviewState::Entering);
    assert_eq!(overview.mode(), OverviewMode::AppExpose);
    assert_eq!(overview.progress(), 400);
    assert!(overview.slots().is_empty(), "no half layout");
    events.push(shell(ShellEvent::OverviewDone));
    let done = run(events.clone());
    assert_eq!(done.quire.shell().overview.slots().len(), 2);
    assert_eq!(done.quire.shell().overview.spaces().len(), 1);
    // The next batch replaces the layout at its own done.
    events.extend([shell(slot("w3", 0)), shell(ShellEvent::OverviewDone)]);
    let next = run(events);
    let slots = next.quire.shell().overview.slots();
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].window, ToplevelIdentifier::new("w3"));
    assert!(next.quire.shell().overview.spaces().is_empty());
}

#[test]
fn reveals_are_per_edge_and_hidden_by_default() {
    let state = run([
        shell(ShellEvent::Reveal {
            edge: ScreenEdge::Top,
            shown: Shown::Shown,
        }),
        shell(ShellEvent::Reveal {
            edge: ScreenEdge::Bottom,
            shown: Shown::Shown,
        }),
        shell(ShellEvent::Reveal {
            edge: ScreenEdge::Top,
            shown: Shown::Hidden,
        }),
    ]);
    let reveals = &state.quire.shell().reveals;
    assert_eq!(reveals.at(ScreenEdge::Top), Shown::Hidden);
    assert_eq!(reveals.at(ScreenEdge::Bottom), Shown::Shown);
    assert_eq!(reveals.at(ScreenEdge::Left), Shown::Hidden);
}

#[test]
fn moments_change_no_state() {
    let moments = [
        ShellEvent::KeyGesture(KeyGesture::Summon),
        ShellEvent::HotCorner(Corner::TopLeft),
        ShellEvent::MenuFlash {
            app: AppId::new("a"),
            item: 2,
        },
        ShellEvent::Bell(AppId::new("a")),
        ShellEvent::Reopen(AppId::new("a")),
        ShellEvent::WindowMenu {
            window: ToplevelIdentifier::new("w-1"),
            at: desktop_client::Point::new(40, 52),
            enabled: 0b11,
        },
        ShellEvent::PermissionAsk(PermissionAsk::new(
            AskId::new(1),
            Power::Screen,
            SubjectKind::Program,
            "/usr/bin/grim",
            Memory::Remembered,
        )),
        ShellEvent::PermissionWithdraw(AskId::new(1)),
        ShellEvent::DisplaysTrial(DisplayTrial::new(TrialId::new(1), 15)),
        ShellEvent::DisplaysTrialEnd(TrialId::new(1)),
        ShellEvent::Killed,
    ];
    for moment in moments {
        let before = DesktopState::default();
        let after = reduce(before.clone(), shell(moment.clone()));
        assert_eq!(after, before, "{moment:?}");
    }
}

#[test]
fn a_display_trial_says_how_long_the_person_has() {
    let trial = DisplayTrial::new(TrialId::new(2), 15);
    assert_eq!(trial.within(), std::time::Duration::from_secs(15));
}

// The linker.

fn obj(n: u64) -> WindowObject {
    WindowObject::new(n)
}

fn id(text: &str) -> ToplevelIdentifier {
    ToplevelIdentifier::new(text)
}

fn state_change() -> WindowChange {
    WindowChange::State {
        kind: WindowStateKind::Filled,
        actor: ActorKind::User,
    }
}

#[test]
fn the_handle_first_then_the_window_links_at_the_window_identifier() {
    let mut linker = Linker::default();
    assert!(linker.listed(tk(1), id("a")).is_empty());
    let events = linker.identified(obj(9), id("a"));
    assert_eq!(events, vec![QuireEvent::Window(WindowEvent::Opened(tk(1)))]);
    assert_eq!(linker.key_of(obj(9)), Some(tk(1)));
    assert_eq!(linker.object_of(tk(1)), Some(obj(9)));
    assert_eq!(
        linker.change(obj(9), state_change()),
        vec![QuireEvent::Window(WindowEvent::Changed {
            key: tk(1),
            change: state_change()
        })]
    );
    assert_eq!(
        linker.done(obj(9)),
        vec![QuireEvent::Window(WindowEvent::Done(tk(1)))]
    );
}

#[test]
fn the_window_first_holds_its_events_until_the_handle_names_it() {
    let mut linker = Linker::default();
    assert!(
        linker.change(obj(9), state_change()).is_empty(),
        "before the identifier"
    );
    assert!(linker.identified(obj(9), id("a")).is_empty());
    assert!(
        linker
            .change(obj(9), WindowChange::Emphasis(Emphasis::Key))
            .is_empty()
    );
    assert!(linker.done(obj(9)).is_empty());
    assert_eq!(linker.key_of(obj(9)), None);
    let events = linker.listed(tk(4), id("a"));
    let want = vec![
        QuireEvent::Window(WindowEvent::Opened(tk(4))),
        QuireEvent::Window(WindowEvent::Changed {
            key: tk(4),
            change: state_change(),
        }),
        QuireEvent::Window(WindowEvent::Changed {
            key: tk(4),
            change: WindowChange::Emphasis(Emphasis::Key),
        }),
        QuireEvent::Window(WindowEvent::Done(tk(4))),
    ];
    assert_eq!(events, want, "in the order they arrived");
}

#[test]
fn windows_link_to_the_handle_of_their_own_identifier() {
    let mut linker = Linker::default();
    linker.identified(obj(1), id("a"));
    linker.identified(obj(2), id("b"));
    linker.listed(tk(20), id("b"));
    assert_eq!(linker.key_of(obj(1)), None);
    assert_eq!(linker.key_of(obj(2)), Some(tk(20)));
    linker.listed(tk(10), id("a"));
    assert_eq!(linker.key_of(obj(1)), Some(tk(10)));
}

#[test]
fn a_closed_window_reports_once_and_a_waiting_one_is_forgotten() {
    let mut linker = Linker::default();
    linker.listed(tk(1), id("a"));
    linker.identified(obj(9), id("a"));
    assert_eq!(
        linker.closed(obj(9)),
        vec![QuireEvent::Window(WindowEvent::Closed(tk(1)))]
    );
    assert!(linker.closed(obj(9)).is_empty());
    assert_eq!(linker.object_of(tk(1)), None);

    linker.identified(obj(10), id("never"));
    assert!(linker.closed(obj(10)).is_empty());
    assert!(
        linker.listed(tk(2), id("never")).is_empty(),
        "nothing is waiting any more"
    );
}

#[test]
fn a_second_identifier_for_a_linked_window_is_ignored() {
    let mut linker = Linker::default();
    linker.listed(tk(1), id("a"));
    linker.identified(obj(9), id("a"));
    assert!(linker.identified(obj(9), id("b")).is_empty());
    assert_eq!(linker.key_of(obj(9)), Some(tk(1)));
}

#[test]
fn a_handle_that_closed_no_longer_links_new_windows() {
    let mut linker = Linker::default();
    linker.listed(tk(1), id("a"));
    linker.unlisted(tk(1));
    assert!(linker.identified(obj(9), id("a")).is_empty());
    assert_eq!(linker.key_of(obj(9)), None);
}

#[test]
fn linker_output_reduces_into_a_window_under_its_toplevel() {
    let mut linker = Linker::default();
    let mut events = toplevel(1);
    linker.identified(obj(9), id("a"));
    linker.change(obj(9), state_change());
    linker.done(obj(9));
    events.extend(linker.listed(tk(1), id("a")).into_iter().map(quire));
    let state = run(events);
    let w = state.quire.window(tk(1)).unwrap();
    assert_eq!(w.state, WindowStateKind::Filled);
    assert_eq!(w.announcement, Announcement::Announced);
    assert!(
        state
            .toplevels
            .get(tk(1))
            .unwrap()
            .flags
            .contains(ToplevelFlag::Maximized)
    );
}
