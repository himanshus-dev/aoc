// AOC_2022_Day_4
// Problem: https://adventofcode.com/2022/day/4

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> + std::fmt::Debug, {
    println!("Reading input from file: {:?}", filename);
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // File input.txt must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        // part_1
        let mut overlapping_assignments = 0;
        for line in lines {
            if let Ok(ip) = line {
                // part_1
                // In how many assignment pairs does one range fully contain the other?
                // if camp_cleanup_collision(ip) {
                //     overlapping_assignments += 1;
                // }
                // part_2
                // In how many assignment pairs do the ranges overlap?
                if camp_cleanup_collision_part_two(ip) {
                    overlapping_assignments += 1;
                }
            }
        }
        println!("Overlapping_Assignments: {}", overlapping_assignments);
    }
}

fn camp_cleanup_collision(assignment: String) -> bool {
    // Split the line and find individual ranges
    let mut sec_1:Vec<i32> = Vec::new();
    let mut sec_2:Vec<i32> = Vec::new();
    let mut ctr = 0;
    println!("{}", assignment);
    for range in assignment.split(",") {
        let bounds:Vec<&str> = range.split("-").collect();
        let lb = bounds[0].parse::<i32>().unwrap_or(0);
        let ub = bounds[1].parse::<i32>().unwrap_or(0);
        if ctr == 0 {
            sec_1 = (lb..ub+1).collect();
            ctr = 1;
        } else {
            sec_2 = (lb..ub+1).collect();
            ctr = 0;
        }
    }
    let result = (sec_1.iter().all(|item| sec_2.contains(item))) || (sec_2.iter().all(|item| sec_1.contains(item)));
    println!("{}:: {:?} :: {:?}", result, sec_1, sec_2);
    // Check if the sections are inclusive
    return result;
}

fn camp_cleanup_collision_part_two(assignment: String) -> bool {
    // Split the line and find individual ranges
    let mut sec_1:[i32; 2] = [0, 0];
    let mut sec_2:[i32; 2] = [0, 0];
    let mut ctr = 0;
    println!("{}", assignment);
    for range in assignment.split(",") {
        let bounds:Vec<&str> = range.split("-").collect();
        let lb = bounds[0].parse::<i32>().unwrap_or(0);
        let ub = bounds[1].parse::<i32>().unwrap_or(0);
        if ctr == 0 {
            sec_1[0] = lb;
            sec_1[1] = ub;
            ctr = 1;
        } else {
            sec_2[0] = lb;
            sec_2[1] = ub;
            ctr = 0;
        }
    }
    // Check if the sections are overlapping
    let mut result = false;
    // if LB are equal or inclusive
    if (sec_1[0] <= sec_2[0] && sec_2[0] <= sec_1[1]) || (sec_1[0] <= sec_2[1] && sec_2[1] <= sec_1[1]) {
        result = true;
    }
    // if UB are equal or inclusive
    if (sec_2[0] <= sec_1[0] && sec_1[0] <= sec_2[1]) || (sec_2[0] <= sec_1[1] && sec_1[1] <= sec_2[1]) {
        result = true;
    }
    println!("{}:: {:?} :: {:?}", result, sec_1, sec_2);
    return result;
}