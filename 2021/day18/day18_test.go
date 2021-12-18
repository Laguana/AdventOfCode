package day18

import (
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`[1,1]
[2,2]
[3,3]
[4,4]
[5,5]`, "\n")

func justParse(s string) snailfishNumber {
	n, _, _ := parseNumber(s)
	return n
}

func TestDay18Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	if len(pi.nums) != 5 {
		t.Errorf("Should have had 5 numbers, got %d", len(pi.nums))
	}

	flat := flatten(pi.nums[0])
	if flat[0].depth != 1 || flat[0].value != 1 {
		t.Errorf("Expected to have flattened into 1,1, got %d", flat[0])
	}

	f1 := flatten(justParse("[[[[[9,8],1],2],3],4]"))
	fe := flatten(justParse("[[[[0,9],2],3],4]"))
	r1 := reduce(f1)
	if !fsnEqual(r1, fe) {
		t.Errorf("Expected %v == %v", r1, fe)
	}

	f2 := flatten(justParse("[[[[0,7],4],[15,[0,2]]],[1,1]]"))
	fe = flatten(justParse("[[[[0,7],4],[[7,8],[0,2]]],[1,1]]"))
	r2 := reduce(f2)
	if !fsnEqual(r2, fe) {
		t.Errorf("Expected %v == %v", r2, fe)
	}

	a := add(flatten(justParse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")),
		flatten(justParse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")))
	ae := flatten(justParse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"))
	if !fsnEqual(a, ae) {
		t.Errorf("Expected %v == %v", a, ae)
	}

	m1 := magnitude(flatten(justParse("[[1,2],[[3,4],5]]")))
	if m1 != 143 {
		t.Errorf("Expected magnitude 143, got %d", m1)
	}

}

func TestDay18Part1(t *testing.T) {
	d18i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d18i.Close()
	result, err := Part1(d18i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 3486
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestDay18Part2Sample(t *testing.T) {
	_, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}
}

func TestDay18Part2(t *testing.T) {
	d18i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d18i.Close()
	result, err := Part2(d18i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 4747
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
