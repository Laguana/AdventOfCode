package day6

import (
	"AoC2021/common"
	"io"
	"strconv"
	"strings"
)

func parseInput(input []string) ([]int, error) {
	result := make([]int, 0)

	for _, v := range strings.Split(input[0], ",") {
		i, err := strconv.Atoi(v)
		if err != nil {
			return nil, err
		}
		result = append(result, i)
	}

	return result, nil
}

func growOneDay(input []int) []int {
	result := make([]int, len(input))

	for i, v := range input {
		if v > 0 {
			result[i] = v - 1
		} else {
			result[i] = 6
			result = append(result, 8)
		}
	}
	return result
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	di, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	for i := 0; i < 80; i++ {
		di = growOneDay(di)
	}

	return len(di), nil
}

// Ok so 256 is too long. Dynamic programming to the rescue?
func computeNumberOfFishFromSingleStart(days int) []int {
	// Invariant: state[i] is how many fish of age i
	// are descended from one of age 0
	state := []int{1, 0, 0, 0, 0, 0, 0, 0, 0}

	// N(age, t+1) = N(age+1, t) if age < 6
	// N(6, t+1) = N(7, t) + N(0, t)
	// N(7, t+1) = N(8, t)
	// N(8, t+1) = N(0, t)

	oneStep := func(before []int) []int {
		return []int{
			before[1],
			before[2],
			before[3],
			before[4],
			before[5],
			before[6],
			before[7] + before[0],
			before[8],
			before[0]}
	}

	for day := 0; day < days-9; day++ {
		state = oneStep(state)
	}

	// We want to track the final 9 states
	// so that we can work out the offsets
	result := make([]int, 9)
	for d := 0; d <= 8; d++ {
		state = oneStep(state)
		result[d] = 0
		for _, v := range state {
			result[d] += v
		}
	}

	return result
}

type Matrix9 [9][9]int
type Vector9 [9]int

func MatrixMult(a Matrix9, b Matrix9) Matrix9 {
	result := Matrix9{}

	for x := 0; x < 9; x++ {
		for y := 0; y < 9; y++ {
			// result[x][y] = b[x][:] * a[:][y]
			result[x][y] = 0
			for i := 0; i < 9; i++ {
				result[x][y] += b[x][i] * a[i][y]
			}
		}
	}

	return result
}

// yeah the vector rotates, but i don't care here
func MatrixVectorMult(a Matrix9, b Vector9) Vector9 {
	result := Vector9{}

	for x := 0; x < 9; x++ {
		result[x] = 0
		for y := 0; y < 9; y++ {
			result[x] += a[x][y] * b[y]
		}
	}

	return result
}

func Part2Matrix(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	di, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	recurrence := Matrix9{
		{0, 1, 0, 0, 0, 0, 0, 0, 0},
		{0, 0, 1, 0, 0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0, 0, 0, 0, 0},
		{0, 0, 0, 0, 1, 0, 0, 0, 0},
		{0, 0, 0, 0, 0, 1, 0, 0, 0},
		{0, 0, 0, 0, 0, 0, 1, 0, 0},
		{1, 0, 0, 0, 0, 0, 0, 1, 0},
		{0, 0, 0, 0, 0, 0, 0, 0, 1},
		{1, 0, 0, 0, 0, 0, 0, 0, 0},
	}
	mi := recurrence
	for i := 0; i < 8; i++ {
		mi = MatrixMult(mi, mi)
	}
	// mi = recurrence ^ 256

	vec := Vector9{0, 0, 0, 0, 0, 0, 0, 0, 0}
	for _, v := range di {
		vec[v]++
	}

	finalState := MatrixVectorMult(mi, vec)

	sum := 0
	for _, v := range finalState {
		sum += v
	}

	return sum, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	di, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	lookup := computeNumberOfFishFromSingleStart(256)
	sum := 0
	for _, v := range di {
		sum += lookup[8-v]
	}

	return sum, nil
}
