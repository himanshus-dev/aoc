use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dial {
    state: i32,
    zero_values: i32,
    zero_passed: i32,
}

impl Dial {
    const MIN: i32 = 0;
    const MAX: i32 = 99;

    pub fn new(initial_state: i32) -> Result<Self, String> {
        if initial_state > Self::MAX {
            return Err(format!("State must be between {} and {}", Self::MIN, Self::MAX));
        }
        Ok(Self {
            state: initial_state,
            zero_values: 0,
            zero_passed: 0,
        })
    }

    pub fn rotate(&mut self, input: String) {
        let direction = input.chars().nth(0).unwrap();
        let num_clicks = input.trim_matches(direction).parse::<i32>().unwrap_or(0);

        print!("State: {}; Input: {}; ", self.state, input);
        match direction {
            'L' => Self::_rotate_left(self, num_clicks),
            'R' => Self::_rotate_right(self, num_clicks),
            _ => println!("Invalid Input: {:?}", input),
        }
        println!("Update_State: {}; ZV: {}; ZP: {}", self.state, self.zero_values, self.zero_passed)
    }

    fn _rotate_right(&mut self, mut value: i32) {
        while self.state + value >= Self::MAX + 1 {
            let r_value = (self.state + value) - (Self::MAX + 1);
            self.state = Self::MIN;
            // Track Wrap arounds
            if r_value != 0 {
                self.zero_passed += 1
            }
            value = r_value;
        }
        // Update state
        self.state += value;
        // Track if value is zero
        if self.state == 0 {
            self.zero_values += 1
        }
    }
    fn _rotate_left(&mut self, mut value: i32) {
        while self.state - value < Self::MIN {
            let l_value = ((self.state - value) + 1) * (-1);
            // Track Wrap arounds
            if self.state != 0 {
                self.zero_passed += 1
            }
            self.state = Self::MAX;
            value = l_value;
        }
        // Update state
        self.state -= value;
        // Track if value is zero
        if self.state == 0 {
            self.zero_values += 1
        }
    }

    pub fn get_zv(&self) -> i32 {
        self.zero_values
    }

    pub fn get_zp(&self) -> i32 {
        self.zero_passed
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Read input
    let lines: Vec<String> = read_file(&args[1]);

    // State Machine
    let initial_state = 50;
    let mut dial = Dial::new(initial_state).unwrap();
    for input in lines {
        dial.rotate(input)
    }
    println!("Passcode is {:?}", dial.get_zv());
    println!("Using password method 0x434C49434B, Passcode is {:?}", dial.get_zv() + dial.get_zp());
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
