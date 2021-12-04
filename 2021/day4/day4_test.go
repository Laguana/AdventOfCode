package day4

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
`, "\n")

func TestDay2Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if len(pi.sequence) != 27 {
		t.Errorf("Expected sequence of 25, got %d", len(pi.sequence))
	}
	if len(pi.boards) != 3 {
		t.Errorf("Expected 3 boards, got %d", len(pi.boards))
	}

	winningState, err := findWinningBoard(pi)
	if err != nil {
		t.Error(err)
	}

	if len(winningState.sequence) != 12 {
		t.Errorf("Expected a board to win in 12 moves, got %d", len(winningState.sequence))
	}
	if winningState.sequence[11] != 24 {
		t.Errorf("Expected last called number to be 24, got %d", winningState.sequence[11])
	}
	if winningState.board[0][0] != 14 {
		t.Errorf("Expected the winning board to be the third board, starting with 14")
	}

	sum := unpickedSum(winningState.sequence, winningState.board)
	if sum != 188 {
		t.Errorf("Expected unpicked sum to be 188, got %d", sum)
	}

}

func TestDay2Part1Input(t *testing.T) {
	d4i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d4i.Close()
	result, err := Part1(d4i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 82440
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}

func TestDay2Part2Sample(t *testing.T) {

}

func TestDay2Part2Input(t *testing.T) {
	d4i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d4i.Close()
	result, err := Part2(d4i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 749376
	if result != expected {
		t.Errorf("Expected product %d, got %d", expected, result)
	}
}
