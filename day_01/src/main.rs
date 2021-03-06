// Day 1: Chronal Calibration
// Date: April 18th, 2019
// Author: Yuki

use std:: {
    fs::File,
    io::Read,
    collections::HashSet,
    time::{Duration, Instant}
};
const FILE: &str = "input.txt";

fn main() {
    let mut input = File::open(FILE).expect("Error: Cannot open file.");
    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Error: Reading file.");
    let (result_1, benchmark_1) = do_part_1(&contents);
    let (result_2, benchmark_2) = do_part_2(&contents);

    println!("Part 1: The resulting frequency is {}\nBenchmark: {:.3?}",
    result_1, benchmark_1);
    println!("-----------------------------------------------");
    println!("Part 2: The first repeating frequency is {}\nBenchmark: {:.3?}",
    result_2, benchmark_2);
}

fn do_part_1(contents: &str) -> (i32, Duration) {
    let start = Instant::now();
    let mut frequency = 0;
    for line in contents.lines() {
        frequency += line.parse::<i32>().expect("Error: Not a number.");
    }
    let end = Instant::now().duration_since(start);
    return (frequency, end)
}

/// I used a HashSet to store frequencies as I do not need indexing.
fn do_part_2(contents: &str) -> (i32, Duration) {
    let start = Instant::now();
    let mut frequency_set = HashSet::new();
    frequency_set.insert(0); // It starts at 0.
    let mut frequency = 0;
    loop {
        for line in contents.lines() {
            frequency += line.parse::<i32>().expect("Error: Not a number.");
            if frequency_set.contains(&frequency) {
                let end = Instant::now().duration_since(start);
                return (frequency, end)
            }
            frequency_set.insert(frequency);
        }
    }
}
