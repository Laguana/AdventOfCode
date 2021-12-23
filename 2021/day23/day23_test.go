package day23

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########`, "\n")

func TestDay23Part1Sample(t *testing.T) {

	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	/*
		start := roomState{hallway: [7]int{0, 0, 2, 3, 0, 0, 0}, rooms: [4][2]int{{2, 1}, {0, 4}, {0, 3}, {4, 1}}}
		for succ := range start.successors() {
			fmt.Printf("succ %v\n", succ)
		}
	*/

	cost := astar(pi.state, isSolved)
	if cost != 12521 {
		t.Errorf("Expected least cost to be 12521, got %d", cost)
	}

}

func TestDay23Part1(t *testing.T) {
	d23i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d23i.Close()
	result, err := Part1(d23i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 14460
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay23Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay23Part2(t *testing.T) {
	d23i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d23i.Close()
	result, err := Part2(d23i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
