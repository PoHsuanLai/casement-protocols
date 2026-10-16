//! The XML files against the frozen interface: names, versions, every message with its
//! arguments in opcode order, enum codes, cross-references, the vocab, the licence notice, and
//! `quire-agent-v1.xml`'s bytes.

use crate::support;

use std::collections::BTreeSet;

use casement_protocols::vocab::{
    ActorKind, Answer, AppLifeKind, AppVisibility, Assign, Attention, Corner, DragOutcome, Edited,
    Emphasis, FrameMode, GlowCode, KeyGesture, LeaseState, MacMods, MaskReason, Memory,
    MenuSourceKind, Modality, OverviewMode, OverviewState, PlaceKind, PlaceTarget, PlacementMark,
    Power, ProofKind, Responsiveness, RevokeReason, RunMode, ScreenEdge, SensitivityLevel,
    ShellRole, Shown, SpaceActivity, SpaceKind, SplitSide, SubjectKind, Suspend, TileZone, Toggle,
    TrustedAnchor, TrustedKind, WindowEntry, WindowRole, WindowStateKind, WireCode, Withdrawn,
};
use support::frozen::{ERRORS, EXTERNAL, INTERFACES};
use support::{Interface, Message, Protocol, protocols};

/// A vocab enum's entries: (serde name, code), in code order.
type Entries = fn() -> Vec<(String, u32)>;

fn entries<T: WireCode + serde::Serialize>() -> Vec<(String, u32)> {
    T::ALL
        .iter()
        .map(|value| {
            let name = serde_json::to_value(value).expect("a vocab value serializes");
            (
                name.as_str()
                    .expect("a fieldless enum serializes as a string")
                    .to_owned(),
                value.code(),
            )
        })
        .collect()
}

/// Every non-error, non-bitfield XML enum and its vocab type: (interface, enum, entries).
const VOCAB: &[(&str, &str, Entries)] = &[
    (
        "quire_desktop_manager_v1",
        "actor_kind",
        entries::<ActorKind>,
    ),
    ("quire_desktop_app_v1", "proof_kind", entries::<ProofKind>),
    (
        "quire_desktop_app_v1",
        "app_life_kind",
        entries::<AppLifeKind>,
    ),
    (
        "quire_desktop_app_v1",
        "app_visibility",
        entries::<AppVisibility>,
    ),
    (
        "quire_desktop_app_v1",
        "menu_source_kind",
        entries::<MenuSourceKind>,
    ),
    ("quire_desktop_app_v1", "attention", entries::<Attention>),
    ("quire_desktop_window_v1", "modality", entries::<Modality>),
    (
        "quire_desktop_window_v1",
        "window_state_kind",
        entries::<WindowStateKind>,
    ),
    (
        "quire_desktop_window_v1",
        "place_kind",
        entries::<PlaceKind>,
    ),
    (
        "quire_desktop_window_v1",
        "responsiveness",
        entries::<Responsiveness>,
    ),
    (
        "quire_desktop_window_v1",
        "lease_state",
        entries::<LeaseState>,
    ),
    ("quire_desktop_window_v1", "toggle", entries::<Toggle>),
    ("quire_desktop_window_v1", "assign", entries::<Assign>),
    ("quire_desktop_space_v1", "space_kind", entries::<SpaceKind>),
    (
        "quire_desktop_space_v1",
        "space_activity",
        entries::<SpaceActivity>,
    ),
    ("quire_shell_v1", "shell_role", entries::<ShellRole>),
    ("quire_shell_v1", "key_gesture", entries::<KeyGesture>),
    ("quire_shell_v1", "overview_mode", entries::<OverviewMode>),
    ("quire_shell_v1", "overview_state", entries::<OverviewState>),
    ("quire_shell_v1", "corner", entries::<Corner>),
    ("quire_shell_v1", "screen_edge", entries::<ScreenEdge>),
    ("quire_shell_v1", "shown", entries::<Shown>),
    ("quire_shell_v1", "window_entry", entries::<WindowEntry>),
    ("quire_shell_v1", "split_side", entries::<SplitSide>),
    ("quire_shell_v1", "drag_outcome", entries::<DragOutcome>),
    ("quire_shell_v1", "power", entries::<Power>),
    ("quire_shell_v1", "subject_kind", entries::<SubjectKind>),
    ("quire_shell_v1", "memory", entries::<Memory>),
    ("quire_shell_v1", "answer", entries::<Answer>),
    ("quire_window_v1", "window_role", entries::<WindowRole>),
    ("quire_window_v1", "tile_zone", entries::<TileZone>),
    ("quire_window_v1", "place_target", entries::<PlaceTarget>),
    (
        "quire_window_v1",
        "placement_mark",
        entries::<PlacementMark>,
    ),
    ("quire_window_v1", "emphasis", entries::<Emphasis>),
    ("quire_window_v1", "frame_mode", entries::<FrameMode>),
    ("quire_window_v1", "edited", entries::<Edited>),
    (
        "quire_sensitive_surface_v1",
        "sensitivity_level",
        entries::<SensitivityLevel>,
    ),
    ("quire_agent_manager_v1", "lease_mode", entries::<RunMode>),
    (
        "quire_agent_lease_v1",
        "revoke_reason",
        entries::<RevokeReason>,
    ),
    ("quire_agent_lease_v1", "suspend_reason", entries::<Suspend>),
    ("quire_agent_lease_v1", "glow_state", entries::<GlowCode>),
    ("quire_agent_lease_v1", "mask_reason", entries::<MaskReason>),
    (
        "quire_trusted_manager_v1",
        "trusted_kind",
        entries::<TrustedKind>,
    ),
    (
        "quire_trusted_manager_v1",
        "trusted_anchor",
        entries::<TrustedAnchor>,
    ),
    (
        "quire_trusted_surface_v1",
        "withdrawn",
        entries::<Withdrawn>,
    ),
];

/// `quire_shell_v1.mac_mods`, by entry name.
const MAC_MODS: &[(&str, MacMods)] = &[
    ("none", MacMods::NONE),
    ("shift", MacMods::SHIFT),
    ("control", MacMods::CONTROL),
    ("option", MacMods::OPTION),
    ("command", MacMods::COMMAND),
    ("caps_lock", MacMods::CAPS_LOCK),
    ("function", MacMods::FUNCTION),
];

fn all_interfaces(protocols: &[Protocol]) -> Vec<&Interface> {
    protocols
        .iter()
        .flat_map(|protocol| &protocol.interfaces)
        .collect()
}

fn interface<'a>(protocols: &'a [Protocol], name: &str) -> &'a Interface {
    all_interfaces(protocols)
        .into_iter()
        .find(|interface| interface.name == name)
        .unwrap_or_else(|| panic!("no interface {name}"))
}

/// An entry value as the Wayland schema allows it: decimal or `0x` hexadecimal.
fn value(text: &str) -> u32 {
    let parsed = match text.strip_prefix("0x") {
        Some(hex) => u32::from_str_radix(hex, 16),
        None => text.parse(),
    };
    parsed.unwrap_or_else(|_| panic!("entry value {text:?} is not a u32"))
}

#[test]
fn every_protocol_is_named_after_its_file() {
    for protocol in protocols() {
        let expected = protocol.file.trim_end_matches(".xml").replace('-', "_");
        assert_eq!(protocol.name, expected, "{}", protocol.file);
        assert!(
            protocol.name.starts_with("quire_") && protocol.name.ends_with("_v1"),
            "{}",
            protocol.name
        );
    }
}

#[test]
fn every_interface_is_a_quire_v1_interface_at_its_version() {
    let protocols = protocols();
    let names: Vec<&str> = all_interfaces(&protocols)
        .iter()
        .map(|interface| interface.name.as_str())
        .collect();
    let unique: BTreeSet<&str> = names.iter().copied().collect();
    assert_eq!(
        unique.len(),
        names.len(),
        "an interface name repeats: {names:?}"
    );
    for interface in all_interfaces(&protocols) {
        assert!(
            interface.name.starts_with("quire_") && interface.name.ends_with("_v1"),
            "{}",
            interface.name
        );
        // Only the shell's global, the window manager and the desktop's manager and app (whose
        // objects take the manager's version) have grown past version 1.
        let version = match interface.name.as_str() {
            "quire_shell_v1" => "5",
            "quire_window_manager_v1" | "quire_desktop_manager_v1" | "quire_desktop_app_v1" => "2",
            _ => "1",
        };
        assert_eq!(interface.version, version, "{}", interface.name);
    }
}

#[test]
fn the_shells_newer_messages_are_appended_in_version_order_and_say_since() {
    let protocols = protocols();
    let shell = interface(&protocols, "quire_shell_v1");
    let names = |list: &[Message]| -> Vec<String> { list.iter().map(|m| m.name.clone()).collect() };
    // (kind, messages, the names added by version 2, 3, 4 and 5), each in wire order.
    for (kind, messages, added) in [
        (
            "requests",
            &shell.requests,
            [
                &[
                    "get_space_capture_source",
                    "space_target_clear",
                    "space_target",
                ][..],
                &["space_target_new"][..],
                &[] as &[&str],
                &["permission_answer"][..],
            ],
        ),
        (
            "events",
            &shell.events,
            [
                &["overview_output", "drop_target"][..],
                &["drop_target_new"][..],
                &["window_drag_begin", "window_drag_end"][..],
                &["permission_ask", "permission_withdraw"][..],
            ],
        ),
    ] {
        let newer: usize = added.iter().map(|names| names.len()).sum();
        let (old, mut rest) = messages.split_at(messages.len() - newer);
        assert!(
            old.iter().all(|m| m.since.is_none()),
            "{kind}: a version 1 message has no since"
        );
        for (version, want) in (2..).zip(added) {
            let (these, after) = rest.split_at(want.len());
            assert_eq!(
                names(these),
                want,
                "{kind}: version {version} comes after version {}",
                version - 1
            );
            let since = version.to_string();
            assert!(
                these
                    .iter()
                    .all(|m| m.since.as_deref() == Some(since.as_str())),
                "{kind}: a version {version} message says since={version}"
            );
            rest = after;
        }
    }
}

#[test]
fn the_consent_enums_are_used_only_by_version_5_messages() {
    let protocols = protocols();
    let shell = interface(&protocols, "quire_shell_v1");
    for name in ["power", "subject_kind", "memory", "answer"] {
        let users: Vec<(&str, Option<&str>)> = shell
            .requests
            .iter()
            .chain(&shell.events)
            .filter(|m| {
                m.args
                    .iter()
                    .any(|a| a.enumeration.as_deref() == Some(name))
            })
            .map(|m| (m.name.as_str(), m.since.as_deref()))
            .collect();
        assert!(!users.is_empty(), "{name} is used by a message");
        assert!(
            users.iter().all(|(_, since)| *since == Some("5")),
            "{name}: a version 4 client would see it through {users:?}"
        );
    }
}

#[test]
fn the_desktop_apps_subject_event_comes_last_and_says_since_2() {
    let protocols = protocols();
    let app = interface(&protocols, "quire_desktop_app_v1");
    let (old, newer) = app.events.split_at(app.events.len() - 1);
    assert!(
        old.iter().all(|m| m.since.is_none()),
        "a version 1 event has no since"
    );
    assert_eq!(
        newer.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(),
        ["subject"]
    );
    assert_eq!(newer[0].since.as_deref(), Some("2"));
    assert!(
        app.requests.iter().all(|m| m.since.is_none()),
        "the requests are version 1"
    );
    // Objects take their parent's version, so the manager is at the app's version.
    let manager = interface(&protocols, "quire_desktop_manager_v1");
    assert_eq!(manager.version, app.version);
}

#[test]
fn the_window_managers_quit_event_comes_last_and_says_since_2() {
    let protocols = protocols();
    let manager = interface(&protocols, "quire_window_manager_v1");
    let names: Vec<&str> = manager.events.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, ["quit"]);
    assert!(
        manager
            .events
            .iter()
            .all(|m| m.since.as_deref() == Some("2")),
        "a version 2 event says since=2"
    );
    assert!(
        manager.requests.iter().all(|m| m.since.is_none()),
        "the requests are version 1"
    );
}

#[test]
fn every_interface_has_exactly_the_frozen_messages_in_opcode_order() {
    let protocols = protocols();
    for (file, name, requests, events) in INTERFACES {
        let protocol = protocols
            .iter()
            .find(|protocol| protocol.file == *file)
            .expect("a known file");
        let interface = protocol
            .interfaces
            .iter()
            .find(|interface| interface.name == *name)
            .unwrap_or_else(|| panic!("{file} has no {name}"));
        let got_requests: Vec<String> = interface.requests.iter().map(|m| m.signature()).collect();
        let got_events: Vec<String> = interface.events.iter().map(|m| m.signature()).collect();
        assert_eq!(got_requests, *requests, "{name} requests");
        assert_eq!(got_events, *events, "{name} events");
    }
    let frozen: BTreeSet<&str> = INTERFACES.iter().map(|(_, name, _, _)| *name).collect();
    let present: BTreeSet<&str> = all_interfaces(&protocols)
        .iter()
        .map(|i| i.name.as_str())
        .collect();
    assert_eq!(
        present, frozen,
        "the XML has interfaces the freeze does not list, or lacks some"
    );
}

#[test]
fn every_enum_entry_has_a_distinct_code() {
    let protocols = protocols();
    for interface in all_interfaces(&protocols) {
        for enumeration in &interface.enums {
            let codes: Vec<u32> = enumeration
                .entries
                .iter()
                .map(|(_, code)| value(code))
                .collect();
            let what = format!("{}.{}", interface.name, enumeration.name);
            assert!(!codes.is_empty(), "{what} has no entries");
            match enumeration.bitfield.as_deref() {
                Some("true") => {
                    let bits: Vec<u32> = codes.iter().copied().filter(|code| *code != 0).collect();
                    assert!(
                        bits.iter().all(|bit| bit.is_power_of_two()),
                        "{what}: {codes:?}"
                    );
                    let unique: BTreeSet<u32> = bits.iter().copied().collect();
                    assert_eq!(unique.len(), bits.len(), "{what}: {codes:?}");
                }
                _ => {
                    let dense: Vec<u32> = (0..codes.len() as u32).collect();
                    assert_eq!(codes, dense, "{what}: codes run 0, 1, 2, ... in order");
                }
            }
        }
    }
}

#[test]
fn every_error_enum_is_the_frozen_one() {
    let protocols = protocols();
    for (name, want) in ERRORS {
        let interface = interface(&protocols, name);
        let error = interface
            .enums
            .iter()
            .find(|e| e.name == "error")
            .unwrap_or_else(|| panic!("{name} has no error enum"));
        let got: Vec<&str> = error
            .entries
            .iter()
            .map(|(entry, _)| entry.as_str())
            .collect();
        assert_eq!(got, *want, "{name}.error");
    }
    let with_errors: BTreeSet<&str> = all_interfaces(&protocols)
        .into_iter()
        .filter(|interface| interface.enums.iter().any(|e| e.name == "error"))
        .map(|interface| interface.name.as_str())
        .collect();
    let frozen: BTreeSet<&str> = ERRORS.iter().map(|(name, _)| *name).collect();
    assert_eq!(with_errors, frozen);
}

#[test]
fn every_vocab_enum_has_the_xml_names_and_codes() {
    let protocols = protocols();
    for (name, enumeration, vocab) in VOCAB {
        let interface = interface(&protocols, name);
        let xml = interface
            .enums
            .iter()
            .find(|e| e.name == *enumeration)
            .unwrap_or_else(|| panic!("{name} has no enum {enumeration}"));
        let got: Vec<(String, u32)> = xml
            .entries
            .iter()
            .map(|(entry, code)| (entry.clone(), value(code)))
            .collect();
        assert_eq!(got, vocab(), "{name}.{enumeration}");
    }
}

#[test]
fn every_xml_enum_has_a_vocab_type() {
    let protocols = protocols();
    let in_xml: BTreeSet<(&str, &str)> = all_interfaces(&protocols)
        .into_iter()
        .flat_map(|interface| {
            interface
                .enums
                .iter()
                .filter(|e| e.name != "error" && e.bitfield.is_none())
                .map(move |e| (interface.name.as_str(), e.name.as_str()))
        })
        .collect();
    let in_vocab: BTreeSet<(&str, &str)> = VOCAB
        .iter()
        .map(|(name, enumeration, _)| (*name, *enumeration))
        .collect();
    assert_eq!(in_xml, in_vocab);
}

#[test]
fn mac_mods_bits_are_the_vocab_constants() {
    let protocols = protocols();
    let shell = interface(&protocols, "quire_shell_v1");
    let mods = shell
        .enums
        .iter()
        .find(|e| e.name == "mac_mods")
        .expect("quire_shell_v1.mac_mods");
    assert_eq!(mods.bitfield.as_deref(), Some("true"));
    let got: Vec<(&str, u32)> = mods
        .entries
        .iter()
        .map(|(name, code)| (name.as_str(), value(code)))
        .collect();
    let want: Vec<(&str, u32)> = MAC_MODS
        .iter()
        .map(|(name, mods)| (*name, mods.0))
        .collect();
    assert_eq!(got, want);
}

#[test]
fn every_reference_resolves() {
    let protocols = protocols();
    let interfaces = all_interfaces(&protocols);
    let known: BTreeSet<&str> = interfaces
        .iter()
        .map(|interface| interface.name.as_str())
        .chain(EXTERNAL.iter().copied())
        .collect();
    for interface in &interfaces {
        for message in interface.requests.iter().chain(&interface.events) {
            for arg in &message.args {
                let at = format!("{}.{}({})", interface.name, message.name, arg.name);
                if let Some(target) = &arg.interface {
                    assert!(
                        known.contains(target.as_str()),
                        "{at}: unknown interface {target}"
                    );
                }
                if let Some(enumeration) = &arg.enumeration {
                    let (owner, name) = match enumeration.split_once('.') {
                        Some((owner, name)) => (owner, name),
                        None => (interface.name.as_str(), enumeration.as_str()),
                    };
                    let owner = interfaces
                        .iter()
                        .find(|candidate| candidate.name == owner)
                        .unwrap_or_else(|| panic!("{at}: enum owner {owner} is not one of ours"));
                    assert!(
                        owner.enums.iter().any(|e| e.name == name),
                        "{at}: no enum {enumeration}"
                    );
                    assert!(
                        matches!(arg.kind.as_str(), "uint" | "int"),
                        "{at}: an enum on {}",
                        arg.kind
                    );
                }
            }
        }
    }
}

#[test]
fn every_copyright_carries_the_licence_notice() {
    for protocol in protocols() {
        assert!(
            protocol.copyright.contains("Copyright 2026 Po Hsuan Lai"),
            "{}",
            protocol.file
        );
        assert!(
            protocol.copyright.contains(
                "Licensed under the MIT license or the Apache License, Version 2.0, at your option."
            ),
            "{}",
            protocol.file
        );
    }
}

#[test]
fn every_new_protocol_carries_the_mit_permission_notice() {
    for protocol in protocols()
        .iter()
        .filter(|protocol| protocol.file != "quire-agent-v1.xml")
    {
        assert!(
            protocol
                .copyright
                .contains("Permission is hereby granted, free of charge"),
            "{}",
            protocol.file
        );
        assert!(
            protocol
                .copyright
                .contains("THE SOFTWARE IS PROVIDED \"AS IS\""),
            "{}",
            protocol.file
        );
    }
}

/// The protocol table of `ARCHITECTURE.md` section 2: (file name, its Server cell).
fn architecture_servers() -> Vec<(String, String)> {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ARCHITECTURE.md"),
    )
    .expect("ARCHITECTURE.md");
    text.lines()
        .filter(|line| line.starts_with("| `protocols/"))
        .filter_map(|line| {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            let file = cells.get(1)?.trim_matches('`').strip_prefix("protocols/")?;
            Some((file.to_owned(), (*cells.get(4)?).to_owned()))
        })
        .collect()
}

#[test]
fn every_protocol_is_served_or_says_it_is_not_served_yet() {
    let rows = architecture_servers();
    assert_eq!(
        rows.len(),
        support::FILES.len(),
        "one row per file: {rows:?}"
    );
    for file in support::FILES {
        let (_, server) = rows
            .iter()
            .find(|(name, _)| name == file)
            .unwrap_or_else(|| panic!("{file} has no row in ARCHITECTURE.md"));
        let text = std::fs::read_to_string(support::protocols_dir().join(file)).expect("the XML");
        let says_not_served = text.contains("Not served yet:");
        match server.as_str() {
            "casement" => assert!(
                !says_not_served,
                "{file}: served, yet its XML says it is not"
            ),
            "not served yet" => assert!(
                says_not_served,
                "{file}: not served, yet its XML does not say so"
            ),
            other => panic!("{file}: the Server cell is {other:?}, not casement or not served yet"),
        }
    }
}

#[test]
fn masking_applies_to_every_capture_trusted_ones_included() {
    let text = std::fs::read_to_string(support::protocols_dir().join("quire-sensitive-v1.xml"))
        .expect("the XML");
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("no capture path is unmasked"),
        "sensitive XML states it"
    );
}

/// FNV-1a 64, for pinning a file's bytes.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// `quire-agent-v1.xml` as cua held it (`cua/protocols/quire-agent-v1.xml` at cua `798866b`),
/// with the description reworded for casement and the "Not served yet" paragraph added: its
/// length and FNV-1a 64.
const AGENT_XML_PIN: (usize, u64) = (10032, 0x70ff_92ed_354f_4f80);

#[test]
fn quire_agent_xml_has_the_bytes_cua_held() {
    let bytes =
        std::fs::read(support::protocols_dir().join("quire-agent-v1.xml")).expect("the agent XML");
    assert_eq!((bytes.len(), fnv1a(&bytes)), AGENT_XML_PIN);
}

#[test]
fn quire_agent_xml_is_byte_identical_to_cuas_copy_while_cua_has_one() {
    let ours =
        std::fs::read(support::protocols_dir().join("quire-agent-v1.xml")).expect("the agent XML");
    let cuas = support::protocols_dir().join("../../cua/protocols/quire-agent-v1.xml");
    match std::fs::read(&cuas) {
        Ok(theirs) => assert!(ours == theirs, "{} differs from ours", cuas.display()),
        // cua deletes its copy when it re-exports this crate; the pin above holds the bytes.
        Err(error) => eprintln!("{}: {error}; the pinned digest stands in", cuas.display()),
    }
}

#[cfg(all(feature = "client", feature = "server"))]
#[test]
fn generated_interfaces_carry_the_xml_names_and_versions() {
    use casement_protocols::{client, server};
    let generated = [
        &client::quire_desktop_v1::__interfaces::QUIRE_DESKTOP_MANAGER_V1_INTERFACE,
        &client::quire_desktop_v1::__interfaces::QUIRE_DESKTOP_APP_V1_INTERFACE,
        &client::quire_desktop_v1::__interfaces::QUIRE_DESKTOP_WINDOW_V1_INTERFACE,
        &client::quire_desktop_v1::__interfaces::QUIRE_DESKTOP_SPACE_V1_INTERFACE,
        &client::quire_shell_v1::__interfaces::QUIRE_SHELL_V1_INTERFACE,
        &client::quire_window_v1::__interfaces::QUIRE_WINDOW_MANAGER_V1_INTERFACE,
        &client::quire_window_v1::__interfaces::QUIRE_WINDOW_V1_INTERFACE,
        &client::quire_sensitive_v1::__interfaces::QUIRE_SENSITIVE_MANAGER_V1_INTERFACE,
        &client::quire_sensitive_v1::__interfaces::QUIRE_SENSITIVE_SURFACE_V1_INTERFACE,
        &client::quire_agent_v1::__interfaces::QUIRE_AGENT_MANAGER_V1_INTERFACE,
        &client::quire_agent_v1::__interfaces::QUIRE_AGENT_LEASE_V1_INTERFACE,
        &client::quire_trusted_surface_v1::__interfaces::QUIRE_TRUSTED_MANAGER_V1_INTERFACE,
        &client::quire_trusted_surface_v1::__interfaces::QUIRE_TRUSTED_SURFACE_V1_INTERFACE,
        &server::quire_desktop_v1::__interfaces::QUIRE_DESKTOP_WINDOW_V1_INTERFACE,
        &server::quire_shell_v1::__interfaces::QUIRE_SHELL_V1_INTERFACE,
        &server::quire_window_v1::__interfaces::QUIRE_WINDOW_V1_INTERFACE,
        &server::quire_trusted_surface_v1::__interfaces::QUIRE_TRUSTED_SURFACE_V1_INTERFACE,
    ];
    let protocols = protocols();
    for generated in generated {
        let xml = interface(&protocols, generated.name);
        assert_eq!(
            generated.version.to_string(),
            xml.version,
            "{}",
            generated.name
        );
        let requests: Vec<&str> = generated.requests.iter().map(|m| m.name).collect();
        let events: Vec<&str> = generated.events.iter().map(|m| m.name).collect();
        let xml_requests: Vec<&str> = xml.requests.iter().map(|m| m.name.as_str()).collect();
        let xml_events: Vec<&str> = xml.events.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(
            (requests, events),
            (xml_requests, xml_events),
            "{}",
            generated.name
        );
    }
}
