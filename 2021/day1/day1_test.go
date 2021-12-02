package day1

import (
	"os"
	"strings"
	"testing"
)

func TestDay1Part1Sample(t *testing.T) {

	var sampleInput = strings.Split("199,200,208,210,200,207,240,269,260,263", ",")
	intInput, err := ParseInputs(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}

	increments := CountIncrements(intInput)
	if increments != 7 {
		t.Errorf("Expected 7 increments but found %d", increments)
	}
}

func TestDay1Part1Input(t *testing.T) {
	d1i, err := os.Open("p1.input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d1i.Close()
	increments, err := Part1(d1i)
	if err != nil {
		t.Error(err)
		return
	}
	if increments != 1832 {
		t.Errorf("Expected 1832 increments but found %d", increments)
	}
}

func TestDay1Part2Sample(t *testing.T) {

	var sampleInput = strings.Split("199,200,208,210,200,207,240,269,260,263", ",")
	intInput, err := ParseInputs(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}

	increments := CountSlidingIncrements(intInput)
	if increments != 5 {
		t.Errorf("Expected 5 increments but found %d", increments)
	}
}

func TestDay1Part2Input(t *testing.T) {
	d1i, err := os.Open("p1.input.txt")
	if err != nil {
		t.Error(err)
	}
	defer d1i.Close()
	increments, err := Part2(d1i)
	if err != nil {
		t.Error(err)
	}
	if increments != 1858 {
		t.Errorf("Expected 1858 increments but found %d", increments)
	}
}
