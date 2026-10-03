//! The input receipt: what the compositor let through an agent seat, counted and digested.
//!
//! The compositor sends `quire_agent_lease_v1.receipt` after each burst and cuad computes the
//! same from what it sent; one algorithm on both sides means a mismatch is a real bug or a
//! second injector. The digest is FNV-1a 64 over each item's canonical bytes: a consistency
//! check, not a security property.

use serde::{Deserialize, Serialize};

use super::agent::PressEdge;
use super::code::WireCode;

/// FNV-1a 64's offset basis.
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
/// FNV-1a 64's prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// One input event an agent seat delivered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "v", rename_all = "snake_case")]
pub enum ReceiptItem {
    /// The pointer moved to a window-local point.
    Motion {
        /// Window-local logical x.
        x: i32,
        /// Window-local logical y.
        y: i32,
    },
    /// A pointer button went down or up.
    Button {
        /// The evdev button code.
        code: u32,
        /// Down or up.
        edge: PressEdge,
    },
    /// A key went down or up.
    Key {
        /// The evdev key code.
        code: u32,
        /// Down or up.
        edge: PressEdge,
    },
    /// Text was typed: its length and digest, never the text.
    Text {
        /// Characters (Unicode scalar values).
        chars: u32,
        /// FNV-1a 64 of the text's UTF-8 bytes.
        fnv: u64,
    },
    /// The pointer scrolled.
    Scroll {
        /// Horizontal, in 120ths of a wheel detent.
        dx120: i32,
        /// Vertical, in 120ths of a wheel detent.
        dy120: i32,
    },
}

impl ReceiptItem {
    /// The receipt item for typed `text`: its character count and digest.
    pub fn text(text: &str) -> ReceiptItem {
        ReceiptItem::Text {
            chars: u32::try_from(text.chars().count()).unwrap_or(u32::MAX),
            fnv: fnv1a(FNV_OFFSET, text.as_bytes()),
        }
    }

    /// The item's canonical bytes: a tag byte, then each field little-endian, edges as their
    /// wire codes.
    fn canonical(&self) -> Vec<u8> {
        match *self {
            ReceiptItem::Motion { x, y } => [&[0][..], &x.to_le_bytes(), &y.to_le_bytes()].concat(),
            ReceiptItem::Button { code, edge } => {
                [&[1][..], &code.to_le_bytes(), &edge.code().to_le_bytes()].concat()
            }
            ReceiptItem::Key { code, edge } => {
                [&[2][..], &code.to_le_bytes(), &edge.code().to_le_bytes()].concat()
            }
            ReceiptItem::Text { chars, fnv } => {
                [&[3][..], &chars.to_le_bytes(), &fnv.to_le_bytes()].concat()
            }
            ReceiptItem::Scroll { dx120, dy120 } => {
                [&[4][..], &dx120.to_le_bytes(), &dy120.to_le_bytes()].concat()
            }
        }
    }
}

/// What a burst of receipt items adds up to: the counts `quire_agent_lease_v1.receipt`
/// carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptCounts {
    /// Every item.
    pub events: u32,
    /// Button presses (releases are not counted).
    pub presses: u32,
    /// Key presses (releases are not counted).
    pub keys: u32,
    /// Characters typed as text.
    pub text_chars: u32,
}

/// The digest of a burst of receipt items: FNV-1a 64, little-endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReceiptDigest(pub [u8; 8]);

/// Counts a burst of receipt items.
pub fn receipt_counts(items: &[ReceiptItem]) -> ReceiptCounts {
    let start = ReceiptCounts {
        events: 0,
        presses: 0,
        keys: 0,
        text_chars: 0,
    };
    items.iter().fold(start, |counts, item| {
        let events = counts.events.saturating_add(1);
        match *item {
            ReceiptItem::Button {
                edge: PressEdge::Press,
                ..
            } => ReceiptCounts {
                events,
                presses: counts.presses.saturating_add(1),
                ..counts
            },
            ReceiptItem::Key {
                edge: PressEdge::Press,
                ..
            } => ReceiptCounts {
                events,
                keys: counts.keys.saturating_add(1),
                ..counts
            },
            ReceiptItem::Text { chars, .. } => ReceiptCounts {
                events,
                text_chars: counts.text_chars.saturating_add(chars),
                ..counts
            },
            _ => ReceiptCounts { events, ..counts },
        }
    })
}

/// Digests a burst of receipt items, in order.
pub fn receipt_digest(items: &[ReceiptItem]) -> ReceiptDigest {
    let hash = items
        .iter()
        .fold(FNV_OFFSET, |hash, item| fnv1a(hash, &item.canonical()));
    ReceiptDigest(hash.to_le_bytes())
}

/// FNV-1a 64 of `bytes`, continuing from `hash`.
fn fnv1a(hash: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(hash, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME)
    })
}

#[cfg(test)]
mod tests {
    use super::{FNV_OFFSET, fnv1a};

    /// FNV-1a 64's published test vectors.
    const VECTORS: &[(&str, u64)] = &[
        ("", 0xcbf2_9ce4_8422_2325),
        ("a", 0xaf63_dc4c_8601_ec8c),
        ("foobar", 0x8594_4171_f739_67e8),
    ];

    #[test]
    fn fnv1a_matches_the_published_vectors() {
        for (input, want) in VECTORS {
            assert_eq!(fnv1a(FNV_OFFSET, input.as_bytes()), *want, "{input:?}");
        }
    }
}
