#![no_main]
extern crate kos;
use std::time::{Duration, Instant};
use tokio::runtime::Builder;
use tokio::time::sleep;

const CALC_FACTORIAL: usize = 10;
const NUM_THREADS: usize = 10; 
const THD_STACK_SIZE: usize = 32 * 1024;

async fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }

    let mut result = 1;

    for i in 2..=n {
        result *= i;

        // Simulate some asynchronous operation
        sleep(Duration::from_secs(1)).await;
    }

    result
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello, world from Rust! - tokio async example");

    let test_num = CALC_FACTORIAL;

    println!("Calculating factorial of {test_num}");

    let start_time = Instant::now();

    let rt = Builder::new_multi_thread()
        .worker_threads(NUM_THREADS)
        .thread_stack_size(THD_STACK_SIZE)
        .enable_all()
        .build()
        .unwrap();

    let future = async {
        let result = factorial(test_num).await;
        println!("Factorial: {}", result);
    };

    rt.block_on(future);

    println!("Time elapsed: {:?}", start_time.elapsed());

    println!("Bye!");

    0
}
