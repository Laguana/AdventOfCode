package day2

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type Direction int8

const (
	Forward Direction = iota
	Down
	Up
	Invalid
)

type Instruction struct {
	dir  Direction
	dist int
}

type SubState struct {
	depth int
	dist  int
	aim   int
}

func parseDirection(s string) (Direction, error) {
	switch s {
	case "forward":
		return Forward, nil
	case "down":
		return Down, nil
	case "up":
		return Up, nil
	default:
		return Invalid, fmt.Errorf("invalid direction '%s'", s)
	}
}

func parseInput(input []string) ([]Instruction, error) {
	instructions := make([]Instruction, len(input))

	for i, v := range input {

		components := strings.Split(v, " ")
		if len(components) != 2 {
			return nil, fmt.Errorf("expected instruction of length 2, got %d", len(components))
		}

		dir, err := parseDirection(components[0])
		if err != nil {
			return nil, err
		}

		dist, err := strconv.Atoi(components[1])
		if err != nil {
			return nil, err
		}

		instructions[i] = Instruction{dir: dir, dist: dist}
	}

	return instructions, nil
}

func computeEffect(instructions []Instruction) SubState {
	state := SubState{depth: 0, dist: 0}
	for _, v := range instructions {
		switch v.dir {
		case Forward:
			state.dist += v.dist
		case Up:
			state.depth -= v.dist
		case Down:
			state.depth += v.dist
		}
	}
	return state
}

func computeEffect2(instructions []Instruction) SubState {
	state := SubState{depth: 0, dist: 0}
	for _, v := range instructions {
		switch v.dir {
		case Forward:
			state.dist += v.dist
			state.depth += state.aim * v.dist
		case Up:
			state.aim -= v.dist
		case Down:
			state.aim += v.dist
		}
	}
	return state
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	instructions, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	state := computeEffect(instructions)

	return state.depth * state.dist, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	instructions, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	state := computeEffect2(instructions)

	return state.depth * state.dist, nil
}
