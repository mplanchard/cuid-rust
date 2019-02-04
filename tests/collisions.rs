extern crate cuid;

use std::collections::HashSet;

#[test]
fn check_cuid_collisions() {
    let mut set = HashSet::new();
    for _ in 0..1200000 {
        let id = cuid::cuid().unwrap();
        assert!(!set.contains(&id));
        set.insert(id);
    }
}

#[test]
fn check_slug_collisions() {
    let mut set = HashSet::new();
    for _ in 0..1200000 {
        let id = cuid::slug().unwrap();
        assert!(!set.contains(&id));
        set.insert(id);
    }
}
