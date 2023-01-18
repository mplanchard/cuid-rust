//! Test that CUIDs are of a predictable length

use std::collections::HashSet;

#[test]
fn check_cuid_length() {
    let mut set = HashSet::new();
    for _ in 0..100000 {
        let id = cuid::cuid().unwrap();
        set.insert(id);
    }
    // all CUIDs are of the same length
    // NOTE: this will start failing in ~2059, at which poit this will need to
    // be updated to 26
    assert!(set.iter().all(|i| i.len() == 25));
}

#[test]
fn check_cuid_slug_length() {
    let mut set = HashSet::new();
    for _ in 0..100000 {
        let id = cuid::slug().unwrap();
        set.insert(id);
    }
    // all slugs are of the same length
    assert!(set.iter().all(|i| i.len() == 10));
}
