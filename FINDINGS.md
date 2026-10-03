# Findings

Open items and standing facts. An entry names the condition that closes it.

## Stubs behind frozen interfaces

None. The XML, the vocab, the bindings and the receipt functions are complete; servers and
clients of the protocols live in their consumers.

## Open items

| Item | Closes when |
| --- | --- |
| `wayland-scanner`, `wayland-server`, `bitflags` and `quick-xml` (dev) are declared here, outside the pinned block copied from quire's `docs/workspace-deps.toml` (the scanner and server lines are cua's; bitflags is named by wayland-scanner's code for the `mac_mods` bitfield; quick-xml is the scanner's own parser, used by the introspection tests) | quire's pinned block carries the four lines and this workspace copies them verbatim |
| `quire-agent-v1.xml` exists in two places: here and `cua/protocols/` | cua's `quire-agent-protocol` becomes a re-export of `client::quire_agent_v1` and cua deletes its copy (lane S0-CU2); `tests/xml_introspection.rs` then holds the bytes through its pin alone |
| `quire-agent-v1.xml`'s description still describes the cosmic-comp fork as the server and stock cosmic-comp's fallbacks | P4-Z amends v1 in place (`max_edge`, `type_text`, `set_task`, `receipt`) and rewrites the description for casement, updating the pin |
| `quire-agent-v1.xml`'s `<copyright>` has the dual-licence line but not the MIT permission text the other five carry | the P4-Z amendment above adds it |
| `vocab::receipt_digest` and `receipt_counts` have no caller | casement-agent sends `quire_agent_lease_v1.receipt` and cua's seat-input checks it (P4) |

## Standing facts

- **Enum ownership across files.** An XML enum is defined once, on the interface that owns the
  concept, and other files reference it as `interface.enum`; wayland-scanner resolves that to
  `super::<interface>::<Enum>`, so each protocol module imports the interface modules of the
  protocols it names (`src/client.rs`, `src/server.rs`). `toggle` lives on
  `quire_desktop_window_v1` and the shell names it there; `glow_state` stays on
  `quire_agent_lease_v1` (its file is frozen) and the desktop window's `lease` event names it.
- **`tile_zone` is on the wire as an enum only.** No message carries it: `place_target` and
  `placement_mark` carry zones with their own codes (`place_target`'s first eight equal
  `tile_zone`'s). It is in the XML so `TileZone` has a protocol home and a generated twin.
- **Error enums have no vocab type.** They are raised by the server through the generated
  `Error` enums; nothing pure needs them. `tests/xml_introspection.rs` pins them instead.
- **Receipt canonical bytes.** Per item: a tag byte (motion 0, button 1, key 2, text 3,
  scroll 4), then each field little-endian (`i32`, `u32`; a text item's `fnv` as `u64`; an edge
  as its `u32` code). The digest is FNV-1a 64 over the concatenation, little-endian.
  `presses` counts button presses and `keys` key presses (releases are not counted); `events`
  counts every item. `ReceiptItem::text` builds a text item (characters are Unicode scalar
  values, the digest is over UTF-8), so both sides digest text one way.
- **The `wayland-backend` line enables `client_system`** (the pinned block's), so a build with
  either feature links the system libwayland-client.
