//! Pairing the two toplevel handles.

use desktop_client::toplevel::{Listed, Pairing, Toplevel, pair};
use desktop_client::{ToplevelIdentifier, ToplevelKey};

use crate::support::tk;

fn listed(key: u64, title: &str, app_id: &str) -> Listed {
    Listed::new(
        tk(key),
        ToplevelIdentifier::new(format!("id-{key}")),
        title,
        app_id,
    )
}

fn managed(key: u64, title: &str, app_id: &str) -> Toplevel {
    let mut t = Toplevel::new(tk(key));
    t.title = title.into();
    t.app_id = app_id.into();
    t
}

fn pairs(listed: &[Listed], managed: &[Toplevel]) -> Vec<(ToplevelKey, ToplevelKey)> {
    let refs: Vec<&Toplevel> = managed.iter().collect();
    pair(listed, &refs)
        .into_iter()
        .map(|p: Pairing| (p.listed, p.managed))
        .collect()
}

#[test]
fn equal_app_id_and_title_pair() {
    let result = pairs(&[listed(1, "T", "a")], &[managed(10, "T", "a")]);
    assert_eq!(result, vec![(tk(1), tk(10))]);
}

#[test]
fn a_different_title_or_app_id_does_not_pair() {
    let cases = [
        (listed(1, "T", "a"), managed(10, "U", "a")),
        (listed(1, "T", "a"), managed(10, "T", "b")),
    ];
    for (l, m) in cases {
        assert!(pairs(&[l], &[m]).is_empty());
    }
}

#[test]
fn pairs_follow_the_content_not_the_order() {
    let result = pairs(
        &[listed(1, "one", "a"), listed(2, "two", "a")],
        &[managed(10, "two", "a"), managed(11, "one", "a")],
    );
    assert_eq!(result, vec![(tk(1), tk(11)), (tk(2), tk(10))]);
}

#[test]
fn identical_windows_pair_in_arrival_order_every_time() {
    let l = [
        listed(1, "same", "a"),
        listed(2, "same", "a"),
        listed(3, "same", "a"),
    ];
    let m = [managed(10, "same", "a"), managed(11, "same", "a")];
    let first = pairs(&l, &m);
    assert_eq!(first, vec![(tk(1), tk(10)), (tk(2), tk(11))]);
    for _ in 0..5 {
        assert_eq!(pairs(&l, &m), first);
    }
}

#[test]
fn each_side_is_used_once() {
    let result = pairs(
        &[listed(1, "T", "a")],
        &[managed(10, "T", "a"), managed(11, "T", "a")],
    );
    assert_eq!(result, vec![(tk(1), tk(10))]);
}

#[test]
fn nothing_to_pair() {
    assert!(pairs(&[], &[managed(10, "T", "a")]).is_empty());
    assert!(pairs(&[listed(1, "T", "a")], &[]).is_empty());
}
