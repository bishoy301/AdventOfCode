use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<i32> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    println!("Result: {}", input.iter().sum::<i32>())
}
