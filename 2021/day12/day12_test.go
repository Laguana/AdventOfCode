package day12

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`start-A
start-b
A-c
A-b
b-d
A-end
b-end`, "\n")

func TestDay12Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	paths := countPaths(pi, true)

	if paths != 10 {
		t.Errorf("Expected 10 paths, got %d", paths)
	}
}

func TestDay12Part1(t *testing.T) {
	d12i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d12i.Close()
	result, err := Part1(d12i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 4754
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay12Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	paths := countPaths(pi, false)

	if paths != 36 {
		t.Errorf("Expected 36 paths, got %d", paths)
	}
}

func TestDay12Part2(t *testing.T) {
	d12i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d12i.Close()
	result, err := Part2(d12i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 143562
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
