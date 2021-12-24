package day24

import (
	"AoC2021/common"
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`inp z
inp x
mul z 3
eql z x`, "\n")

func TestDay24Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	alu := &alu{0, 0, 0, 0}

	inputs := make(chan int)
	go func() {
		inputs <- 1
		inputs <- 3
		close(inputs)
	}()
	alu.process(pi.instructions, inputs)
	if alu.z != 1 {
		t.Errorf("Expected program to report true. %v", *alu)
	}

	zeroSym := &symbolicVal{literal: true, val: 0}
	symAlu := &symbolicAlu{zeroSym, zeroSym, zeroSym, zeroSym}
	symAlu.process(pi.instructions)
	symAlu.z.simplify()
	z := symAlu.z.evaluate([]int{1, 3})

	if z != 1 {
		t.Errorf("Expected z to evaluate to 1, got %d", z)
	}

	inputSet := &common.Set{}
	symAlu.z.getInputs(inputSet)
	if inputSet.Size() != 2 || !inputSet.Has(0) || !inputSet.Has(1) {
		t.Errorf("Expected to see 0 and 1 inputs, got %v", inputSet)
	}

}

func TestDay24Part1(t *testing.T) {
	d24i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d24i.Close()
	result, err := Part1(d24i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 39494195799979
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay24Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay24Part2(t *testing.T) {
	d24i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d24i.Close()
	result, err := Part2(d24i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 13161151139617
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
