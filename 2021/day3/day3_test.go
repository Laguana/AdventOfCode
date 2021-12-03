package day3

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split("00100,11110,10110,10111,10101,01111,00111,11100,10000,11001,00010,01010", ",")

func TestDay2Part1Sample(t *testing.T) {
	parsedInput, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}
	expectedCounts := [5]int{7, 5, 8, 7, 5}

	if parsedInput.nInputs != len(sampleInput) {
		t.Errorf("Expected %d inputs, got %d", len(sampleInput), parsedInput.nInputs)
	}

	for i := 0; i < 5; i++ {
		if parsedInput.counts[i] != expectedCounts[i] {
			t.Errorf("Mismatched counts; position %d should be %d but is %d", i, expectedCounts[i], parsedInput.counts[i])
		}
	}

	diagnostic := computeDiagnostic(parsedInput)

	if diagnostic.gamma != 22 {
		t.Errorf("expected gamma 22, got %d", diagnostic.gamma)
	}
	if diagnostic.epsilon != 9 {
		t.Errorf("expected epsilon 9, got %d", diagnostic.epsilon)
	}
}

func TestDay2Part1Input(t *testing.T) {
	d3i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d3i.Close()
	result, err := Part1(d3i)
	if err != nil {
		t.Error(err)
		return
	}
	if result != 749376 {
		t.Errorf("Expected product 749376, got %d", result)
	}
}

func TestDay2Part2Sample(t *testing.T) {
	lifeSupport, err := computeLifeSupport(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}
	if lifeSupport.o2 != 23 {
		t.Errorf("expected o2 23, got %d", lifeSupport.o2)
	}
	if lifeSupport.co2 != 10 {
		t.Errorf("expected co2 10, got %d", lifeSupport.co2)
	}
}

func TestDay2Part2Input(t *testing.T) {
	d3i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d3i.Close()
	result, err := Part2(d3i)
	if err != nil {
		t.Error(err)
		return
	}
	if result != 2372923 {
		t.Errorf("Expected product 2372923, got %d", result)
	}
}
