use std::time::Instant;
use std::iter::repeat;

fn main() {
    // TODO: add commas to printed numbers 
    let count_total = 1_000_000_000;
    let count_total_string = format!("{}", count_total);
    let indent: String = repeat(" ").take(4).collect();
    let functions = vec!(count_by_1, count_by_fibonacci_nums, double_from_1);

    // Run counting functions consecutively
    let start_time = Instant::now();
    for function in functions {
        function(&count_total, &count_total_string);
    }
    let elapsed_time = start_time.elapsed().as_secs();

    // Display results
    println!(
"\nTotal time taken to run all counting functions:
{0}Consecutively: {1}
{0}Concurrently: _ seconds (future feature)",
        indent, elapsed_time
    );
}

fn count_by_1(&count_total: &i32, count_total_string: &String) {
    println!("Counting by 1 to {}:", count_total_string);
    for i in 1..=count_total {
        // Print progress every time another 10% of the total is counted
        if i % (count_total as f64 * 0.1) as i32 == 0 {
            let percentage_complete = (i as f64 / count_total as f64) * 100.0;
            println!("{}%: {} ", percentage_complete, i);
        }
    }
}

fn count_by_fibonacci_nums(
    &count_total: &i32,
    count_total_string: &String
) {
    let mut i: i32 = 1;
    let mut current: i32 = 1;
    let mut prev: i32 = 0;
    
    println!("\nCounting using fibonacci numbers from 1 to \
        the closest number >= {}:", {count_total_string});
    loop {
        // Print "prev" then compare to "count_total" before continuing to avoid
        // unnecessary processing
        println!("{}: {}", (i).to_string(), prev);
        if prev > count_total {
            break;
        }

        let next: i32 = prev + current;

        prev = current;
        current = next;
        i += 1;
    }
}

fn double_from_1(&count_total: &i32, count_total_string: &String) {
    let mut i = 1;

    println!("\nDoubling from 1 to the closest number >= {}:", count_total_string);
    loop {
        println!("{} ", i);

        if i > count_total {
            break;
        }
        i *= 2;
    }
}
