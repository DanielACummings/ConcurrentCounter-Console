use std::iter::repeat;
use std::time::Instant;
use tokio::join;
use tokio::runtime::Runtime;

fn main() {
    // TODO: add commas to printed numbers 
    let count_total: i32 = 1_000_000_000;
    let indent: String = repeat(" ").take(4).collect();
    let function_message: &str =
"running the following counting functions:
    count_by_1
    count_by_2
    count_by_fibonacci_nums";

    // Run counting functions concurrently
    let mut start_time: Instant = Instant::now();
    let rt: Runtime = Runtime::new().unwrap();
    let future = run_concurrently(&count_total);

    println!("Concurrently {}", function_message);
    rt.block_on(future);
    let elapsed_concurrent_time: u64 = start_time.elapsed().as_secs();

    // Run counting functions consecutively
    start_time = Instant::now();
    println!("Consecutively {}", function_message);
    run_consecutively(&count_total);
    let elapsed_consecutive_time: u64 = start_time.elapsed().as_secs();

    // Display results
    println!(
"\nTotal time taken to run all counting functions:
{0}Concurrently: {1} seconds
{0}Consecutively: {2} seconds",
        indent, elapsed_concurrent_time, elapsed_consecutive_time
    );
}

async fn count_by_1(&count_total: &i32) -> () {
    for _ in 1..=count_total {
        continue;
    }
}

async fn count_by_2(&count_total: &i32) -> () {
    let mut i = 2;

    loop {
        if i > count_total {
            break;
        }
        i += 2;
    }
}

async fn count_by_fibonacci_nums(&count_total: &i32) -> () {
    let mut current: i32 = 1;
    let mut prev: i32 = 0;
    
    loop {
        if prev > count_total {
            break;
        }
        let next: i32 = prev + current;
        prev = current;
        current = next;
    }
}

async fn run_concurrently(&count_total: &i32) {
    join!(
        count_by_1(&count_total),
        count_by_2(&count_total),
        count_by_fibonacci_nums(&count_total),
    );
}

fn run_consecutively(&count_total: &i32) {
    Runtime::new().unwrap().block_on(async {
        count_by_1(&count_total).await;
        count_by_2(&count_total).await;
        count_by_fibonacci_nums(&count_total).await;
    });
}
