//! Input receipts: what a burst counts and digests to, pinned so the compositor and cuad agree.

use casement_protocols::vocab::{
    PressEdge, ReceiptCounts, ReceiptDigest, ReceiptItem, receipt_counts, receipt_digest,
};

/// A burst: a move, a click, a key tap, typed text and a scroll.
fn burst() -> Vec<ReceiptItem> {
    vec![
        ReceiptItem::Motion { x: 10, y: -3 },
        ReceiptItem::Button {
            code: 272,
            edge: PressEdge::Press,
        },
        ReceiptItem::Button {
            code: 272,
            edge: PressEdge::Release,
        },
        ReceiptItem::Key {
            code: 30,
            edge: PressEdge::Press,
        },
        ReceiptItem::Key {
            code: 30,
            edge: PressEdge::Release,
        },
        ReceiptItem::text("hi"),
        ReceiptItem::Scroll {
            dx120: 0,
            dy120: -120,
        },
    ]
}

#[test]
fn counts_count_presses_not_releases() {
    const CASES: &[(&str, &[ReceiptItem], ReceiptCounts)] = &[
        (
            "nothing",
            &[],
            ReceiptCounts {
                events: 0,
                presses: 0,
                keys: 0,
                text_chars: 0,
            },
        ),
        (
            "a click is one press and two events",
            &[
                ReceiptItem::Button {
                    code: 272,
                    edge: PressEdge::Press,
                },
                ReceiptItem::Button {
                    code: 272,
                    edge: PressEdge::Release,
                },
            ],
            ReceiptCounts {
                events: 2,
                presses: 1,
                keys: 0,
                text_chars: 0,
            },
        ),
        (
            "a key tap is one key",
            &[
                ReceiptItem::Key {
                    code: 30,
                    edge: PressEdge::Press,
                },
                ReceiptItem::Key {
                    code: 30,
                    edge: PressEdge::Release,
                },
            ],
            ReceiptCounts {
                events: 2,
                presses: 0,
                keys: 1,
                text_chars: 0,
            },
        ),
        (
            "text adds its characters",
            &[
                ReceiptItem::Text { chars: 3, fnv: 0 },
                ReceiptItem::Text { chars: 4, fnv: 0 },
            ],
            ReceiptCounts {
                events: 2,
                presses: 0,
                keys: 0,
                text_chars: 7,
            },
        ),
        (
            "motion and scroll are events only",
            &[
                ReceiptItem::Motion { x: 1, y: 1 },
                ReceiptItem::Scroll {
                    dx120: 0,
                    dy120: 120,
                },
            ],
            ReceiptCounts {
                events: 2,
                presses: 0,
                keys: 0,
                text_chars: 0,
            },
        ),
    ];
    for (case, items, want) in CASES {
        assert_eq!(&receipt_counts(items), want, "{case}");
    }
}

#[test]
fn text_items_count_characters_and_digest_the_utf8() {
    assert_eq!(
        ReceiptItem::text("foobar"),
        ReceiptItem::Text {
            chars: 6,
            fnv: 0x8594_4171_f739_67e8
        }
    );
    let ReceiptItem::Text { chars, .. } = ReceiptItem::text("héllo") else {
        unreachable!("text builds a Text item")
    };
    assert_eq!(chars, 5);
}

#[test]
fn the_digest_of_a_burst_is_pinned() {
    // Computed independently from the canonical bytes (tag byte, little-endian fields).
    assert_eq!(
        receipt_digest(&burst()),
        ReceiptDigest(0xa6f9_4c16_61f1_08f3_u64.to_le_bytes())
    );
    assert_eq!(
        receipt_counts(&burst()),
        ReceiptCounts {
            events: 7,
            presses: 1,
            keys: 1,
            text_chars: 2
        }
    );
}

#[test]
fn an_empty_burst_digests_to_the_offset_basis() {
    assert_eq!(
        receipt_digest(&[]),
        ReceiptDigest(0xcbf2_9ce4_8422_2325_u64.to_le_bytes())
    );
}

#[test]
fn the_digest_sees_order_and_every_field() {
    let base = receipt_digest(&burst());
    let reversed: Vec<ReceiptItem> = burst().into_iter().rev().collect();
    assert_ne!(receipt_digest(&reversed), base, "order");
    let mut moved = burst();
    moved[0] = ReceiptItem::Motion { x: 10, y: -2 };
    assert_ne!(receipt_digest(&moved), base, "a field");
    let mut edge = burst();
    edge[2] = ReceiptItem::Button {
        code: 272,
        edge: PressEdge::Press,
    };
    assert_ne!(receipt_digest(&edge), base, "an edge");
    let mut kind = burst();
    kind[3] = ReceiptItem::Button {
        code: 30,
        edge: PressEdge::Press,
    };
    assert_ne!(receipt_digest(&kind), base, "a kind with the same fields");
}
