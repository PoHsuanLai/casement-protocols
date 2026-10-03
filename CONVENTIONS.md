# Conventions

casement-protocols follows the program's shared conventions, whose text lives in quire's
`CONVENTIONS.md` (identical in quire, sill, shell-host, palmrest and detent): types, traits,
effects, errors, tests, comments, change discipline, borrowing. Read it first. `ARCHITECTURE.md`
here adds the protocol map, the one-home table, the trait, the recipes and the repo rules; where
the two disagree, `ARCHITECTURE.md` wins for this repo.

What this repo adds or decides differently, each with its reason:

1. **Closed sets are `wire_enum!`s implementing `WireCode`, not quire's `Word`.** This crate is
   a leaf below quire (quire's `ds-blitz` depends on it), so it cannot depend on `ds-core`. A
   value's stable slug is its serde `snake_case` form, which is also its XML entry name;
   labels a person reads belong to the UI that draws them.
2. **The XML is the interface.** The vocab and the bindings follow it; a test fails when either
   drifts from it. A wire change starts in the XML.
3. **Generated bindings are exempt from the style rules** (names, docs, `#[non_exhaustive]`):
   they are wayland-scanner's output, kept behind `client` and `server`, and nothing hand-written
   lives in those modules.
