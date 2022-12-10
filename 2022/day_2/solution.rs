// AOC_2022_Day_2
// Problem: https://adventofcode.com/2022/day/2

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::any::type_name;

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
        let mut player_score = 0;
        for line in lines {
            if let Ok(ip) = line {
                // Split input by " "
                let t:Vec<&str> = ip.split_whitespace().collect();
                // Find and update score
                // player_score += rock_paper_scissors(t[0],t[1]);
                player_score += rock_paper_scissors_part_2(t[0],t[1]);
            }
        }
        println!("Max_Score: {}", player_score);
    }
}

fn rock_paper_scissors(opponent : &str, you: &str) -> i32 {
    let mut score = 0;
    // Item     - Code - Points
    // ------------------------
    // Rock     - A X  - 1
    // Paper    - B Y  - 2
    // Scissors - C Z  - 3
    // ------------------------
    // 
    // Result - Points
    // ------------------------
    // Win    - 6
    // Draw   - 3
    // Lost   - 0
    // ------------------------
    let x = (opponent, you);
    match x {
        ("A", "X") => score = 1 + 3,
        ("A", "Y") => score = 2 + 6,
        ("A", "Z") => score = 3 + 0,
        ("B", "X") => score = 1 + 0,
        ("B", "Y") => score = 2 + 3,
        ("B", "Z") => score = 3 + 6,
        ("C", "X") => score = 1 + 6,
        ("C", "Y") => score = 2 + 0,
        ("C", "Z") => score = 3 + 3,
        _ => println!("unknown input: {:?}", x),
    }
    println!("Part_1 => (Elf, You) => {:?} :: Score = {}", x, score);
    return score;
}

fn rock_paper_scissors_part_2(opponent : &str, result: &str) -> i32 {
    let mut score = 0;
    // Item     - Code - Points
    // ------------------------
    // Rock     - A  - 1
    // Paper    - B  - 2
    // Scissors - C  - 3
    // ------------------------
    // 
    // Code - Means - Points
    // ------------------------
    // X    - Lose   - 0
    // Y    - Draw   - 3
    // Z    - Win    - 0
    // ------------------------
    let x = (opponent, result);
    match x {
        ("A", "X") => score = 3 + 0,
        ("A", "Y") => score = 1 + 3,
        ("A", "Z") => score = 2 + 6,
        ("B", "X") => score = 1 + 0,
        ("B", "Y") => score = 2 + 3,
        ("B", "Z") => score = 3 + 6,
        ("C", "X") => score = 2 + 0,
        ("C", "Y") => score = 3 + 3,
        ("C", "Z") => score = 1 + 6,
        _ => println!("unknown input: {:?}", x),
    }
    println!("Part_2 => (Elf, You) => {:?} :: Score = {}", x, score);
    return score;
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}