fn main() {
    let count_total = 1_000_000_000;
    let count_total_string = format!("{}", count_total);
    let functions = vec!(count_by_1, count_by_fibonacci_nums, double_from_1);

    for function in functions {
        function(&count_total, &count_total_string);
    }
}

fn count_by_1(&count_total: &i32, count_total_string: &String) {
    println!("Counting by 1 to {}:", count_total_string);
    for i in 1..=count_total {
        if i % 100_000_000 == 0 {
            println!("{} ", i);
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
