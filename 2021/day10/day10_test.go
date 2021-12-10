package day10

import (
	"os"
	"sort"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`, "\n")

func TestDay10Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	score := 0

	for _, l := range pi {
		_, mismatch := getMismatchedEnding(l)
		score += scoreError(mismatch)
	}
	if score != 26397 {
		t.Errorf("Expected a score of 26397, got %d", score)
	}
}

func TestDay10Part1(t *testing.T) {
	d10i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d10i.Close()
	result, err := Part1(d10i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 390993
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay10Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	scores := make([]int, 0)

	for _, l := range pi {
		remaining, mismatch := getMismatchedEnding(l)
		if mismatch == 0 {
			scores = append(scores, scoreComplete(remaining))
		}
	}

	sort.Ints(scores)

	if scores[len(scores)/2] != 288957 {
		t.Errorf("Expected middle score to be 288957, got %v", scores)
	}
}

func TestDay10Part2(t *testing.T) {
	d10i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d10i.Close()
	result, err := Part2(d10i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 1558722
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
