package day16

import (
	"AoC2021/common"
	"fmt"
	"io"
)

type ParsedInput struct {
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	return result, fmt.Errorf("not implemented")
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	_, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	return 0, fmt.Errorf("not implemented")
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	_, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	return 0, fmt.Errorf("not implemented")
}
