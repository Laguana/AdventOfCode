package day16

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	``, "\n")

func TestDay16Part1Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay16Part1(t *testing.T) {
	d16i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d16i.Close()
	result, err := Part1(d16i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay16Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay16Part2(t *testing.T) {
	d16i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d16i.Close()
	result, err := Part2(d16i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
