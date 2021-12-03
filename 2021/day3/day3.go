package day3

import (
	"AoC2021/common"
	"fmt"
	"io"
)

type ParsedInput struct {
	counts  []int
	nInputs int
}

type DiagnosticReport struct {
	epsilon int64
	gamma   int64
}

func parseInput(input []string) (ParsedInput, error) {
	var bitLength = len(input[0])
	var counts = make([]int, bitLength)

	for i := 0; i < bitLength; i++ {
		counts[i] = 0
	}

	for _, v := range input {
		if len(v) != bitLength {
			return ParsedInput{}, fmt.Errorf("inconsistent bit length: expected %d, found %d", bitLength, len(v))
		}
		stringBytes := []byte(v)
		for i := 0; i < bitLength; i++ {
			if stringBytes[i] == []byte("1")[0] {
				counts[i]++
			} else if stringBytes[i] != []byte("0")[0] {
				return ParsedInput{}, fmt.Errorf("non-binary input: got %c", stringBytes[i])
			}
		}
	}

	return ParsedInput{
		counts:  counts,
		nInputs: len(input),
	}, nil
}

func computeDiagnostic(pi ParsedInput) DiagnosticReport {
	var gamma int64 = 0
	bitLen := len(pi.counts)
	half := pi.nInputs / 2
	for i := 0; i < bitLen; i++ {
		gamma <<= 1
		if pi.counts[i] > half {
			gamma += 1
		}
	}
	return DiagnosticReport{
		gamma:   gamma,
		epsilon: gamma ^ ((1 << bitLen) - 1),
	}
}

func Part1(r io.Reader) (int64, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	state := computeDiagnostic(pi)

	return state.epsilon * state.gamma, nil
}
