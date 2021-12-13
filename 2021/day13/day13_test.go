package day13

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`, "\n")

func TestDay13Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if pi.grid.width != 11 {
		t.Errorf("Expected 11 width, got %d", pi.grid.width)
	}
	if pi.grid.height != 15 {
		t.Errorf("Expected 15 height, got %d", pi.grid.height)
	}

	//PrintGrid(pi.grid)

	g, err := fold(pi.grid, pi.grid.folds[0])
	if err != nil {
		t.Error(err)
	}

	//PrintGrid(g)

	if g.height != 7 {
		t.Errorf("Expected 7 height, got %d", g.height)
	}
	if countPresent(g) != 17 {
		t.Errorf("Should be 17 present, got %d", countPresent(g))
	}
}

func TestDay13Part1(t *testing.T) {
	d13i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d13i.Close()
	result, err := Part1(d13i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 737
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay13Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay13Part2(t *testing.T) {
	d13i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d13i.Close()
	_, err = Part2(d13i)
	if err != nil {
		t.Error(err)
		return
	}

	// Validation...?
}
