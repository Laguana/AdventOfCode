package day10

import (
	"AoC2021/common"
	"io"
	"sort"
)

type ParsedInput []string

func parseInput(s []string) (ParsedInput, error) {
	result := s

	return result, nil
}

func getMismatchedEnding(s string) ([]rune, rune) {
	stack := make([]rune, 0)
	for _, v := range s {
		switch v {
		case '(':
			fallthrough
		case '[':
			fallthrough
		case '{':
			fallthrough
		case '<':
			stack = append(stack, v)
		case ')':
			if stack[len(stack)-1] != '(' {
				return nil, v
			} else {
				stack = stack[:len(stack)-1]
			}
		case ']':
			if stack[len(stack)-1] != '[' {
				return nil, v
			} else {
				stack = stack[:len(stack)-1]
			}
		case '}':
			if stack[len(stack)-1] != '{' {
				return nil, v
			} else {
				stack = stack[:len(stack)-1]
			}
		case '>':
			if stack[len(stack)-1] != '<' {
				return nil, v
			} else {
				stack = stack[:len(stack)-1]
			}
		default:
			panic("Invalid character")
		}
	}

	return stack, 0
}

func scoreError(r rune) int {
	switch r {
	case ')':
		return 3
	case ']':
		return 57
	case '}':
		return 1197
	case '>':
		return 25137
	default:
		return 0
	}
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	score := 0

	for _, l := range pi {
		_, mismatch := getMismatchedEnding(l)
		score += scoreError(mismatch)
	}

	return score, nil
}

func scoreComplete(rem []rune) int {
	score := 0

	for i := len(rem) - 1; i >= 0; i-- {
		r := rem[i]
		score *= 5
		switch r {
		case '(':
			score += 1
		case '[':
			score += 2
		case '{':
			score += 3
		case '<':
			score += 4
		default:

			panic("Invalid character")
		}
	}

	return score
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	scores := make([]int, 0)

	for _, l := range pi {
		remaining, mismatch := getMismatchedEnding(l)
		if mismatch == 0 {
			scores = append(scores, scoreComplete(remaining))
		}
	}

	sort.Ints(scores)

	return scores[len(scores)/2], nil
}
