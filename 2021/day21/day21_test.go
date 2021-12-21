package day21

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`Player 1 starting position: 4
Player 2 starting position: 8`, "\n")

func TestDay21Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
	dice := newDeterministicDice()
	_, s2, _ := play(pi, dice)
	if dice.rolled != 993 {
		t.Errorf("Expected 993 rolls, had %d", dice.rolled)
	}
	if s2 != 745 {
		t.Errorf("Expected loser to have score 745, got %d", s2)
	}
}

func TestDay21Part1(t *testing.T) {
	d21i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d21i.Close()
	result, err := Part1(d21i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 707784
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay21Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	p1w, _ := countUniverses(dirac{pi.start1, pi.start2, 21, 21})
	if p1w != 444356092776315 {
		t.Errorf("Expected 444356092776315, got %d", p1w)
	}
}

func TestDay21Part2(t *testing.T) {
	d21i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d21i.Close()
	result, err := Part2(d21i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 157595953724471
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
