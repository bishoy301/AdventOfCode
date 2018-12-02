use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<i32> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    println!("Result: {}", input.iter().sum::<i32>());

    let mut frequencies = HashSet::new();
    let freq = input
        .iter()
        .cycle()
        .scan(0, |freq, &input| {
            *freq += input;
            Some(*freq)
        })
        .find(|freq| !frequencies.insert(*freq))
        .unwrap();

    println!("Part 2 Result: {}", freq)
}
