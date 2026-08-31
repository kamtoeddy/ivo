// Concurrency semantics of `IvoRwCtxOptions::{read_sync, write_sync}` --
// unlike every other file in this directory, these tests reach directly into
// `ivo::__ivo_internals::IvoRwCtxOptions` instead of going through a
// `#[ivo_schema]`-generated model: `create`/`update` each own their
// `ctx_options` value outright (moved in per call), so two separate calls
// never share the same underlying lock -- there's no way to observe cross-
// call blocking/exclusion through the public schema API at all. The lock's
// actual concurrency behavior (multiple readers may hold it at once; a
// writer excludes every other reader/writer) is a property of
// `IvoRwCtxOptions` itself, backed by `async-lock`'s blocking primitives, so
// it's tested directly against real OS threads here instead.

use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

use ivo::__ivo_internals::IvoRwCtxOptions;

const TIMEOUT: Duration = Duration::from_secs(5);

#[test]
fn should_allow_multiple_readers_to_hold_read_sync_guards_concurrently() {
    // If `read_sync()` were (incorrectly) exclusive, only one of these
    // threads could ever hold its guard at a time -- the first to acquire
    // would then block forever waiting for `STARTED` to reach 3, since the
    // other two could never acquire their own guard to bump it. Bounded by
    // `TIMEOUT` so a real regression here fails loudly instead of hanging
    // the test run.
    static STARTED: AtomicUsize = AtomicUsize::new(0);
    const READERS: usize = 3;

    let opts: IvoRwCtxOptions<()> = IvoRwCtxOptions::new(());

    let handles: Vec<_> = (0..READERS)
        .map(|_| {
            let opts = opts.clone();
            thread::spawn(move || {
                let _guard = opts.read_sync();
                STARTED.fetch_add(1, Ordering::SeqCst);

                let deadline = Instant::now() + TIMEOUT;
                while STARTED.load(Ordering::SeqCst) < READERS {
                    if Instant::now() > deadline {
                        panic!(
                            "read_sync() guards did not overlap -- readers appear to be \
                             mutually exclusive"
                        );
                    }
                    thread::yield_now();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn should_block_read_sync_while_a_write_sync_guard_is_held() {
    let opts: IvoRwCtxOptions<i32> = IvoRwCtxOptions::new(0);

    let writer_holding = std::sync::Arc::new(AtomicBool::new(false));
    let reader_acquired = std::sync::Arc::new(AtomicBool::new(false));

    let writer = {
        let opts = opts.clone();
        let writer_holding = std::sync::Arc::clone(&writer_holding);
        thread::spawn(move || {
            let mut guard = opts.write_sync();
            writer_holding.store(true, Ordering::SeqCst);
            // Hold the write guard long enough for the reader below to
            // attempt (and block on) its own `read_sync()` call.
            thread::sleep(Duration::from_millis(200));
            *guard = 42;
        })
    };

    let deadline = Instant::now() + TIMEOUT;
    while !writer_holding.load(Ordering::SeqCst) {
        if Instant::now() > deadline {
            panic!("writer never acquired its write_sync() guard");
        }
        thread::yield_now();
    }

    let reader = {
        let opts = opts.clone();
        let reader_acquired = std::sync::Arc::clone(&reader_acquired);
        thread::spawn(move || {
            let guard = opts.read_sync();
            reader_acquired.store(true, Ordering::SeqCst);
            *guard
        })
    };

    // The writer is still sleeping inside its critical section at this
    // point -- a correctly-exclusive lock must not have let the reader in.
    thread::sleep(Duration::from_millis(50));
    assert!(
        !reader_acquired.load(Ordering::SeqCst),
        "read_sync() acquired a guard while a write_sync() guard was still held"
    );

    writer.join().unwrap();
    let observed = reader.join().unwrap();

    // The reader only got in *after* the writer released, so it must see
    // the writer's update -- proves ordering, not just eventual exclusion.
    assert_eq!(observed, 42);
}

#[test]
fn should_block_write_sync_while_another_write_sync_guard_is_held() {
    let opts: IvoRwCtxOptions<i32> = IvoRwCtxOptions::new(0);

    let first_holding = std::sync::Arc::new(AtomicBool::new(false));
    let second_acquired = std::sync::Arc::new(AtomicBool::new(false));

    let first = {
        let opts = opts.clone();
        let first_holding = std::sync::Arc::clone(&first_holding);
        thread::spawn(move || {
            let mut guard = opts.write_sync();
            first_holding.store(true, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(200));
            *guard = 1;
        })
    };

    let deadline = Instant::now() + TIMEOUT;
    while !first_holding.load(Ordering::SeqCst) {
        if Instant::now() > deadline {
            panic!("first writer never acquired its write_sync() guard");
        }
        thread::yield_now();
    }

    let second = {
        let opts = opts.clone();
        let second_acquired = std::sync::Arc::clone(&second_acquired);
        thread::spawn(move || {
            let mut guard = opts.write_sync();
            second_acquired.store(true, Ordering::SeqCst);
            *guard = 2;
        })
    };

    thread::sleep(Duration::from_millis(50));
    assert!(
        !second_acquired.load(Ordering::SeqCst),
        "a second write_sync() guard was acquired while the first was still held"
    );

    first.join().unwrap();
    second.join().unwrap();

    // Whichever writer ran last (deterministically the second here, since
    // it could only start after the first released) must be the value seen.
    assert_eq!(*opts.read_sync(), 2);
}
