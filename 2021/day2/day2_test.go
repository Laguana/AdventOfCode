package day2

import (
	"os"
	"strings"
	"testing"
)

var sampleInput []string = strings.Split("", ",")

func TestDay2Part1Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
		return
	}
}

func TestDay2Part1Input(t *testing.T) {
	d2i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d2i.Close()
	_, err = Part1(d2i)
	if err != nil {
		t.Error(err)
		return
	}
}

func TestDay2Part2Input(t *testing.T) {
	d2i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d2i.Close()
	_, err = Part1(d2i)
	if err != nil {
		t.Error(err)
		return
	}
}
