package day2

import (
	"os"
	"strings"
	"testing"
)

var sampleInput []string = strings.Split("forward 5,down 5,forward 8,up 3,down 8,forward 2", ",")

func TestDay2Part1Sample(t *testing.T) {
	instructions, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}
	state := computeEffect(instructions)
	if state.depth != 10 {
		t.Errorf("Expected depth 10, got depth %d", state.depth)
	}
	if state.dist != 15 {
		t.Errorf("Expected dist 15, got depth %d", state.dist)
	}
}

func TestDay2Part1Input(t *testing.T) {
	d2i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d2i.Close()
	result, err := Part1(d2i)
	if err != nil {
		t.Error(err)
		return
	}
	if result != 1636725 {
		t.Errorf("Expected product 1636725, got %d", result)
	}
}

func TestDay2Part2Sample(t *testing.T) {
	instructions, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}
	state := computeEffect2(instructions)
	if state.depth != 60 {
		t.Errorf("Expected depth 60, got depth %d", state.depth)
	}
	if state.dist != 15 {
		t.Errorf("Expected dist 15, got depth %d", state.dist)
	}
	if state.aim != 10 {
		t.Errorf("Expected aim 10, got aim %d", state.aim)
	}
}

func TestDay2Part2Input(t *testing.T) {
	d2i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d2i.Close()
	result, err := Part2(d2i)
	if err != nil {
		t.Error(err)
		return
	}
	if result != 1872757425 {
		t.Errorf("Expected result 1872757425 but got %d", result)
	}
}
