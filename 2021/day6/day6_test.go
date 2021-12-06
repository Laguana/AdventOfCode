package day6

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`3,4,3,1,2`, "\n")

func TestDay6Part1Sample(t *testing.T) {
	input, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	d1 := growOneDay(input)
	if len(d1) != 5 {
		t.Errorf("Should have no growth after 1 day: %d", len(d1))
	}
	d2 := growOneDay(d1)
	if len(d2) != 6 {
		t.Errorf("Should have grown after 2 days: %d", len(d2))
	}

	di := d2
	for i := 3; i <= 18; i++ {
		di = growOneDay(di)
	}
	if len(di) != 26 {
		t.Errorf("Expected 26 fish after 18 days: %d", len(di))
	}
}

func TestDay6Part1(t *testing.T) {
	d6i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d6i.Close()
	result, err := Part1(d6i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 390011
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}

func TestDay6Part2Sample(t *testing.T) {
	input, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
	lookup := computeNumberOfFishFromSingleStart()
	sum := 0
	for _, v := range input {
		sum += lookup[8-v]
	}
	if sum != 26984457539 {
		t.Errorf("Expected a sum of 26984457539, got %d", sum)
	}
}

func TestDay6Part2(t *testing.T) {
	d6i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d6i.Close()
	result, err := Part2(d6i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 1746710169834
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}
