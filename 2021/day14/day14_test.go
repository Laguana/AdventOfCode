package day14

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`, "\n")

func TestDay14Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if pi.start != "NNCB" {
		t.Errorf("Expected starting string to be NNCB, got %s", pi.start)
	}
	if len(pi.rules) != 16 {
		t.Errorf("Wanted 16 rules, got %d", len(pi.rules))
	}

	p2 := step(pi)
	if p2.start != "NCNBCHB" {
		t.Errorf("Expected NCNBCHB after 1 step, got %s", p2.start)
	}

	pn := p2
	for i := 1; i < 10; i++ {
		pn = step(pn)
	}
	if len(pn.start) != 3073 {
		t.Errorf("Expected len 3073 after 10 steps, got %d", len(pn.start))
	}

	if score(pn) != 1588 {
		t.Errorf("Expected score 1588, got %d", score(pn))
	}
}

func TestDay14Part1(t *testing.T) {
	d14i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d14i.Close()
	result, err := Part1(d14i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 2233
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay14Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	counts := repeatCount(pi, 10)
	score := scoreCounts(counts)

	if score != 1588 {
		t.Errorf("Expected 10 score to be 1588 got %d", score)
	}

	counts = repeatCount(pi, 40)
	score = scoreCounts(counts)
	if score != 2188189693529 {
		t.Errorf("Expected 40 score to be 2188189693529 got %d", score)
	}

}

func TestDay14Part2(t *testing.T) {
	d14i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d14i.Close()
	result, err := Part2(d14i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := uint(2884513602164)
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
