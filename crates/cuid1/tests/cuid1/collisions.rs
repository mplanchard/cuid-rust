use std::sync::{Arc, Mutex};
use std::{collections::HashSet, thread};

/// Tests to be run normally. Run like `cargo test -- collisions::test`
mod test {
    use super::*;

    #[test]
    #[ignore] // slow, run explicitly
    fn check_cuid_collisions() {
        let mut set = HashSet::new();
        for _ in 0..1_200_000 {
            let id = cuid::cuid().unwrap();
            set.insert(id);
        }
        // we generated unique CUIDs
        assert!(set.len() == 1_200_000);
    }

    #[test]
    #[ignore] // slow, run explicitly
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
            acc.intersection(set)
                .into_iter()
                .map(|i| i.to_owned())
                .collect()
        });
        assert!(intersection.is_empty());
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
                let intersection: HashSet<_> =
                    (*local_sets).iter().fold(HashSet::new(), |acc, i| {
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
}

/// Tests to be run in a single thread. Run like `cargo test -- collisions::single_thread`
mod single_thread {
    use super::*;

    #[test]
    #[ignore]
    /// Build 1.2e6 slugs
    ///
    /// Note that, by default, the faster your system is, the more likely this is to
    /// fail, since the timestamp (a) is truncated as part of the slugification
    /// process, (b) more slugs will be generated at very similar timestamps, and
    /// (c) this test is occuring simultaneously to the cross-thread collision test,
    /// which generates 12 million IDs of its own, and the cuid collision test,
    /// which generates 1.2 million IDs, meaning we're definitely going to have some
    /// counter wraparound, since the counter only has ~1.6 million unique values.
    /// All that, combined with the fact that only a slice of one random block is
    /// used, means that the occasional collision is inevitable, even if pretty
    /// unlikely.
    ///
    /// To reduce the likelihood of a false negative from this test, we thereefore
    /// ignore it for normal test runs and run it with some other tests that we have
    /// to run in single-threaded mode.
    fn check_slug_collisions() {
        let mut set = HashSet::new();
        for _ in 0..1_200_000 {
            let id = cuid::slug().unwrap();
            set.insert(id);
        }
        // we had no duplicate slugs
        assert!(set.len() == 1_200_000);
    }
}
