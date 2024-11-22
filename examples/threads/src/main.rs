use kos::dbglog::DbgLevel::Info;
use kos::dbglog;
use std::thread;
use std::time::Duration;

const NUM_THREADS: usize = 50;

fn main() {
    dbglog!(Info, "Hello, world from Rust! - threads example\n");
    dbglog!(Info, "Spawning {NUM_THREADS} threads!\n");

    let mut handles = Vec::with_capacity(NUM_THREADS);

    for i in 0..NUM_THREADS {
        let handle = thread::Builder::new()
            .name(i.to_string())
            .stack_size(32 * 1024)
            .spawn(move || {
                dbglog!(Info, "Thread {i} spawned!\n");
                thread::sleep(Duration::from_micros(50));
                dbglog!(Info, "Thread {i} says, \"Hello!\"\n");
                thread::sleep(Duration::from_micros(100));
                dbglog!(Info, "Thread {i} says \"Goodbye!\"\n");
                thread::sleep(Duration::from_micros(200));
            })
            .expect("Failed to spawn thread!");
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    dbglog!(Info, "All threads have finished.\n");
    dbglog!(Info, "Bye!\n");
}
