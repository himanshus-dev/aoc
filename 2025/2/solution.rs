use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Read input
    let lines: Vec<String> = read_file(&args[1]);

    // Extract Ranges
    let ranges: Vec<&str> = lines[0].split_inclusive(',').collect();
    // Do whatever needs to be done
    let mut twice_sum: u64 = 0;
    let mut duplicate_sum: u64 = 0;

    for mut range in ranges {
        println!("{range}");
        range = range.trim_matches(',');
        // Extract Start and End
        let [start, end]: [u64; 2] = range
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();

        let duplicates: Vec<u64> = find_invalid_ids(start, end);
        let twice: Vec<u64> = find_invalid_ids_2(start, end);
        println!(
            "Start: {start}, End: {end}, Duplicates: {:?}, Twice: {:?}",
            duplicates, twice
        );

        // Part1 - Find sum of duplicaes
        for num in duplicates {
            duplicate_sum += num
        }
        // Part 2 - Find sum of all ids with repeating substrings
        for num in twice {
            twice_sum += num
        }
    }
    println!("Duplicate Sum: {duplicate_sum}");
    println!("Twice Sum: {twice_sum}");
}

fn find_invalid_ids(start: u64, end: u64) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = Vec::new();

    for n in start..=end {
        if has_duplicates(&n.to_string()) {
            invalid_ids.push(n);
        }
    }
    invalid_ids
}

fn has_duplicates(s: &str) -> bool {
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }
    let (l, r) = s.split_at(len / 2);
    let a = l.parse::<u64>().unwrap();
    let b = r.parse::<u64>().unwrap();
    a == b
}

fn find_invalid_ids_2(start: u64, end: u64) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = Vec::new();

    for n in start..=end {
        if has_twice_repeating(&n.to_string()) {
            invalid_ids.push(n);
        }
    }
    invalid_ids
}

fn has_twice_repeating(s: &str) -> bool {
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            if (0..len)
                .step_by(pattern_len)
                .all(|i| &s[i..i + pattern_len] == pattern)
            {
                return true;
            }
        }
    }
    false
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
