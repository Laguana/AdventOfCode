package day18

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
)

type snailfishNumber interface {
	IsPlain() bool
	Value() int
	Left() snailfishNumber
	Right() snailfishNumber
}

type plainNumber struct {
	v int
}

func (i plainNumber) IsPlain() bool          { return true }
func (i plainNumber) Value() int             { return i.v }
func (i plainNumber) Left() snailfishNumber  { return nil }
func (i plainNumber) Right() snailfishNumber { return nil }

type pairNumber struct {
	left, right snailfishNumber
}

func (i pairNumber) IsPlain() bool          { return false }
func (i pairNumber) Value() int             { return -1 }
func (i pairNumber) Left() snailfishNumber  { return i.left }
func (i pairNumber) Right() snailfishNumber { return i.right }

func parsePair(s string) (snailfishNumber, string, error) {
	v1, s1, err := parseNumber(s[1:])
	if err != nil {
		return v1, s1, err
	}
	if s1[0] != ',' {
		return v1, s1, fmt.Errorf("pair did not have comma after first element")
	}
	v2, s2, err := parseNumber(s1[1:])
	if err != nil {
		return v1, s1, err
	}
	if s2[0] != ']' {
		return v1, s1, fmt.Errorf("pair did not end with ]")
	}
	return pairNumber{left: v1, right: v2}, s2[1:], nil
}

func parseInt(s string) (snailfishNumber, string, error) {
	var i int
	found := false
	for i = 0; i < len(s); i++ {
		if s[i] == ',' || s[i] == ']' {
			found = true
			break
		}
	}
	if !found {
		return plainNumber{}, s, fmt.Errorf("ran out of string when parsing number")
	}
	//fmt.Printf("parsing int from '%s', index %d substring '%s'\n", s, i, s[:i])
	val, err := strconv.Atoi(s[:i])
	return plainNumber{val}, s[i:], err

}

func parseNumber(s string) (snailfishNumber, string, error) {
	//fmt.Printf("Parsing number from %s\n", s)
	if s[0] == '[' {
		return parsePair(s)
	} else {
		return parseInt(s)
	}
}

type ParsedInput struct {
	nums []snailfishNumber
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	for _, l := range s {
		number, remainder, err := parseNumber(l)
		if err != nil {
			return result, err
		}
		if len(remainder) != 0 {
			return result, fmt.Errorf("did not fully parse number")
		}
		result.nums = append(result.nums, number)
	}

	return result, nil
}

type flatSnailfish struct {
	depth int
	value int
}
type flatSnailfishList []flatSnailfish

func fsnEqual(a, b flatSnailfishList) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if b[i] != v {
			return false
		}
	}
	return true
}

func flatten(sn snailfishNumber) flatSnailfishList {
	ret := make(flatSnailfishList, 0)

	var visit func(sn snailfishNumber, depth int)
	visit = func(sn snailfishNumber, depth int) {
		if sn.IsPlain() {
			ret = append(ret, flatSnailfish{depth, sn.Value()})
		} else {
			visit(sn.Left(), depth+1)
			visit(sn.Right(), depth+1)
		}
	}
	visit(sn, 0)

	return ret
}

func reduce(sn flatSnailfishList) flatSnailfishList {
	explode := func() bool {
		for i, v := range sn {
			if v.depth > 4 {
				if sn[i+1].depth != v.depth {
					panic("pair not a pair?")
				}
				if i > 0 {
					sn[i-1].value += v.value
				}
				if i+2 < len(sn) {
					sn[i+2].value += sn[i+1].value
				}
				//fmt.Printf("left '%v' omitting '%v' right '%v'\n", sn[:i], sn[i:i+2], sn[i+2:])
				t := append(sn[:i], flatSnailfish{v.depth - 1, 0})
				sn = append(t, sn[i+2:]...)
				return true
			}
		}
		return false
	}

	split := func() bool {
		for i, v := range sn {
			if v.value >= 10 {
				l := v.value / 2
				r := v.value - l

				ln := flatSnailfish{v.depth + 1, l}
				rn := flatSnailfish{v.depth + 1, r}
				//fmt.Printf("Splitting %v into [%v,%v]\n", v, ln, rn)

				rest := make(flatSnailfishList, len(sn[i+1:]))
				copy(rest, sn[i+1:])
				t := append(sn[:i], ln, rn)
				sn = append(t, rest...)
				//fmt.Println(sn)
				return true
			}
		}
		return false
	}

	for {
		//fmt.Println(sn)
		if explode() {
			//fmt.Println("Exploded")
			continue
		}
		if split() {
			//fmt.Println("Split")
			continue
		}
		return sn
	}
}

func add(a, b flatSnailfishList) flatSnailfishList {
	result := make(flatSnailfishList, len(a)+len(b))
	copy(result, a)
	copy(result[len(a):], b)
	for i, _ := range result {
		result[i].depth++
	}
	return reduce(result)
}

func magnitude(sn flatSnailfishList) int {
	// need to reconstruct the pairing behavior
	stack := make([]flatSnailfish, 0)
	// value is now the magnitude
	// Whenever we complete a node, we want to propagate it back up.
	// When is a node completed? whenever it is adjacent to one of equal depth?
	// e.g. [4,4, ...] can be converted to [3,...]
	// we can't have a case where [4,3,3,4] incorrectly becomes [4,2,4] because [4,3] is missing a node

	for i := 0; i < len(sn); i++ {
		v := sn[i]
		stack = append(stack, v)

		for j := len(stack) - 1; j > 0 && stack[j].depth == stack[j-1].depth; j-- {
			newDepth := stack[j].depth - 1
			l := stack[j-1].value
			r := stack[j].value
			stack = append(stack[:j-1], flatSnailfish{depth: newDepth, value: 3*l + 2*r})
		}
	}
	if len(stack) != 1 {
		panic("Couldn't sort out magnitude?")
	}
	return stack[0].value
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	var snSum flatSnailfishList = nil
	for _, v := range pi.nums {
		if snSum == nil {
			snSum = flatten(v)
			continue
		}
		snSum = add(snSum, flatten(v))
	}

	return magnitude(snSum), nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	flats := make([]flatSnailfishList, 0)
	for _, v := range pi.nums {
		flats = append(flats, flatten(v))
	}

	maxMagnitude := 0

	for i, v := range flats {
		for j, v2 := range flats {
			if i != j {
				mag := magnitude(add(v, v2))
				if mag > maxMagnitude {
					maxMagnitude = mag
				}
			}
		}
	}

	return maxMagnitude, nil
}
