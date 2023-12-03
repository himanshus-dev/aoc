package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	// Read input data
	filename := os.Args[1]
	fp, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer fp.Close()

	scanner := bufio.NewScanner(fp)
	var lines []string
	// Read each line
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}
	// Exit On Err
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	fmt.Println("=========== P A R T - O N E ===========")
	partOne(lines)
	fmt.Println("=========== P A R T - T W O ===========")
	partTwo(lines)
}

func partOne(lines []string) {
	first_digit := -1
	last_digit := -1
	cal_sum := 0

	for l, line := range lines {
		for idx := 0; idx < len(line); idx++ {
			// Check for Digits
			c, err := strconv.Atoi(string(line[idx]))
			if err == nil {
				// fmt.Printf("Character: %v at index: %d\n", c, idx)
				if first_digit == -1 {
					first_digit = c
				} else {
					last_digit = c
				}
			}
		}
		if last_digit == -1 {
			last_digit = first_digit
		}
		cal_val := (first_digit * 10) + last_digit
		fmt.Printf("[%v] Calibration value in line '%v' is %v\n", l+1, line, cal_val)
		cal_sum += cal_val

		// Reset vars
		first_digit = -1
		last_digit = -1
	}
	// Print the sum
	fmt.Printf("Sum of all of the calibration values? %v\n", cal_sum)
}

func partTwo(lines []string) {
	first_digit := -1
	last_digit := -1
	found_word := false
	cal_sum := 0

	num_strs := []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	// Read each line
	for l, line := range lines {
		for i := 0; i < len(line); i++ {
			// Check for num_strs
			for idx := 0; idx < len(num_strs); idx++ {
				if strings.HasPrefix(line[i:], num_strs[idx]) {
					// fmt.Printf("Word: %v found in line: '%v'\n", num_strs[idx], line[i:])
					if first_digit == -1 {
						first_digit = idx + 1
					} else {
						last_digit = idx + 1
					}
					found_word = true
					break
				}
			}
			if found_word {
				found_word = false
				continue
			}
			// Check for Digits
			c, err := strconv.Atoi(string(line[i]))
			if err == nil {
				// fmt.Printf("Character: %v at index: %d\n", c, i)
				if first_digit == -1 {
					first_digit = c
				} else {
					last_digit = c
				}
			}
		}
		if last_digit == -1 {
			last_digit = first_digit
		}
		cal_val := (first_digit * 10) + last_digit
		fmt.Printf("[%d] Calibration value in line '%v' is %v\n", l+1, line, cal_val)
		cal_sum += cal_val

		// Reset vars
		first_digit = -1
		last_digit = -1
		found_word = false
	}
	// Print the sum
	fmt.Printf("Sum of all of the calibration values? %v\n", cal_sum)
}
