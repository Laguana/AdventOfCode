package day8

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`, "\n")

func TestDay8Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	n1478 := 0

	for _, v := range pi {
		if len(v.result) != 4 {
			t.Errorf("Expected 4 segments, got %d", len(v.result))
		}
		if len(v.combinations) != 10 {
			t.Errorf("Expected 10 combinations, got %d", len(v.combinations))
		}

		for _, d := range v.result {
			if is1478(d) {
				n1478++
			}
		}
	}

	if n1478 != 26 {
		t.Errorf("Expected 26 1478s got %d", n1478)
	}

}

func TestDay8Part1(t *testing.T) {
	d8i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d8i.Close()
	result, err := Part1(d8i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 237
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay8Part2SampleSimple(t *testing.T) {
	pi, err := parseInput(strings.Split("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", "\n"))
	if err != nil {
		t.Error(err)
	}
	digits := decodeDigits(pi[0])
	if digits != 5353 {
		t.Errorf("Expected 5353, got %d", digits)
	}
}

func TestDay8Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	sum := 0

	for _, v := range pi {
		sum += decodeDigits(v)
	}

	if sum != 61229 {
		t.Errorf("Expected 61229, got %d", sum)
	}
}

func TestDay8Part2(t *testing.T) {
	d8i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d8i.Close()
	result, err := Part2(d8i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 1009098
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
