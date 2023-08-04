fn main() {
    count_to_100();
}

fn count_to_100() {
    for i in 1..=100 {
        print!("{}, ", i);
        if (i % 10) == 0 {
            println!()
        }
    }
}
