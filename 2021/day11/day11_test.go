package day11

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`, "\n")

func TestDay11Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	totalFlashes := 0

	for i := 0; i < 100; i++ {
		stepFlashes := step(&pi)
		totalFlashes += stepFlashes
		if i == 9 && totalFlashes != 204 {
			t.Errorf("Expected 204 flashes after 10 steps, got %d", totalFlashes)
		}
	}

	if totalFlashes != 1656 {
		t.Errorf("Expected 1656 flashes after 100 steps, got %d", totalFlashes)
	}

}

func TestDay11Part1(t *testing.T) {
	d11i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d11i.Close()
	result, err := Part1(d11i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 1732
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay11Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	i := 0
	for {
		i++
		stepFlashes := step(&pi)
		if stepFlashes == 100 {
			break
		}
	}
	if i != 195 {
		t.Errorf("Should be simultaneous on step 195, got %d", i)
	}
}

func TestDay11Part2(t *testing.T) {
	d11i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d11i.Close()
	result, err := Part2(d11i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 290
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
