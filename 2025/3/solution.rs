use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Read input
    let lines: Vec<String> = read_file(&args[1]);

    // Part 1
    total_max_joltage(lines.clone());
    // Part 2
    total_largest_joltage(lines);
}

fn total_max_joltage(lines: Vec<String>) {
    let mut total_max_joltage: u32 = 0;
    for line in lines {
        // Extract battery bank
        let battery_bank: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        print!("Battery_Bank: [{line}]; Cells: {:?}; ", battery_bank.len());

        // Basically find largest and second largest - but preserve order
        let mut first_idx = 0;
        for i in 1..battery_bank.len() - 1 {
            if battery_bank[i] > battery_bank[first_idx] {
                first_idx = i;
            }
        }

        let mut second_idx = first_idx + 1;
        for i in (second_idx + 1)..battery_bank.len() {
            if battery_bank[i] > battery_bank[second_idx] {
                second_idx = i;
            }
        }
        let max_joltage = (battery_bank[first_idx] * 10) + battery_bank[second_idx];
        println!("Max_Jolatge: {max_joltage}");

        total_max_joltage += max_joltage;
    }
    println!("Total Output Jolatge: {total_max_joltage}");
}

fn total_largest_joltage(lines: Vec<String>) {
    let mut total_largest_joltage: u64 = 0;
    for line in lines {
        // Extract battery bank
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        print!("Battery_Bank: [{line}]; Cells: {:?}; ", len);

        let to_remove = len - 12;
        let mut result: Vec<char> = chars.clone();

        for _ in 0..to_remove {
            // Find the smallest digit that should be removed
            // Remove the first character that is smaller than its next character
            // If no such character exists, remove the last one
            let mut removed = false;

            for i in 0..result.len() - 1 {
                if result[i] < result[i + 1] {
                    result.remove(i);
                    removed = true;
                    break;
                }
            }

            if !removed {
                result.pop();
            }
        }

        let max_number_str: String = result.iter().collect();
        let max_number: u64 = max_number_str.parse().unwrap();
        println!("Max_Number: {max_number}");

        total_largest_joltage += max_number;
    }
    println!("Largest Output Jolatge: {total_largest_joltage}");
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
