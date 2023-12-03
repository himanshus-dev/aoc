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
	possible_sum := 0
	for l, line := range lines {
		game_not_possible := false

		// Get the Game ID
		gamePart := strings.Split(line, ":")[0]
		gameID, _ := strconv.Atoi(strings.TrimPrefix(gamePart, "Game "))
		line = strings.Split(line, ":")[1]

		// Playback all turns
		for _, turn := range strings.Split(line, ";") {
			rgb := []int{0, 0, 0}
			// Check all values
			for _, value := range strings.Split(turn, ",") {
				value := strings.TrimSpace(value)
				number, _ := strconv.Atoi(strings.Split(value, " ")[0])
				color := strings.Split(value, " ")[1]

				if color == "red" {
					rgb[0] = number
				} else if color == "green" {
					rgb[1] = number
				} else if color == "blue" {
					rgb[2] = number
				}
			}
			// Check if game is possible
			if !partOneChk(rgb) {
				game_not_possible = true
				break
			}
		}

		if !game_not_possible {
			fmt.Printf("[%v] Game %v is possible.\n", l+1, gameID)
			possible_sum += gameID
		}
	}
	// sum of gameIDs of all possible games
	fmt.Printf("Sum of the IDs of possible games? %v\n", possible_sum)
}

func partOneChk(rgb []int) bool {
	if rgb[0] > 12 || rgb[1] > 13 || rgb[2] > 14 {
		return false
	}
	return true
}

func partTwo(lines []string) {
	sum_of_powers := 0
	for l, line := range lines {
		rgb := []int{0, 0, 0}

		// Get the Game ID
		gamePart := strings.Split(line, ":")[0]
		gameID, _ := strconv.Atoi(strings.TrimPrefix(gamePart, "Game "))
		line = strings.Split(line, ":")[1]

		// Playback all turns
		for _, turn := range strings.Split(line, ";") {
			// Check all values
			for _, value := range strings.Split(turn, ",") {
				value := strings.TrimSpace(value)
				number, _ := strconv.Atoi(strings.Split(value, " ")[0])
				color := strings.Split(value, " ")[1]

				// Find highest i.e. minimum no of cubes required for the game
				if color == "red" && rgb[0] < number {
					rgb[0] = number
				} else if color == "green" && rgb[1] < number {
					rgb[1] = number
				} else if color == "blue" && rgb[2] < number {
					rgb[2] = number
				}
			}
		}

		cubeSetPower := calculateSetPower(rgb)
		fmt.Printf("[%v] Game %v requires %+v cubes with power %v.\n", l+1, gameID, rgb, cubeSetPower)
		sum_of_powers += cubeSetPower
	}
	// sum of gameIDs of all possible games
	fmt.Printf("Sum of the power of all sets? %v\n", sum_of_powers)
}

func calculateSetPower(rgb []int) (pow int) {
	pow = 1
	for _, x := range rgb {
		pow *= x
	}
	return
}
