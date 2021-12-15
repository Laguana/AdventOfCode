package day15

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`, "\n")

func TestDay15Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	cost := astar(pi, pair{x: 0, y: 0}, pair{x: pi.width - 1, y: pi.height - 1})
	if cost != 40 {
		t.Errorf("Should have cost 40, cost %d", cost)

	}
}

func TestDay15Part1(t *testing.T) {
	d15i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d15i.Close()
	result, err := Part1(d15i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay15Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay15Part2(t *testing.T) {
	d15i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d15i.Close()
	result, err := Part2(d15i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
