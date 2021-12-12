package day12

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	``, "\n")

func TestDay12Part1Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
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
	expected := 1732
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay12Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
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
	expected := 290
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
