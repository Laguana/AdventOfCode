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

	fmap := floodMap(pi, pair{x: pi.width - 1, y: pi.height - 1}, false)
	if fmap[0][0] != 41 {
		t.Errorf("Expected cost 40, got %d", fmap[0][0])
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
	expected := 415
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay15Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
	fmap := floodMap(pi, pair{x: pi.width*5 - 1, y: pi.height*5 - 1}, true)
	if fmap[0][0] != 316 {
		t.Errorf("Expected cost 316, got %d", fmap[0][0])
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
	expected := 2864
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
