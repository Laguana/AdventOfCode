package day14

import (
	"AoC2021/common"
	"io"
	"strings"
)

type ParsedInput struct {
	rules map[string]string
	start string
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{rules: make(map[string]string)}

	result.start = s[0]

	s = s[2:]
	for _, r := range s {
		parts := strings.Split(r, " -> ")
		result.rules[parts[0]] = parts[1]
	}

	return result, nil
}

func step(pi ParsedInput) ParsedInput {
	result := ParsedInput{rules: pi.rules, start: ""}
	for i := 0; i < len(pi.start)-1; i++ {
		result.start += pi.start[i : i+1]
		s := pi.start[i : i+2]
		r, ok := pi.rules[s]
		if ok {
			result.start += r
		}
	}
	result.start += pi.start[len(pi.start)-1 : len(pi.start)]
	return result
}

func score(pi ParsedInput) int {
	counts := make(map[rune]int)
	for _, r := range pi.start {
		counts[r]++
	}
	min := 99999999
	max := -1

	for _, v := range counts {
		if v > max {
			max = v
		}
		if v < min {
			min = v
		}
	}

	return max - min
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	pn := pi
	for i := 0; i < 10; i++ {
		pn = step(pn)
	}

	return score(pn), nil
}

func repeatCount(pi ParsedInput, steps int) map[string]uint {
	// each application of XY->Z is almost entirely unrelated to anything else;
	// it just shares one X with an adjacent WX and one Y with an adjacent YU
	// so dynamic programming up how things work, then fiddle with the ends?

	// XY->Z ==> XZY ==> XZ -> A, ZY -> B
	// Don't 'insert chars', but instead 'replace tokens'

	counts := make(map[string]uint)
	counts["<"+pi.start[0:1]] = 1
	for i := 0; i < len(pi.start)-1; i++ {
		counts[pi.start[i:i+2]]++
	}
	counts[pi.start[len(pi.start)-1:]+">"] = 1

	for i := 0; i < steps; i++ {
		counts2 := make(map[string]uint)
		for k, v := range counts {
			counts2[k] = v
		}

		for from, to := range pi.rules {
			c := counts[from]
			counts2[from] -= c
			counts2[from[0:1]+to] += c
			counts2[to+from[1:2]] += c
		}

		counts = counts2
	}

	return counts
}

func scoreCounts(counts map[string]uint) uint {
	runecount := make(map[rune]uint)

	for k, v := range counts {
		kr := []rune(k)
		if kr[0] != '<' {
			runecount[kr[0]] += v
		}
	}

	var max uint = 0
	var min uint = ^uint(0)

	for _, c := range runecount {
		if c > max {
			max = c
			//fmt.Printf("Max ?= %d %s\n", max, string(r))
		}
		if c < min {
			min = c
			//fmt.Printf("Min ?= %d %s\n", max, string(r))
		}
	}
	return max - min
}

func Part2(r io.Reader) (uint, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	counts := repeatCount(pi, 40)
	score := scoreCounts(counts)

	return score, nil
}
