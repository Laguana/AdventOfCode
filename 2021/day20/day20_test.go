package day20

import (
	"fmt"
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###`, "\n")

func TestDay20Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	printGrid(pi)
	fmt.Printf("\n")
	p1 := evolveGrid(pi)
	printGrid(p1)
	fmt.Printf("\n")
	p2 := evolveGrid(p1)
	printGrid(p2)
	fmt.Printf("\n")
	if len(p2.grid) != 35 {
		t.Errorf("Expected 35 on tiles, got %d", len(p2.grid))
	}
}

func TestDay20Part1(t *testing.T) {
	d20i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d20i.Close()
	result, err := Part1(d20i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 5268
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay20Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay20Part2(t *testing.T) {
	d20i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d20i.Close()
	result, err := Part2(d20i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 16875
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
