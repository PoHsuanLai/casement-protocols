//! The output reducer, configuration validation and completion.

use desktop_client::output::{
    AdaptiveSync, ConfigError, ConfigOutcome, ConfigPurpose, Enabled, HeadChange,
    HeadChangeRequest, HeadConfig, HeadSettings, ModeChange, ModeChoice, OutputConfig, OutputEvent,
    Preferred, RefreshMilliHz, ScaleMilli, Transform, complete,
};
use desktop_client::{ConfigId, DesktopState, OutputName, Point, Size};

use crate::support::{head_change, hk, mk, mode_change, out, run};

/// Two heads: `DP-1` (modes 10, 11; 10 current and preferred) enabled, `HDMI-1` (mode 20) off.
fn two_heads() -> DesktopState {
    run([
        out(OutputEvent::HeadOpened(hk(1))),
        head_change(hk(1), HeadChange::Name(OutputName::new("DP-1"))),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(10),
        }),
        mode_change(mk(10), ModeChange::Size(Size::new(1920, 1080))),
        mode_change(mk(10), ModeChange::Refresh(RefreshMilliHz::new(60_000))),
        mode_change(mk(10), ModeChange::Preferred(Preferred::Yes)),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(11),
        }),
        mode_change(mk(11), ModeChange::Size(Size::new(1280, 720))),
        head_change(hk(1), HeadChange::Enabled(Enabled::On)),
        head_change(hk(1), HeadChange::CurrentMode(mk(10))),
        head_change(hk(1), HeadChange::Scale(ScaleMilli::new(1500))),
        out(OutputEvent::HeadOpened(hk(2))),
        head_change(hk(2), HeadChange::Name(OutputName::new("HDMI-1"))),
        out(OutputEvent::ModeOpened {
            head: hk(2),
            mode: mk(20),
        }),
        out(OutputEvent::Done { serial: 7 }),
    ])
}

fn enable(head: u64, settings: HeadSettings) -> HeadConfig {
    HeadConfig::new(hk(head), HeadChangeRequest::Enable(settings))
}

#[test]
fn heads_and_modes_build_up() {
    let state = two_heads();
    let outputs = &state.outputs;
    assert_eq!(outputs.serial(), Some(7));
    assert_eq!(outputs.heads().count(), 2);
    let dp = outputs.by_name(&OutputName::new("DP-1")).unwrap();
    assert_eq!(dp.key, hk(1));
    assert_eq!(dp.enabled, Enabled::On);
    assert_eq!(dp.modes.len(), 2);
    let current = dp.current().unwrap();
    assert_eq!(current.size, Size::new(1920, 1080));
    assert_eq!(current.refresh, RefreshMilliHz::new(60_000));
    assert_eq!(current.preferred, Preferred::Yes);
    assert_eq!(dp.scale, ScaleMilli::new(1500));
    assert!(outputs.by_name(&OutputName::new("nope")).is_none());
}

#[test]
fn optional_properties_are_set_by_their_events() {
    let state = run([
        out(OutputEvent::HeadOpened(hk(1))),
        head_change(hk(1), HeadChange::Make("Acme".into())),
        head_change(hk(1), HeadChange::Model("M1".into())),
        head_change(hk(1), HeadChange::SerialNumber("S".into())),
        head_change(hk(1), HeadChange::PhysicalSize(Size::new(600, 340))),
        head_change(hk(1), HeadChange::Position(Point::new(10, 20))),
        head_change(hk(1), HeadChange::Transform(Transform::Rotate90)),
        head_change(hk(1), HeadChange::AdaptiveSync(AdaptiveSync::Enabled)),
        head_change(hk(1), HeadChange::Description("desc".into())),
    ]);
    let head = state.outputs.get(hk(1)).unwrap();
    assert_eq!(head.make.as_deref(), Some("Acme"));
    assert_eq!(head.model.as_deref(), Some("M1"));
    assert_eq!(head.serial_number.as_deref(), Some("S"));
    assert_eq!(head.physical_size_mm, Some(Size::new(600, 340)));
    assert_eq!(head.position, Point::new(10, 20));
    assert_eq!(head.transform, Transform::Rotate90);
    assert_eq!(head.adaptive_sync, Some(AdaptiveSync::Enabled));
    assert_eq!(head.description, "desc");
}

#[test]
fn disabling_clears_the_current_mode() {
    let mut events = vec![
        out(OutputEvent::HeadOpened(hk(1))),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(10),
        }),
        head_change(hk(1), HeadChange::Enabled(Enabled::On)),
        head_change(hk(1), HeadChange::CurrentMode(mk(10))),
    ];
    assert_eq!(
        run(events.clone()).outputs.get(hk(1)).unwrap().current_mode,
        Some(mk(10))
    );
    events.push(head_change(hk(1), HeadChange::Enabled(Enabled::Off)));
    assert_eq!(run(events).outputs.get(hk(1)).unwrap().current_mode, None);
}

#[test]
fn a_finished_mode_goes_from_its_head() {
    let mut events = vec![
        out(OutputEvent::HeadOpened(hk(1))),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(10),
        }),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(11),
        }),
        head_change(hk(1), HeadChange::CurrentMode(mk(10))),
    ];
    events.push(mode_change(mk(10), ModeChange::Finished));
    let state = run(events);
    let head = state.outputs.get(hk(1)).unwrap();
    assert_eq!(head.modes.len(), 1);
    assert_eq!(head.modes[0].key, mk(11));
    assert_eq!(head.current_mode, None);
}

#[test]
fn a_mode_opened_twice_is_kept_once() {
    let state = run([
        out(OutputEvent::HeadOpened(hk(1))),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(10),
        }),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(10),
        }),
    ]);
    assert_eq!(state.outputs.get(hk(1)).unwrap().modes.len(), 1);
}

#[test]
fn head_finished_and_manager_finished_drop() {
    let state = run([
        out(OutputEvent::HeadOpened(hk(1))),
        out(OutputEvent::HeadOpened(hk(2))),
        out(OutputEvent::Done { serial: 3 }),
        out(OutputEvent::HeadFinished(hk(1))),
    ]);
    assert!(state.outputs.get(hk(1)).is_none());
    assert!(state.outputs.get(hk(2)).is_some());
    assert_eq!(state.outputs.serial(), Some(3));

    let state = run([
        out(OutputEvent::HeadOpened(hk(2))),
        out(OutputEvent::Done { serial: 3 }),
        out(OutputEvent::Finished),
    ]);
    assert_eq!(state.outputs.heads().count(), 0);
    assert_eq!(state.outputs.serial(), None);
}

#[test]
fn config_outcomes_leave_the_heads_alone() {
    let before = two_heads();
    for outcome in [
        ConfigOutcome::Succeeded,
        ConfigOutcome::Failed,
        ConfigOutcome::Cancelled,
    ] {
        let after = desktop_client::reduce(
            before.clone(),
            out(OutputEvent::ConfigResolved {
                id: ConfigId::new(1),
                purpose: ConfigPurpose::Apply,
                outcome,
            }),
        );
        assert_eq!(after, before, "{outcome:?}");
    }
}

#[test]
fn unknown_keys_are_ignored() {
    let state = run([
        head_change(hk(9), HeadChange::Name(OutputName::new("x"))),
        mode_change(mk(9), ModeChange::Size(Size::new(1, 1))),
        out(OutputEvent::ModeOpened {
            head: hk(9),
            mode: mk(9),
        }),
    ]);
    assert_eq!(state.outputs.heads().count(), 0);
}

#[test]
fn unit_codes() {
    for code in 0..8 {
        assert_eq!(Transform::from_code(code).unwrap().code(), code);
    }
    assert_eq!(Transform::from_code(8), None);
    assert_eq!(Enabled::from_wire(0), Enabled::Off);
    assert_eq!(Enabled::from_wire(-1), Enabled::On);
    assert_eq!(AdaptiveSync::from_code(1), AdaptiveSync::Enabled);
    assert_eq!(AdaptiveSync::from_code(0), AdaptiveSync::Disabled);
    assert_eq!(ScaleMilli::from_f64(1.5), ScaleMilli::new(1500));
    assert_eq!(ScaleMilli::from_f64(-2.0), ScaleMilli::new(0));
    assert_eq!(ScaleMilli::new(1250).to_f64(), 1.25);
}

#[test]
fn validate_table() {
    let state = two_heads();
    let custom = |w, h, hz| {
        enable(
            1,
            HeadSettings::keep().with_mode(ModeChoice::Custom {
                size: Size::new(w, h),
                refresh: RefreshMilliHz::new(hz),
            }),
        )
    };
    let with_mode = |head, mode| {
        enable(
            head,
            HeadSettings::keep().with_mode(ModeChoice::Existing(mk(mode))),
        )
    };
    let cases: Vec<(OutputConfig, Result<(), ConfigError>)> = vec![
        (OutputConfig::new(vec![]), Err(ConfigError::Empty)),
        (
            OutputConfig::new(vec![enable(9, HeadSettings::keep())]),
            Err(ConfigError::UnknownHead(hk(9))),
        ),
        (
            OutputConfig::new(vec![
                enable(1, HeadSettings::keep()),
                enable(1, HeadSettings::keep()),
            ]),
            Err(ConfigError::DuplicateHead(hk(1))),
        ),
        (
            OutputConfig::new(vec![
                enable(1, HeadSettings::keep()),
                HeadConfig::new(hk(1), HeadChangeRequest::Disable),
            ]),
            Err(ConfigError::DuplicateHead(hk(1))),
        ),
        (
            OutputConfig::new(vec![with_mode(1, 99)]),
            Err(ConfigError::UnknownMode(mk(99))),
        ),
        (
            OutputConfig::new(vec![with_mode(1, 20)]),
            Err(ConfigError::ModeOfOtherHead {
                head: hk(1),
                mode: mk(20),
            }),
        ),
        (
            OutputConfig::new(vec![custom(0, 1080, 60_000)]),
            Err(ConfigError::InvalidCustomMode(hk(1))),
        ),
        (
            OutputConfig::new(vec![custom(1920, 0, 60_000)]),
            Err(ConfigError::InvalidCustomMode(hk(1))),
        ),
        (
            OutputConfig::new(vec![custom(1920, 1080, 0)]),
            Err(ConfigError::InvalidCustomMode(hk(1))),
        ),
        (OutputConfig::new(vec![custom(1920, 1080, 75_000)]), Ok(())),
        (OutputConfig::new(vec![with_mode(1, 11)]), Ok(())),
        (
            OutputConfig::new(vec![
                with_mode(1, 10),
                HeadConfig::new(hk(2), HeadChangeRequest::Disable),
            ]),
            Ok(()),
        ),
        (
            OutputConfig::new(vec![HeadConfig::new(hk(1), HeadChangeRequest::Disable)]),
            Ok(()),
        ),
    ];
    for (index, (config, expected)) in cases.into_iter().enumerate() {
        assert_eq!(config.validate(&state.outputs), expected, "case {index}");
    }
}

#[test]
fn complete_fills_what_the_config_leaves_open() {
    let state = two_heads();
    let config = OutputConfig::new(vec![enable(1, HeadSettings::keep().at(Point::new(100, 0)))]);
    let parts = complete(&config, &state.outputs);
    assert_eq!(parts.len(), 1);
    let HeadChangeRequest::Enable(settings) = &parts[0].request else {
        panic!("expected enable");
    };
    assert_eq!(settings.mode, Some(ModeChoice::Existing(mk(10))));
    assert_eq!(settings.position, Some(Point::new(100, 0)));
    assert_eq!(settings.transform, Some(Transform::Normal));
    assert_eq!(settings.scale, Some(ScaleMilli::new(1500)));
    assert_eq!(settings.adaptive_sync, None);
}

#[test]
fn complete_mentions_the_other_enabled_heads_and_not_the_disabled_ones() {
    let state = two_heads();
    // Head 2 is off: switching head 1 off mentions nothing else.
    let config = OutputConfig::new(vec![HeadConfig::new(hk(1), HeadChangeRequest::Disable)]);
    assert_eq!(complete(&config, &state.outputs), config.heads);

    // Head 1 is on: a config that only turns head 2 on keeps head 1 as it is.
    let config = OutputConfig::new(vec![enable(2, HeadSettings::keep())]);
    let parts = complete(&config, &state.outputs);
    let keys: Vec<_> = parts.iter().map(|p| p.head).collect();
    assert_eq!(keys, vec![hk(2), hk(1)]);
    let HeadChangeRequest::Enable(kept) = &parts[1].request else {
        panic!("expected enable");
    };
    assert_eq!(kept.mode, Some(ModeChoice::Existing(mk(10))));
}

#[test]
fn complete_falls_back_to_the_preferred_then_the_first_mode() {
    let state = two_heads();
    // Head 2 has one mode, not preferred, and none current.
    let parts = complete(
        &OutputConfig::new(vec![enable(2, HeadSettings::keep())]),
        &state.outputs,
    );
    let HeadChangeRequest::Enable(settings) = &parts[0].request else {
        panic!("expected enable");
    };
    assert_eq!(settings.mode, Some(ModeChoice::Existing(mk(20))));

    // Without a current mode, the preferred one wins over the first.
    let state = run([
        out(OutputEvent::HeadOpened(hk(1))),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(10),
        }),
        out(OutputEvent::ModeOpened {
            head: hk(1),
            mode: mk(11),
        }),
        mode_change(mk(11), ModeChange::Preferred(Preferred::Yes)),
    ]);
    let parts = complete(
        &OutputConfig::new(vec![enable(1, HeadSettings::keep())]),
        &state.outputs,
    );
    let HeadChangeRequest::Enable(settings) = &parts[0].request else {
        panic!("expected enable");
    };
    assert_eq!(settings.mode, Some(ModeChoice::Existing(mk(11))));
}
