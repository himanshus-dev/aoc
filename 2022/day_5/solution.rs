// AOC_2022_Day_5
// Problem: https://adventofcode.com/2022/day/5

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
        // N number of stacks
        let mut stack_lines:Vec<String> = Vec::new();
        let mut stacks:Vec<Vec<char>> = Vec::new();
        let mut parse_rule:bool = false;
        for line in lines {
            if let Ok(ip) = line {
                // Stacks initial arrangement end with a empty line
                if ip == "" {
                    stacks = build_stacks(&stack_lines);
                    parse_rule = true;
                    continue;
                }
                // Modify stack as per the instructions
                if parse_rule {
                    // part_1 - After the rearrangement procedure completes, what crate ends up on top of each stack?
                    // supply_stacks_rearrange(&mut stacks, ip);
                    // part_2 - Crane can move multiple crates at once
                    supply_stacks_rearrange_part_two(&mut stacks, ip);
                    continue;
                }
                stack_lines.push(ip);
            }
        }
        // print_stacks(&stacks);
        println!("Crates_At_Top: {}", crates_at_top(&stacks));
    }
}

fn supply_stacks_rearrange(stacks:&mut Vec<Vec<char>>, rule: String) {
    // Parse the rule
    // Format: move N from SRC to TRG
    let mut curr_num:String = "".to_string();
    let mut rule_values:[usize; 3] = [0, 0, 0];
    let mut idx:usize = 0;
    for c in rule.chars() {
        if c.is_numeric() {
            curr_num.push(c);
            continue;
        }
        if c.is_whitespace() {
            if curr_num != "" {
                rule_values[idx] = curr_num.parse::<usize>().unwrap_or(0);
                idx += 1;
                curr_num = "".to_string();
            }
        }
    }
    // push last value
    if curr_num != "" {
        rule_values[idx] = curr_num.parse::<usize>().unwrap_or(0);
    }
    println!("{:?} => {:?}", rule, rule_values);
    // Crates are moved one at a time
    for _ in 0..rule_values[0] {
        // Perform the action
        let poped_item = stacks[rule_values[1]-1].pop().unwrap();
        stacks[rule_values[2]-1].push(poped_item);
    }
    println!("{:?} => {:?}", rule, stacks);
}

fn supply_stacks_rearrange_part_two(stacks:&mut Vec<Vec<char>>, rule: String) {
    // Parse the rule
    // Format: move N from SRC to TRG
    let mut curr_num:String = "".to_string();
    let mut rule_values:[usize; 3] = [0, 0, 0];
    let mut idx:usize = 0;
    for c in rule.chars() {
        if c.is_numeric() {
            curr_num.push(c);
            continue;
        }
        if c.is_whitespace() {
            if curr_num != "" {
                rule_values[idx] = curr_num.parse::<usize>().unwrap_or(0);
                idx += 1;
                curr_num = "".to_string();
            }
        }
    }
    // push last value
    if curr_num != "" {
        rule_values[idx] = curr_num.parse::<usize>().unwrap_or(0);
    }
    println!("{:?} => {:?}", rule, rule_values);
    // Crane can move multiple crates at once and they retain order
    let mut tmp_crate:Vec<char> = Vec::new();
    // Bad hack - move things to tmp stack and then to TRG
    for _ in 0..rule_values[0] {
        // Perform the action
        let poped_item = stacks[rule_values[1]-1].pop().unwrap();
        tmp_crate.push(poped_item);
    }
    // move things to TRG crate
    for _ in 0..rule_values[0] {
        let poped_item = tmp_crate.pop().unwrap();
        stacks[rule_values[2]-1].push(poped_item);
    }
    println!("{:?} => {:?}", rule, stacks);
}
fn build_stacks(stack_lines:&Vec<String>) -> Vec<Vec<char>> {
    let mut stacks:Vec<Vec<char>> = Vec::new();
    for line in stack_lines.iter().rev() {
        println!("{:?}", line);
        let mut ctr = 0;
        for i in (1..line.len()-1).step_by(4) {
            // println!("{} ({}) '{}'", ctr, i, line.chars().nth(i).unwrap());
            let char_to_push = line.chars().nth(i).unwrap();

            println!(":: What to do with {:?} at ctr = {}; stack_len = {}", char_to_push, ctr, stacks.len());
            if stacks.len() >= ctr + 1 {
                // Push to existing
                println!(":: Pushing {:?} to stack at {}", char_to_push, ctr);
                if char_to_push != ' ' {
                    stacks[ctr].push(char_to_push);
                }
            } else {
                // Create new Vec<String>
                println!(":: Pushing {:?} new stack at {}", char_to_push, ctr);
                let mut new_stack = Vec::<char>::new();
                if char_to_push != ' '{
                    new_stack.push(char_to_push);
                }
                stacks.push(new_stack);
            }
            ctr += 1;
        }
    }
    println!("Stak Built :: {:?}", stacks);
    return stacks;
}

fn crates_at_top(stacks:&Vec<Vec<char>>) -> String {
    let mut top:String = String::new();
    for stack in stacks {
        top.push(*stack.last().unwrap());
    }
    return top;
}