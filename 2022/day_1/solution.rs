// AOC_2022_Day_1
// Problem: https://adventofcode.com/2022/day/1

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::any::type_name;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File input.txt must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut max_cal = 0;
        let mut curr_elf = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if curr_elf > max_cal {
                        max_cal = curr_elf;
                    }
                    curr_elf = 0;
                } else {
                    let line_int = ip.parse::<i32>().unwrap_or(0);
                    curr_elf += line_int;
                }
            }
        }
        println!("Max_Carried_Calories: {}", max_cal);
    }
}
