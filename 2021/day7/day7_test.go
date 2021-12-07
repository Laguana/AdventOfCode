package day7

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(`16,1,2,0,4,2,7,1,2,14`, "\n")

func TestDay7Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	mid := pi[len(pi)/2]

	diffs := make([]int, len(pi))
	for i, v := range pi {
		diffs[i] = v - mid
	}

	sum := absSumList(diffs)

	if sum != 37 {
		t.Errorf("Expected the cost to be 37, got %d", sum)
	}
}

func TestDay7Part1(t *testing.T) {
	d7i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d7i.Close()
	result, err := Part1(d7i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 349769
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay7Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
	mid := pi[len(pi)/2]

	bestSum := 1 << 62
	dir := -1
	pos := mid
	for dir < 2 {
		diffs := make([]int, len(pi))
		for i, v := range pi {
			diffs[i] = v - pos
		}

		sum := sumSquareNumberList(diffs)
		if sum < bestSum {
			bestSum = sum
		} else {
			dir = dir + 2
			pos = mid
		}
		pos += dir
	}

	if bestSum != 168 {
		t.Errorf("Expected 168, got %d", bestSum)
	}
}

func TestDay7Part2(t *testing.T) {
	d7i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d7i.Close()
	result, err := Part2(d7i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 99540554
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
