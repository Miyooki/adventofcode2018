// Day 2: Inventory Management System
// Date: April 19th
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

    println!("Part 1: The candidate checksum is {}\nBenchmark: {:.3?}",
    result_1, benchmark_1);
    println!("-----------------------------------------------");
    println!("Part 2: The correct box ID is {}\nBenchmark: {:.3?}",
    result_2, benchmark_2);
}

fn do_part_1(contents: &str) -> (u64, Duration) {
    let start = Instant::now();
    let mut candidate_check = HashMap::new();
    let (mut doubles_count, mut triples_count) = (0, 0);
    for line in contents.lines() {
        for chars in line.chars() {
            let count = candidate_check.entry(chars).or_insert(0);
            *count += 1;
        }
        let (mut doubles, mut triples) = (false, false);
        for val in candidate_check.values() {
            if doubles && triples {
                break; // Save some run time.
            } else if *val == 2 && !doubles {
                doubles = true;
                doubles_count += 1;
            } else if *val == 3 && !triples {
                triples = true;
                triples_count += 1;
            }
        }
        candidate_check.clear();
    }
    let end = Instant::now().duration_since(start);
    return (doubles_count * triples_count, end)
}

fn do_part_2(contents: &str) -> (String, Duration) {
    let start = Instant::now();
    let mut box_id = String::new();
    let mut content_index = 0;
    println!("Total: {}", contents.len());
    for line in contents.lines() {
        content_index += line.len() + 2; 
        // The +2 is for the newline character that is trimmed by lines().
        for compare in contents[content_index..].lines() {
            let mut errors = 0;
            let mut compare_chars = compare.chars();
            for chars in line.chars() {
                if errors > 1 {
                    break; // Save some run time.
                } else if Some(chars) != compare_chars.next() {
                    errors += 1;      
                } else {
                    box_id.push(chars);
                    // I'm not sure what the best way to remove a char from a string is...
                    // So I decided to skip adding the error char to box_id.
                }
            }
            if errors == 1 {
                let end = Instant::now().duration_since(start);
                return (box_id, end)
            }
            box_id.clear();
        }
    }
    // Didn't find any similar box ids. Fail safe return.
    let end = Instant::now().duration_since(start);
    return (box_id, end)
}
