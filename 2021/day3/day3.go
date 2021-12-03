package day3

import (
	"AoC2021/common"
	"fmt"
	"io"
	"sort"
)

type ParsedInput struct {
	counts  []int
	nInputs int
}

type DiagnosticReport struct {
	epsilon int64
	gamma   int64
}

type LifeSupportReport struct {
	o2  int64
	co2 int64
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
			if stringBytes[i] == '1' {
				counts[i]++
			} else if stringBytes[i] != '0' {
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

func parseBinary(s string) (int64, error) {
	var result int64 = 0
	for i := 0; i < len(s); i++ {
		result <<= 1
		switch s[i] {
		case '1':
			result += 1
		case '0':
			break
		default:
			return 0, fmt.Errorf("unexpected binary bit %c", s[i])
		}
	}
	return result, nil
}

func get1Index(s []string, b int) int {
	for i := 0; i < len(s); i++ {
		if s[i][b] == '1' {
			return i
		}
	}
	return -1
}

func computeSingleLifeSupport(input []string, useMost bool) (int64, error) {
	bitLen := len(input[0])

	for bit := 0; bit < bitLen; bit++ {
		if len(input) == 1 {
			return parseBinary(input[0])
		}

		//fmt.Printf("Bit %d, %d remaining\n", bit, len(input))

		mid := get1Index(input, bit)
		if mid == -1 {
			return 0, fmt.Errorf("Had input with no 1s, need to rethink")
		}
		half := len(input) / 2
		if useMost {
			if mid == half || mid < half {
				input = input[mid:]
			} else {
				input = input[:mid]
			}
		} else {
			if mid == half || mid < half {
				input = input[:mid]
			} else {
				input = input[mid:]
			}
		}
	}
	if len(input) == 1 {
		return parseBinary(input[0])
	}

	return 0, fmt.Errorf("somehow fell off the end!")
}

func computeLifeSupport(input []string) (LifeSupportReport, error) {
	// If we sort the strings, then we can just walk forward until bit i
	// matches the expected bit, and we know the result is in the next
	// contiguous chunk
	sort.Strings(input)

	o2, err := computeSingleLifeSupport(input, true)
	if err != nil {
		return LifeSupportReport{}, err
	}
	co2, err := computeSingleLifeSupport(input, false)
	if err != nil {
		return LifeSupportReport{}, err
	}

	return LifeSupportReport{
		o2:  o2,
		co2: co2,
	}, nil
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

func Part2(r io.Reader) (int64, error) {
	input := common.ReadLinesToSlice(r)
	lifeSupport, err := computeLifeSupport(input)
	if err != nil {
		return 0, err
	}
	return lifeSupport.o2 * lifeSupport.co2, nil
}
