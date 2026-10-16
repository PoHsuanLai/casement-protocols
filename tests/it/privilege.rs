//! The consent vocabulary: the words a person reads for each power, and the ask id.

use std::collections::BTreeSet;

use casement_protocols::vocab::{AskId, Power, WireCode};

/// Words that name a protocol or a developer tool; nothing a person reads may contain one.
const DEVELOPER_WORDS: &[&str] = &[
    "wayland",
    "d-bus",
    "dbus",
    "polkit",
    "uinput",
    "protocol",
    "cli",
    "api",
    "xml",
    "socket",
    "daemon",
    "compositor",
];

#[test]
fn every_power_has_distinct_plain_words() {
    let titles: BTreeSet<&str> = Power::ALL.iter().map(|power| power.title()).collect();
    assert_eq!(titles.len(), Power::ALL.len(), "titles are distinct");
    for power in Power::ALL {
        for text in [power.title(), power.blurb()] {
            assert!(!text.trim().is_empty(), "{power:?} has words");
            let lower = text.to_lowercase();
            let words: BTreeSet<&str> = lower
                .split(|c: char| !(c.is_alphanumeric() || c == '-'))
                .collect();
            for word in DEVELOPER_WORDS {
                assert!(!words.contains(word), "{power:?}: {text:?} says {word:?}");
            }
        }
        assert!(power.blurb().ends_with('.'), "{power:?}: a sentence");
    }
}

#[test]
fn the_settings_section_titles_are_the_designed_ones() {
    let titles: Vec<&str> = Power::ALL.iter().map(|power| power.title()).collect();
    assert_eq!(
        titles,
        [
            "Screen",
            "Typing and clicking",
            "Typing help",
            "Copy and paste",
            "Locking the screen",
            "Displays",
            "Windows",
        ]
    );
}

#[test]
fn an_ask_id_is_its_wire_number_in_serde_too() {
    let id = AskId::new(7);
    assert_eq!(id.get(), 7);
    assert_eq!(serde_json::to_string(&id).expect("serializes"), "7");
    assert_eq!(serde_json::from_str::<AskId>("7").expect("parses"), id);
    assert!(AskId::new(1) < AskId::new(2), "ids order by number");
}
