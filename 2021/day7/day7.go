package day7

import (
	"AoC2021/common"
	"io"
	"sort"
	"strconv"
	"strings"
)

func parseInput(s []string) ([]int, error) {
	result := make([]int, 0)

	for _, v := range strings.Split(s[0], ",") {
		i, err := strconv.Atoi(v)
		if err != nil {
			return result, err
		}
		result = append(result, i)
	}
	sort.Ints(result)
	return result, nil
}

func absSumList(i []int) int {
	result := 0
	for _, v := range i {
		av := 0
		if v < 0 {
			av = -v
		} else {
			av = v
		}
		result += av
	}
	return result
}

func sumSquareNumberList(i []int) int {
	result := 0
	for _, v := range i {
		av := 0
		if v < 0 {
			av = -v
		} else {
			av = v
		}
		result += (av * (av + 1)) / 2
	}
	return result
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	mid := pi[len(pi)/2]

	diffs := make([]int, len(pi))
	for i, v := range pi {
		diffs[i] = v - mid
	}

	sum := absSumList(diffs)
	return sum, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	mid := pi[len(pi)/2]

	bestSum := 1 << 62
	dir := -1
	pos := mid
	for dir < 2 {
		diffs := make([]int, len(pi))
		for i, v := range pi {
			diffs[i] = v - pos
		}

		sum := sumSquareNumberList(diffs)
		if sum < bestSum {
			bestSum = sum
		} else {
			dir = dir + 2
			pos = mid
		}
		pos += dir
	}

	return bestSum, nil
}
