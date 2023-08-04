fn main() {
    count_to_100_million();

}

fn count_to_100_million() {
    let total_i32 = 100000000;
    let total_string = format!("{}", total_i32);
    println!("Counting to {}...", total_string);
    for i in 1..=total_i32 {
        if i == total_i32 {
            print!("Finished counting to {}", total_string);
        }
    }
}
