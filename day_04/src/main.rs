// Day 4: Repose Record
// Date: April 20th
// Author: Yuki

use std:: {
    fs::File,
    io::Read,
    collections::HashMap,
    time::{Duration, Instant}
};
const FILE: &str = "input.txt";

fn main() {
    let mut input = File::open(FILE).expect("Error: Cannot open file.");
    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Error: Reading file.");
    let (result_1, benchmark_1) = do_part_1(&contents);
    let (result_2, benchmark_2) = do_part_2(&contents);

    println!("Part 1: The amount of overlap is {}\nBenchmark: {:.3?}",
    result_1, benchmark_1);
    println!("-----------------------------------------------");
    println!("Part 2: The non-overlapping claim ID is {}\nBenchmark: {:.3?}",
    result_2, benchmark_2);
}

fn do_part_1() {

}

fn do_part_2() {
    
}