package day21

import (
	"AoC2021/common"
	"io"
	"strconv"
	"strings"
)

type ParsedInput struct {
	start1, start2 int
}

type DeterministicDice struct {
	rolled, next int
}

type Dice interface {
	Roll() int
}

func (d *DeterministicDice) Roll() int {
	d.rolled++
	ret := d.next
	d.next++
	if d.next > 100 {
		d.next = 1
	}
	return ret
}

func newDeterministicDice() *DeterministicDice {
	ret := &DeterministicDice{0, 1}
	return ret
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	parts1 := strings.Split(s[0], ": ")
	start1, err := strconv.Atoi(parts1[1])
	if err != nil {
		return result, err
	}

	parts2 := strings.Split(s[1], ": ")
	start2, err := strconv.Atoi(parts2[1])
	if err != nil {
		return result, err
	}

	result.start1 = start1
	result.start2 = start2

	return result, nil
}

func play(pi ParsedInput, d Dice) (p1score, p2score int, rd Dice) {
	rd = d
	p1score = 0
	p2score = 0

	p1turn := true
	for p1score < 1000 && p2score < 1000 {
		roll := d.Roll() + d.Roll() + d.Roll()

		if p1turn {
			pi.start1 += roll
			if pi.start1 > 10 {
				pi.start1 %= 10
				if pi.start1 == 0 {
					pi.start1 = 10
				}
			}
			p1score += pi.start1
		} else {
			pi.start2 += roll
			if pi.start2 > 10 {
				pi.start2 %= 10
				if pi.start2 == 0 {
					pi.start2 = 10
				}
			}
			p2score += pi.start2
		}
		p1turn = !p1turn
		//fmt.Printf("rolled %d, now %d %d@%v\n", roll, p1score, p2score, pi)
	}
	return
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	d := newDeterministicDice()
	s1, s2, _ := play(pi, d)
	min := s1
	if s2 < min {
		min = s2
	}

	return min * d.rolled, nil
}

type dirac struct {
	p1pos, p2pos, p1left, p2left int
}
type pair struct {
	a, b int
}

var memoised map[dirac]pair = make(map[dirac]pair)

// how many ways to roll an X via 3d3
var threeRollPrevalence []int = []int{0, 0, 0,
	// 3 4 5 6 7 8 9
	1, 3, 6, 7, 6, 3, 1}

func countUniverses(d dirac) (int, int) {
	v, ok := memoised[d]
	if ok {
		//fmt.Printf("%v: !%v\n", d, v)
		return v.a, v.b
	}
	// if it is currently p1's turn, how's everything go?
	if d.p1left <= 0 {
		// without rolling anything, the game is over
		return 1, 0
	}
	if d.p2left <= 0 {
		return 0, 1
	}
	p1wins := 0
	p2wins := 0
	for r := 3; r <= 9; r++ {
		newpos := d.p1pos + r
		if newpos > 10 {
			newpos -= 10
		}
		p2w, p1w := countUniverses(dirac{d.p2pos, newpos, d.p2left, d.p1left - newpos})
		p1wins += p1w * threeRollPrevalence[r]
		p2wins += p2w * threeRollPrevalence[r]
	}

	memoised[d] = pair{p1wins, p2wins}

	//fmt.Printf("%v %d,%d\n", d, p1wins, p2wins)
	return p1wins, p2wins
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	max, p2w := countUniverses(dirac{pi.start1, pi.start2, 21, 21})
	if p2w > max {
		max = p2w
	}

	return max, nil
}
