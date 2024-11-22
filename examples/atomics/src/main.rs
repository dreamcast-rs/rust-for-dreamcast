use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    println!("Hello, world from Rust! - atomics example");

    // Create a new atomic u32 value
    let atomic_counter = Arc::new(AtomicU32::new(0));

    // Clone Arc for each thread
    let mut handles = vec![];
    for _ in 0..5 {
        let atomic_counter_clone = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // Atomically increment the value
                atomic_counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Get the final value of the atomic counter
    let final_value = atomic_counter.load(Ordering::SeqCst);
    println!("Final value: {}", final_value);

    println!("Bye!");
}
