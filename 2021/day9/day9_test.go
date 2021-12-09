package day9

import (
	"os"
	"sort"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`2199943210
3987894921
9856789892
8767896789
9899965678`, "\n")

func TestDay9Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if pi.height != 5 {
		t.Errorf("Expected height 5, got %d", pi.height)
	}
	if pi.width != 10 {
		t.Errorf("Expected width 10, got %d", pi.width)
	}

	//fmt.Println(pi)

	lowRisk := 0

	for y, l := range pi.grid {
		for x, _ := range l {
			lowRisk += getRisk(x, y, pi)
		}
	}

	if lowRisk != 15 {
		t.Errorf("Expected risk 15, got %d", lowRisk)
	}
}

func TestDay9Part1(t *testing.T) {
	d9i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d9i.Close()
	result, err := Part1(d9i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 504
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay9Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	regions := make([]int, 0)

	for y, l := range pi.grid {
		for x, _ := range l {
			if getRisk(x, y, pi) > 0 {
				regions = append(regions, floodFill(x, y, pi))
			}
		}
	}

	sort.Slice(regions, func(i, j int) bool {
		return regions[i] > regions[j]
	})

	if regions[0]*regions[1]*regions[2] != 1134 {
		t.Errorf("Expected 14 9 9 3, got %v", regions)
	}
}

func TestDay9Part2(t *testing.T) {
	d9i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d9i.Close()
	result, err := Part2(d9i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 1558722
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
