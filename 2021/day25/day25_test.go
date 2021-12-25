package day25

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>`, "\n")

func TestDay25Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	i := 0
	for {
		i++
		moved := evolve(pi)
		if moved == 0 {
			break
		}
		if i > 60 {
			break
		}
	}
	if i != 58 {
		t.Errorf("Should have reached a fixed point after 58 steps")
	}
}

func TestDay25Part1(t *testing.T) {
	d25i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d25i.Close()
	result, err := Part1(d25i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 321
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
