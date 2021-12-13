package dayXX

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	``, "\n")

func TestDayXXPart1Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDayXXPart1(t *testing.T) {
	dXXi, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer dXXi.Close()
	result, err := Part1(dXXi)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDayXXPart2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDayXXPart2(t *testing.T) {
	dXXi, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer dXXi.Close()
	result, err := Part2(dXXi)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
