package day4

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type BingoBoard [5][5]int

type ParsedInput struct {
	sequence []int
	boards   []BingoBoard
}

type WinningState struct {
	board    BingoBoard
	sequence []int
}

type Set struct {
	items map[int]struct{}
}

func (s *Set) Add(e int) *Set {
	if s.items == nil {
		s.items = make(map[int]struct{})
	}
	s.items[e] = struct{}{}
	return s
}

func (s *Set) Has(e int) bool {
	_, ok := s.items[e]
	return ok
}

func parseBoard(input []string) (BingoBoard, error) {
	board := [5][5]int{}
	for i := 0; i < 5; i++ {
		j := 0
		for _, v := range strings.Split(input[i], " ") {
			if len(v) > 0 {
				vi, err := strconv.Atoi(v)
				if err != nil {
					return board, err
				}
				board[i][j] = vi
				j++
			}
		}
	}
	return board, nil
}

func parseInput(input []string) (ParsedInput, error) {
	calledNumberStrings := strings.Split(input[0], ",")
	calledNumbers := make([]int, 0)

	for _, v := range calledNumberStrings {
		num, err := strconv.Atoi(v)
		if err != nil {
			return ParsedInput{}, err
		}
		calledNumbers = append(calledNumbers, num)
	}

	input = input[2:]

	boards := make([]BingoBoard, 0)

	for len(input) > 5 {
		board, err := parseBoard(input[:5])
		if err != nil {
			return ParsedInput{}, err
		}

		boards = append(boards, board)
		input = input[5:]
		if len(input) > 1 && len(input[0]) == 0 {
			input = input[1:]
		}
	}

	return ParsedInput{
		sequence: calledNumbers,
		boards:   boards,
	}, nil
}

func hasBoardWon(s *Set, b BingoBoard) bool {
	// check columns
	for c := 0; c < 5; c++ {
		won := true
		for r := 0; r < 5; r++ {
			if !s.Has(b[r][c]) {
				won = false
				break
			}
		}
		if won {
			return true
		}
	}
	// check rows
	for r := 0; r < 5; r++ {
		won := true
		for c := 0; c < 5; c++ {
			if !s.Has(b[r][c]) {
				won = false
				break
			}
		}
		if won {
			return true
		}
	}
	return false
}

func findWinningBoard(input ParsedInput) (WinningState, error) {
	s := &Set{}
	for i, v := range input.sequence {
		s = s.Add(v)
		for _, b := range input.boards {
			if hasBoardWon(s, b) {
				return WinningState{
					board:    b,
					sequence: input.sequence[:i+1],
				}, nil
			}
		}
	}
	return WinningState{}, fmt.Errorf("did not find a winning board")
}

func unpickedSum(seq []int, b BingoBoard) int {
	s := &Set{}
	for _, v := range seq {
		s = s.Add(v)
	}

	sum := 0
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			if !s.Has(b[i][j]) {
				sum += b[i][j]
			}
		}
	}
	return sum
}

func findLosingBoard(input ParsedInput) (WinningState, error) {
	s := &Set{}
	activeBoards := input.boards
	for i, v := range input.sequence {
		s = s.Add(v)
		bi := 0
		if len(activeBoards) > 1 {
			// Whittle down to the loser
			for {
				if bi >= len(activeBoards) {
					break
				}
				if hasBoardWon(s, activeBoards[bi]) {
					activeBoards = append(activeBoards[:bi], activeBoards[bi+1:]...)
				} else {
					bi++
				}
			}
		} else {
			//find when it wins
			if hasBoardWon(s, activeBoards[0]) {
				return WinningState{
					board:    activeBoards[0],
					sequence: input.sequence[:i+1],
				}, nil
			}
		}
	}
	return WinningState{}, fmt.Errorf("did not find a winning board")
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	winState, err := findWinningBoard(pi)
	if err != nil {
		return 0, err
	}
	sum := unpickedSum(winState.sequence, winState.board)

	return sum * winState.sequence[len(winState.sequence)-1], nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	winState, err := findLosingBoard(pi)
	if err != nil {
		return 0, err
	}
	sum := unpickedSum(winState.sequence, winState.board)

	return sum * winState.sequence[len(winState.sequence)-1], nil
}
