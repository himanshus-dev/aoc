// AOC_2022_Day_3
// Problem: https://adventofcode.com/2022/day/3

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
        let mut sum_of_priorities = 0;
        // part_2
        let mut group_of_elves:Vec<String> = Vec::new();
        let mut elves_in_group = 0;
        for line in lines {
            if let Ok(ip) = line {
                // part_1
                // sum_of_priorities += rucksack_reorganization(ip);
                // println!("------------------------------------");
                // part_2
                if group_of_elves.len() <= elves_in_group {
                    group_of_elves.push(ip.clone());
                } else {
                    group_of_elves[elves_in_group] = ip.clone();
                }
                elves_in_group += 1;
                if elves_in_group == 3 {
                    sum_of_priorities += rucksack_reorganization_part_two(&group_of_elves);
                    println!("------------------------------------");
                    elves_in_group = 0;
                }
            }
        }
        // What is the sum of the priorities of common item types?
        println!("Sum_of_Priorities: {}", sum_of_priorities);
    }
}

fn rucksack_reorganization(items: String) -> i32 {
    // Each rucksack has two large compartments => compartment_size = len(ip)/2
    let compartment_size = items.len() / 2;
    let compartment_1 = &items[..compartment_size];
    let compartment_2 = &items[compartment_size..];
    // Find item type that appears in both compartments
    for (_, c) in compartment_1.chars().enumerate() {
        // use the character `c`
        let x = compartment_2.find(c);
        // println!("Finding '{}' = {:?}", c, x);
        if x != None {
            println!("{} :: {} :: {}", c, compartment_1, compartment_2);
            return char_to_priority(c);
        }
    }
    println!("Nothing :: {} :: {}", compartment_1, compartment_2);
    return 0;
}

fn rucksack_reorganization_part_two(rucksacks: &Vec<String>) -> i32 {
    println!("Group: {:?}", rucksacks);
    let (rs_1, rs_2, rs_3) = (&rucksacks[0], &rucksacks[1], &rucksacks[2]);
    // Find one item type that is common between all three Elves in each group
    for (_, c) in rs_1.chars().enumerate() {
        // use the character `c`
        if rs_2.find(c) == None {
            continue;
        }
        if rs_3.find(c) != None {
            println!("{} :: {} :: {} :: {}", c, rs_1, rs_2, rs_3);
            return char_to_priority(c);
        }
    }
    return 0;
}

fn char_to_priority(c: char) -> i32 {
    // Every item type can be converted to a priority:
    // - Lowercase item types a through z have priorities 1 through 26.
    // - Uppercase item types A through Z have priorities 27 through 52.
    if c.is_lowercase() {
        // println!("'{}' = {} :: 'a' = {}", c, c as u32, 'a' as u32);
        let priority = (c as u32 - 'a' as u32 + 1) as i32;
        println!("Priority of '{}': {}", c, priority);
        return priority;
    }
    if c.is_uppercase() {
        // println!("'{}' = {} :: 'A' = {}", c, c as u32, 'A' as u32);
        let priority = (c as u32 - 'A' as u32 + 1 + 26) as i32;
        println!("Priority of '{}': {}", c, priority);
        return priority;
    }
    println!("Invalid Char: {}", c);
    return 0;
}
