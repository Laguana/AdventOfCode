package day17

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`target area: x=20..30, y=-10..-5`, "\n")

func TestDay17Part1Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay17Part1(t *testing.T) {
	d17i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d17i.Close()
	result, err := Part1(d17i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 3570
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay17Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	count := countVelocities(pi)
	if count != 112 {
		t.Errorf("Should have been 112 matching, got %d", count)
	}
}

func TestDay17Part2(t *testing.T) {
	d17i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d17i.Close()
	result, err := Part2(d17i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 1919
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
