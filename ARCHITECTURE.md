# Architecture

casement-protocols holds the desktop's private Wayland protocols, `quire_*_v1`: the XML, the pure
wire vocabulary every tier shares, and the wayland-scanner bindings for clients and the
compositor. It is a leaf: it depends on nothing of ours, so the compositor (casement), the shell
(sill, shell-host), the agent tier (cua, docket, porter's `prov`), the portal (transom) and
quire's `ds-blitz` all depend on it without a cycle. `CONVENTIONS.md` holds the rules;
`FINDINGS.md` the open items.

## 1. The crate and its layers

One crate, `casement-protocols`, in three layers:

| Layer | Path | Built | Depends on |
| --- | --- | --- | --- |
| XML | `protocols/*.xml` | data | — |
| vocab | `src/vocab/` | always (default features) | serde, thiserror |
| bindings | `src/client.rs`, `src/server.rs` | features `client`, `server` | wayland-scanner, wayland-backend, wayland-client or wayland-server, wayland-protocols, bitflags |

Allowed edges (checked by `scripts/check-boundary.sh`):

- With default features the crate reaches none of `wayland-*`, calloop, smithay, zbus or tokio,
  through any path. Its direct dependencies are exactly serde and thiserror.
- With every feature its direct dependencies are exactly serde, thiserror, wayland-scanner,
  wayland-backend, wayland-client, wayland-server, wayland-protocols and bitflags.
- `src/vocab/` and `src/lib.rs` name no wayland crate.

Consumers take the layer they need: pure crates (casement-core, sill-model, cua-desktop,
docket-core, prov) take default features; Wayland clients take `client`; casement takes `server`.

## 2. The six protocols

| File | Interfaces | Advertised on | Server | Clients |
| --- | --- | --- | --- | --- |
| `protocols/quire-desktop-v1.xml` | `quire_desktop_manager_v1`, `quire_desktop_app_v1`, `quire_desktop_window_v1`, `quire_desktop_space_v1` | Shell and Agent connections (management requests are `not_permitted` on Agent) | casement | sill (full), cuad (read-only) |
| `protocols/quire-shell-v1.xml` | `quire_shell_v1` (version 5: `permission_ask`, `permission_withdraw`, `permission_answer`, the consent question) | Shell connections | casement | sill through shell-host |
| `protocols/quire-window-v1.xml` | `quire_window_manager_v1` (version 2: `quit`, the person asked the app to quit), `quire_window_v1` | every connection | casement | shell-host, ds-blitz, any app |
| `protocols/quire-sensitive-v1.xml` | `quire_sensitive_manager_v1`, `quire_sensitive_surface_v1` | every connection | casement | shell-host, ds-blitz, any app |
| `protocols/quire-agent-v1.xml` | `quire_agent_manager_v1`, `quire_agent_lease_v1` | Agent connections | casement | cuad |
| `protocols/quire-trusted-surface-v1.xml` | `quire_trusted_manager_v1`, `quire_trusted_surface_v1` | Shell connections | casement | sill |

Objects never cross connections, so the shell and trusted-surface protocols name windows by
their ext-foreign-toplevel identifier string and apps by their app key string.

An enum lives in one XML file, on the interface that owns the concept, and other protocols name
it as `interface.enum` (`quire_window_v1.emphasis` in the desktop window's `emphasis` event).
wayland-scanner turns that into the owning module's type, so one concept has one generated type.

| Owner | Enums |
| --- | --- |
| `quire_window_v1` | `window_role`, `tile_zone`, `place_target`, `placement_mark`, `emphasis`, `frame_mode`, `edited` |
| `quire_sensitive_surface_v1` | `sensitivity_level` |
| `quire_agent_manager_v1`, `quire_agent_lease_v1` | `lease_mode`; `revoke_reason`, `suspend_reason`, `glow_state`, `mask_reason` |
| `quire_desktop_manager_v1` | `actor_kind` |
| `quire_desktop_app_v1` | `proof_kind`, `app_life_kind`, `app_visibility`, `menu_source_kind`, `attention` |
| `quire_desktop_window_v1` | `modality`, `window_state_kind`, `place_kind`, `responsiveness`, `lease_state`, `toggle`, `assign` |
| `quire_desktop_space_v1` | `space_kind`, `space_activity` |
| `quire_shell_v1` | `shell_role`, `mac_mods` (bitfield), `key_gesture`, `overview_mode`, `overview_state`, `corner`, `screen_edge`, `shown`, `window_entry`, `split_side`, `drag_outcome`, `power`, `subject_kind`, `memory`, `answer` |
| `quire_trusted_manager_v1`, `quire_trusted_surface_v1` | `trusted_kind`, `trusted_anchor`; `withdrawn` |

The window, sensitive, agent and trusted-surface protocols name no other protocol of ours; the
desktop protocol names window, sensitive and agent enums; the shell protocol names the desktop's
`toggle`.

## 3. One home

| Concept | Home |
| --- | --- |
| A wire value's code and its Rust type | `src/vocab/<area>.rs`, one `wire_enum!` per XML enum |
| Code conversion (`code`, `from_code`, `ALL`) | `vocab::WireCode`, written by `wire_enum!` (`src/vocab/code.rs`) |
| The Mac modifier view | `vocab::MacMods` (`src/vocab/mac_mods.rs`) |
| Input receipt counts and digest | `vocab::{receipt_counts, receipt_digest}` (`src/vocab/receipt.rs`) |
| `RunMode` (docket re-exports it) | `vocab::RunMode` (`src/vocab/agent.rs`) |
| `ActorKind` (prov's wire projection) | `vocab::ActorKind` (`src/vocab/actor.rs`) |
| Generated bindings | `client::<protocol>`, `server::<protocol>` (`src/client.rs`, `src/server.rs`) |

Vocab files: `window.rs` (window and placement enums), `app.rs`, `space.rs`, `shell.rs`,
`mac_mods.rs`, `agent.rs` (lease enums, `LeaseState`, `PressEdge`), `sensitive.rs`,
`trusted.rs`, `actor.rs`, `receipt.rs`, `privilege.rs` (the seven `Power`s with the words a
person reads for each, `SubjectKind`, `Memory`, `Answer`, `AskId`). A vocab enum whose concept carries data in
casement-core is named `…Kind` (`WindowStateKind`, `SpaceKind`, `AppLifeKind`).

## 4. The canonical trait

```rust
pub trait WireCode: Sized + Copy + 'static {
    const ALL: &'static [Self];
    fn code(self) -> u32;
    fn from_code(code: u32) -> Result<Self, CodeError>;
}
```

Every vocab enum implements it through `wire_enum!`; nothing writes `code`, `from_code` or
`ALL` by hand. A vocab enum's serde form is its XML entry name (`snake_case`), so a stored value
reads the same as the protocol.

## 5. Recipes

**Add a value to an enum, or an enum.**
1. Add the `<entry>` (or `<enum>`) to the owning interface in its XML; codes run 0, 1, 2, ... in
   order (a bitfield's are single bits).
2. Add the variant (or a `wire_enum!`) in the vocab file of its area, with a doc line; export a
   new enum from `src/vocab/mod.rs`.
3. A new enum gets a row in `tests/it/xml_introspection.rs` `VOCAB`, a `same_codes` line in
   `tests/it/vocab_codes.rs` and a line in `tests/it/round_trip.rs`.
4. Every consumer that matches on the enum is updated in the same change.

**Add a request or event.**
1. Add it to the interface in its XML with a description; arguments that carry a closed set use
   `enum=`, naming the owning interface when it is another one.
2. Update the interface's row in `tests/it/support/frozen.rs` `INTERFACES` (order is opcode
   order).
3. The server (casement) and every client are updated in the same change (see section 7).

**Add a protocol.**
1. `protocols/quire-<name>-v1.xml` with the copyright block every file carries.
2. A `client_protocol!` line in `src/client.rs` and a `server_protocol!` line in
   `src/server.rs`, listing the protocols whose interfaces it names.
3. The file in `tests/it/support/mod.rs` `FILES`; its interfaces and errors in
   `tests/it/support/frozen.rs` `INTERFACES` and `ERRORS`.

## 6. Tests

| File | Proves |
| --- | --- |
| `tests/it/xml_introspection.rs` | every file parses and is named after itself; every interface is `quire_*_v1` at version 1 (the shell's global at 5 and the window manager at 2, their newer messages last and saying `since=`); every request and event with its arguments, in opcode order, as frozen; error enums; codes dense from 0 (bitfields: single bits); every vocab enum has its XML enum's names and codes and every XML enum has a vocab type; cross-references resolve; the licence notice; `quire-agent-v1.xml`'s pinned bytes; with both features, the generated interfaces' names, versions and messages |
| `tests/it/vocab_codes.rs` | (features `client,server`) every vocab enum accepts exactly the codes the generated enum accepts, with the same variant names, on both sides; `MacMods` bits |
| `tests/it/round_trip.rs` | every vocab value round-trips through serde and its code; pinned JSON for stored forms |
| `tests/it/receipt.rs` | receipt counts (table) and a pinned digest |

`tests/it/support/mod.rs` reads the XML into plain data with quick-xml; `tests/it/support/frozen.rs`
holds the frozen interface as tables (every message with its arguments, every error enum).

## 7. Repo rules

- **Gate**, every exit code checked:
  ```
  cargo fmt --all --check
  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --features client,server -- -D warnings
  cargo test
  cargo test --features client,server
  cargo clippy -p desktop-client --all-targets -- -D warnings
  cargo clippy -p desktop-client --all-targets --features client,fake -- -D warnings
  cargo test -p desktop-client --features client,fake
  ./scripts/check-boundary.sh
  cargo deny check licenses
  ```
- **No `unsafe`** in our source (`unsafe_code = "deny"`); the bindings' `unsafe` is
  wayland-scanner's generated code.
- **Protocol evolution:** until the user declares the protocols released, a v1 XML changes in
  place, with the server and every client updated in the same change (one `~/desktop` manifest
  bump); no `since=` until then, except where a global has already grown: `quire_shell_v1` (version 5)
  and `quire_window_manager_v1` (version 2: the `quit` event, sent only to a client that bound
  version 2, so a version 1 client keeps working).
- **`quire-agent-v1.xml` came from cua byte for byte**; `tests/it/xml_introspection.rs` pins its
  bytes. A change to it updates the pin in the same commit.
- **Clean room:** the XML is written from our own design notes; every interface here is ours
  (`check-boundary.sh` row 5).
- **Licence:** MIT OR Apache-2.0; every XML `<copyright>` carries the notice.

## 8. `desktop-client` (second package, `desktop-client/`)

The workspace has two members: `casement-protocols` (the repo root) and `desktop-client`. The
second is the one typed Wayland desktop client (amendment A3 of the P1 library plan): the
standard wlroots-style protocols plus the Quire arm (`quire_desktop_v1`, `quire_shell_v1`,
filled at P2-Z-dc). It is a separate package so a consumer that wants only toplevels and
outputs on a stock compositor does not take the private bindings: its core takes
`casement-protocols` with default features (the pure vocab, serde and thiserror only), and its
`client` feature takes `casement-protocols/client` for the generated bindings.

| Layer | Path | Built | Depends on |
| --- | --- | --- | --- |
| core | `desktop-client/src/` (all but `client/`, `fake/`) | always | thiserror, casement-protocols (vocab only) |
| wire | `desktop-client/src/client/` | feature `client` | wayland-client, wayland-protocols (ext), wayland-protocols-wlr, rustix (memfd, no mmap), casement-protocols `client` (the `quire_*` bindings) |
| fake | `desktop-client/src/fake/` | feature `fake` | nothing beyond the core |

Core, one concept per file:

| Concept | Home |
| --- | --- |
| Keys the wire layer hands out, names | `keys.rs`, `names.rs` |
| Offered protocols, the arm they choose, `supports` and `admit` | `protocol.rs`, `backend.rs` (`backend_for`), `supports.rs` (`requires_on`), `admit.rs`, `config.rs` |
| Toplevels: state, events, pairing the two protocol handles | `toplevel/` |
| Workspaces: pending and committed snapshots | `workspace/` |
| Outputs and configurations | `output/` |
| Capture: request, events, frame, session | `capture/` |
| Commands, tickets, errors | `command.rs`, `error.rs` |
| The one transition `reduce(DesktopState, DesktopEvent)` | `state.rs`, `event.rs`, `reduce.rs` |
| The seam consumers code against | `desktop.rs` (`trait Desktop`; `DesktopClient` and `FakeDesktop` implement it) |
| The Quire arm: apps, windows, Spaces, the shell's notes, the window-to-handle `Linker`, the projection onto toplevel flags | `quire/` (`app.rs`, `window.rs`, `space.rs`, `shell.rs`, `link.rs`, `project.rs`, `targets.rs`, `reduce.rs`, `set.rs`) |

**Choosing the arm.** `backend_for(&ProtocolSet)` reads the offered globals only (never a
name or the environment): Quire when `quire_desktop_manager_v1` and the ext list are both
offered, else wlr, else the ext list alone. On the Quire arm the plain window commands
(`Activate`, `Close`, `Minimise`, ...) go to `quire_desktop_window_v1`, the wlr manager is not
bound, and a window is the ext handle (title, app id, identifier) plus the Quire window
(everything else) under one `ToplevelKey`: `Linker` matches the manager's window object to the
ext handle by identifier, whichever arrives first. The reducer projects the Quire state onto
`Toplevel.flags` (filled is maximised, the key window is activated), `parent` and `outputs`, so
a consumer that reads only toplevels is unchanged. `quire_shell_v1` is bound only when the
caller says it holds the shell role (`DesktopConfig::holding_shell()`); `admit` returns
`CommandError::NotBound` for a shell command otherwise. A shell connection comes either from
the environment or handed over (`client::WaylandRoute::{Environment, Handed(OwnedFd)}`, the
shape of shell-host's own `WaylandRoute`).

**Recipe: a new Quire command.** Add the variant and its `CommandKind` (`command.rs`), its
protocols in `requires_on` (`supports.rs`), its target check in `quire::check_targets`, its
request in `client/quire_commands.rs` (`carries` and `send`), then a row in
`tests/it/protocol.rs` `quire_only` and a fake test. **A new Quire event:** the variant in the
object's `…Change`/`…Event`, its row in `quire/<object>.rs` `apply`, the dispatch in
`client/quire.rs` (or `client/shell.rs`), and a table row in `tests/it/quire.rs`.

Rules: the core names no `wayland_*` crate and does no I/O; the wire layer reads the
environment only for `WaylandRoute::Environment`, which the caller picks (otherwise the caller
passes the `Connection`); a proxy never enters the state, a key does
(`ToplevelKey` and the others in `keys.rs`), and `Handles` maps keys back to proxies; no
`bool` in a field or parameter (`Enabled`, `Preferred`, `CursorMode`, `Supports`,
`Advance`); every growing public type is `non_exhaustive` with a constructor.

Consumers: sill maps `DesktopState` into its `DesktopView` (app grouping, most-recently-used
order and Spaces are sill's); cua's toplevel client moves onto it later; `casementctl shot`
and the testclient use `DesktopCommand::CaptureOutput` and `Frame::to_rgba8`.

## 9. Consent: `quire_shell_v1` version 5

casement asks the shell, over this global, whether a program may use one of seven `Power`s; the
shell puts the question to the person and answers. The design (who is trusted, the book, the
words) is in the production-audit wave-2 design, section 3; this repo owns the wire.

| Message | Direction | Meaning |
| --- | --- | --- |
| `permission_ask(ask, power, subject_kind, subject, memory)` | casement to shell | one open question; `ask` is unique in a compositor run and never reused; `subject` is a path or an app id, at most 4096 bytes |
| `permission_withdraw(ask)` | casement to shell | the answer is no longer needed (time ran out, screen locked, settled elsewhere); nothing is remembered |
| `permission_answer(ask, answer)` | shell to casement | `allow`, `deny` or `dismissed`; the first answer to an open ask settles it, any other is ignored |

Rules the XML states and the consumers rely on:

- **Version gate.** All three say `since="5"`; a shell that bound an older version never sees an
  ask, and casement refuses the program unremembered (fail closed).
- **Allow Once is not an answer.** casement decides what an answer is kept as (`memory`); the
  shell answers `allow` to an ask whose `memory` is `this_time_only` and that is Allow Once.
- **Replay.** An ask stays open until answered or withdrawn, with or without a shell. A shell that
  binds sees every open ask again, in first-sent order, with the same ids, right after the bind
  events. A shell keys its queue by `ask`.
- **Denial.** `deny` and `dismissed` both refuse the power, with the refusal of the protocol the
  program used; only `deny` on a `remembered` ask is kept. No shell for the grace period, a locked
  screen, too many open asks and an unanswered ask all refuse unremembered.
- **One at a time.** casement keeps at most one ask open toward the shell; a shell may still
  queue.

`desktop-client`: `ShellEvent::{PermissionAsk(PermissionAsk), PermissionWithdraw(AskId)}` (moments,
no state: the shell keeps its queue by id, because an answered ask is not echoed back) and
`DesktopCommand::AnswerPermission { ask, answer }` (45 command kinds; the shell is bound at
`1..=5`; the command is refused with `NotBound` on a compositor that serves less than version 5).
