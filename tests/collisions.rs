use cuid;

use std::sync::{Arc, Mutex};
use std::{collections::HashSet, thread};

#[test]
fn check_cuid_collisions() {
    let mut set = HashSet::new();
    for _ in 0..1_200_000 {
        let id = cuid::cuid().unwrap();
        set.insert(id);
    }
    // we generated 1.2 million unique CUIDs
    assert!(set.len() == 1_200_000)
}

#[test]
/// Build 1.2e6 slugs
fn check_slug_collisions() {
    let mut set = HashSet::new();
    for _ in 0..1_200_000 {
        let id = cuid::slug().unwrap();
        set.insert(id);
    }
    // we generated 600k unique slugs
    assert!(set.len() == 1_200_000)
}

#[test]
/// Ensure we don't get any collisions in 10 threads, each generating 1.2e6 CUIDs
fn check_cross_thread_collisions() {
    let sets: Arc<Mutex<Vec<HashSet<_>>>> = Arc::new(Mutex::new(vec![]));
    let mut threads: Vec<thread::JoinHandle<_>> = vec![];
    let ids_per_thread = 1_200_000;
    let num_threads = 10;

    for _ in 0..num_threads {
        let thread_sets = sets.clone();
        let t = thread::spawn(move || {
            let mut thread_set = HashSet::new();
            for _ in 0..ids_per_thread {
                let id = cuid::cuid().unwrap();
                thread_set.insert(id);
            }
            let mut sets = thread_sets.lock().unwrap();
            sets.push(thread_set);
        });
        threads.push(t);
    }

    threads.into_iter().for_each(|t| {
        t.join().unwrap();
    });

    let local_sets = sets.lock().unwrap();
    assert!(local_sets.len() == num_threads);
    let intersection: HashSet<_> = (*local_sets).iter().fold(HashSet::new(), |acc, set| {
        assert!(set.len() == ids_per_thread);
        acc.intersection(&set)
            .into_iter()
            .map(|i| i.to_owned())
            .collect()
    });
    assert!(intersection.len() == 0);
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {

    use super::*;
    use test::Bencher;

    #[bench]
    /// Ensure we don't get any collisions in 10 threads, each generating 1.2e6 CUIDs
    fn bench_multithread_perf(b: &mut Bencher) {
        b.iter(|| {
            let sets: Arc<Mutex<Vec<Arc<Mutex<HashSet<_>>>>>> = Arc::new(Mutex::new(vec![]));
            let mut threads: Vec<thread::JoinHandle<_>> = vec![];

            let mut local_sets = sets.lock().unwrap();
            for _ in 0..4 {
                local_sets.push(Arc::new(Mutex::new(HashSet::new())));
            }
            drop(local_sets);

            for i in 0..4 {
                let thread_sets = sets.clone();
                let t = thread::spawn(move || {
                    for _ in 0..1200000 {
                        let sets = thread_sets.lock().unwrap();
                        let mut set = sets[i].lock().unwrap();
                        let id = cuid::cuid().unwrap();
                        set.insert(id);
                    }
                });
                threads.push(t);
            }

            threads.into_iter().for_each(|t| {
                t.join().unwrap();
            });

            let local_sets = sets.lock().unwrap();
            let intersection: HashSet<_> = (*local_sets).iter().fold(HashSet::new(), |acc, i| {
                let set = i.lock().unwrap();
                acc.intersection(&set)
                    .into_iter()
                    .map(|i| i.to_owned())
                    .collect()
            });
            assert!(intersection.len() == 0);
        })
    }
}
