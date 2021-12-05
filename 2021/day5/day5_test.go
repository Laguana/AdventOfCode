package day5

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`, "\n")

func TestDay5Part1Sample(t *testing.T) {
	segments, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if len(segments) != 10 {
		t.Errorf("expected 10 segments, got %d", len(segments))
	}

	overlaps := countOverlaps(segments, false)
	if overlaps != 5 {
		t.Errorf("expected 5 overlaps, got %d", overlaps)
	}
}

func TestDay5Part1(t *testing.T) {
	d5i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d5i.Close()
	result, err := Part1(d5i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 3990
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}

func TestDay5Part2Sample(t *testing.T) {
	segments, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if len(segments) != 10 {
		t.Errorf("expected 10 segments, got %d", len(segments))
	}

	overlaps := countOverlaps(segments, true)
	if overlaps != 12 {
		t.Errorf("expected 12 overlaps, got %d", overlaps)
	}
}

func TestDay5Part2(t *testing.T) {
	d5i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d5i.Close()
	result, err := Part2(d5i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 21305
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}
