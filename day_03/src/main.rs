// Day 3: No Matter How You Slice It
// Date: April 19th
// Author: Yuki

use std:: {
    fs::File,
    io::Read,
    collections::HashMap,
    time::{Duration, Instant}
};
const FILE: &str = "input.txt";
const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let mut input = File::open(FILE).expect("Error: Cannot open file.");
    let mut contents = String::new();
    input.read_to_string(&mut contents).expect("Error: Reading file.");
    let (result_1, benchmark_1) = do_part_1(&contents);
    let (result_2, benchmark_2) = do_part_2(&contents);

    println!("Part 1: The amount of overlap is {}\nBenchmark: {:.3?}",
    result_1, benchmark_1);
    println!("-----------------------------------------------");
    println!("Part 2: The correct box ID is {}\nBenchmark: {:.3?}",
    result_2, benchmark_2);
}

fn do_part_1(contents: &str) -> (u64, Duration) {
    let start = Instant::now();
    // Tried using Box but it kept overflowing the stack, even with the release build.
    let mut fabric = vec![[0 as u8; WIDTH]; HEIGHT].into_boxed_slice(); 
    let mut overlap = 0;
    for line in contents.lines() {
        // I didn't add the claim IDs to each patch because I didn't need to.
        let tokenize: Vec<&str> = line.split(|c| c == ' ' || c == ':').collect();
        // Array of 5 elements, the 4th is a blank string due to ':' char.
        let position: Vec<usize> = tokenize[2].
            split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        // Position is [x, y], Dimension is [width, height].
        let dimensions: Vec<usize> = tokenize[4].
            split('x').map(|s| s.parse::<usize>().unwrap()).collect();
        for width in 0..dimensions[0] {
            for height in 0..dimensions[1] {
                let vec_ptr = &mut fabric[position[0] + width][position[1] + height];
                if *vec_ptr == 0 {
                    *vec_ptr = 1;
                // Dummy fix to set the value to 2 so I don't get additional overlaps.
                } else if *vec_ptr == 1{
                    overlap += 1;
                    *vec_ptr = 2;
                }
            }
        }
    }
    let end = Instant::now().duration_since(start);
    return (overlap, end)
}

fn do_part_2(contents: &str) -> (u16, Duration) {
     // Pretty much reused the code from Part 1, but now with claim IDs implemented.
    let start = Instant::now();
    let mut fabric = vec![[0 as u16; WIDTH]; HEIGHT].into_boxed_slice();
    let mut claim_id = 0;
    let mut overlap_set = HashMap::new();
    for line in contents.lines() {
        let tokenize: Vec<&str> = line.split(|c| c == ' ' || c == '#' || c == ':').collect();
        // Array of 6 elements now, the 1st and 4th is a blank string due to '#' and ':' chars.
        claim_id = tokenize[1].parse::<u16>().unwrap();
        let position: Vec<usize> = tokenize[3].
            split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        // Position is [x, y], Dimension is [width, height].
        let dimensions: Vec<usize> = tokenize[5].
            split('x').map(|s| s.parse::<usize>().unwrap()).collect();
        overlap_set.insert(claim_id, 0); // Initialize the value to 0 for each ID.
        for width in 0..dimensions[0] {
            for height in 0..dimensions[1] {
                let vec_ptr = &mut fabric[position[0] + width][position[1] + height];
                if *vec_ptr == 0 {
                    *vec_ptr = claim_id;
                } else if *vec_ptr > 0 {
                    overlap_set.insert(claim_id, 1);
                    overlap_set.insert(*vec_ptr, 1);
                    *vec_ptr = claim_id;
                }
            }
        }   
    }
    for key in 1..claim_id + 1 {
        if overlap_set.get(&key).unwrap() == &0 {
            // 0 for no overlaps, 1 for overlaps.
            let end = Instant::now().duration_since(start);
            return (key, end)
        }
    }
    // Didn't find any claims that were not overlapping. Fail safe return.
    let end = Instant::now().duration_since(start);
    return (0, end)
}
