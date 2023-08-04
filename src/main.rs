fn main() {
    let count_total = 1_000_000_000;
    let count_total_string = format!("{}", count_total);
    count_by_1(&count_total, &count_total_string);
    double_from_1(&count_total, &count_total_string);
}

fn count_by_1(&count_total: &i32, count_total_string: &String) {
    println!("Counting to {}...", count_total_string);

    for i in 1..=count_total {
        if i % 100_000_000 == 0 {
            println!("{} ", i);
        }
        if i == count_total {
            println!("Finished counting");
        }
    }
}

fn double_from_1(&count_total: &i32, count_total_string: &String) {
    println!("\nDoubling from 1 the closed doubled number >= {}...", count_total_string);
    let mut i = 1;
 
    while i < count_total {
        i *= 2;
        println!("{} ", i);
    }
    print!("Finished counting");
}