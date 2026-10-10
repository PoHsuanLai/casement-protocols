//! desktop-client's integration tests: the reducers, pairing, `supports`, configuration
//! validation, frame conversion and the fake.
//!
//! One integration binary: every test file is a module here, and `support` is shared. The wire
//! layer needs a real compositor and has no test here; it is kept thin so the reducers carry
//! the logic.

mod capture;
#[cfg(feature = "fake")]
mod fake;
mod frame;
mod output;
mod pairing;
mod protocol;
mod support;
mod toplevel;
mod workspace;

/// Fails when a test file is not included above (a file that is not `mod`-ed in would silently
/// never run).
#[test]
fn every_test_file_is_included() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/it");
    let main = std::fs::read_to_string(dir.join("main.rs")).expect("main.rs");
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("tests/it") {
        let path = entry.expect("entry").path();
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("a name")
            .to_owned();
        let is_module = path.is_dir() || path.extension().is_some_and(|e| e == "rs");
        if is_module && name != "main" && !main.contains(&format!("mod {name};")) {
            missing.push(name);
        }
    }
    assert!(missing.is_empty(), "not included: {missing:?}");
}
