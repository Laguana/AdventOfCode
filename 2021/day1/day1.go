package day1

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
)

func CountIncrements(measurements []int) int {
	var increments = 0
	var currentElement = measurements[0]
	for _, element := range measurements {
		if element > currentElement {
			increments++
		}
		currentElement = element
	}
	return increments
}

func ParseInputs(input []string) ([]int, error) {
	var result = make([]int, len(input))
	for i, v := range input {
		var err error
		result[i], err = strconv.Atoi(v)
		if err != nil {
			return nil, err
		}
		if result[i] < 0 {
			return nil, fmt.Errorf("created a negative value: %d", result[i])
		}

	}
	return result, nil
}

func Part1(r io.Reader) (int, error) {

	inputs := make([]string, 0)
	scanner := bufio.NewScanner(r)

	for {
		scanner.Scan()
		text := scanner.Text()
		if len(text) == 0 {
			break
		}

		inputs = append(inputs, text)
	}

	intInputs, err := ParseInputs(inputs)
	if err != nil {
		return 0, err
	}

	increments := CountIncrements(intInputs)

	return increments, nil
}

func CountSlidingIncrements(measurements []int) int {
	var increments = 0
	var currentSum = measurements[0] + measurements[1] + measurements[2]
	for i := 1; i < len(measurements)-2; i++ {
		newSum := measurements[i] + measurements[i+1] + measurements[i+2]
		if newSum > currentSum {
			increments++
		}
		currentSum = newSum
	}
	return increments
}

func Part2(r io.Reader) (int, error) {
	inputs := make([]string, 0)
	scanner := bufio.NewScanner(r)

	for {
		scanner.Scan()
		text := scanner.Text()
		if len(text) == 0 {
			break
		}

		inputs = append(inputs, text)
	}

	intInputs, err := ParseInputs(inputs)
	if err != nil {
		return 0, err
	}

	increments := CountSlidingIncrements(intInputs)

	return increments, nil
}
