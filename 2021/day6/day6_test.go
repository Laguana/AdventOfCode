package day6

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	``, "\n")

func TestDay6Part1Sample(t *testing.T) {

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
	expected := 3990
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}

func TestDay6Part2Sample(t *testing.T) {

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
	expected := 21305
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}
