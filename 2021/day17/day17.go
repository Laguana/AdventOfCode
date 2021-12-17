package day17

import (
	"AoC2021/common"
	"io"
	"strconv"
	"strings"
)

type ParsedInput struct {
	xmin, xmax int
	ymin, ymax int
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	//"target area: x=%d..%d, y=%d..%d

	ss := s[0]
	ss = ss[len("target area: "):]
	parts := strings.Split(ss, ", ")

	parsePair := func(s string) (int, int, error) {
		s = s[2:]
		ps := strings.Split(s, "..")
		v1, err := strconv.Atoi(ps[0])
		if err != nil {
			return 0, 0, err
		}
		v2, err := strconv.Atoi(ps[1])
		return v1, v2, err
	}

	var err error
	result.xmin, result.xmax, err = parsePair(parts[0])
	if err != nil {
		return result, err
	}
	result.ymin, result.ymax, err = parsePair(parts[1])
	return result, err
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	// its upward speed matches its downward speed when it returns to the same height.
	// We want to reach the bottom of the range, so ymin speed in that one step
	// How high does it go if we fire it at ymin upwards? ymin + ymin-1 + ... + 1 + 0
	// triangular number

	return pi.ymin * (pi.ymin + 1) / 2, nil
}

type pair struct {
	start, stop int
}

func hitsY(pi ParsedInput, vy int) pair {
	p := pair{-1, -1}
	step := 0
	y := 0
	found := false
	for y >= pi.ymin {
		if y >= pi.ymin && y <= pi.ymax {
			p.start = step
			found = true
			break
		}
		y += vy
		vy -= 1
		step++
	}
	if !found {
		return p
	}
	found = false
	for y >= pi.ymin {
		y += vy
		vy -= 1
		step++
	}
	p.stop = step - 1

	return p
}

func hitsX(pi ParsedInput, vx int) pair {
	p := pair{-1, -1}
	step := 0
	x := 0
	found := false
	for x <= pi.xmax {
		if x >= pi.xmin && x <= pi.xmax {
			p.start = step
			found = true
			break
		}
		if vx == 0 {
			return p
		}
		x += vx
		vx -= 1
		step++
	}
	if !found {
		return p
	}
	found = false
	for x <= pi.xmax {
		if vx == 0 {
			return p
		}
		x += vx
		vx -= 1
		step++
	}
	p.stop = step - 1

	return p
}

func countVelocities(pi ParsedInput) int {
	// How many ways are there to get to the target?
	// If we fire up, then we'll come down with the same speed but negative
	// - HOWEVER the vx value will be reduced (and it will be further over)
	// -- If vy > vx/2 then vx = 0 and x = vx * (vx+1)/2 (or -1, tbd)
	// -- if vy < vx/2 then vx = vx - (2*vy) and x is most of the triangular number over
	// eeh just simulate down maybe

	yvals := make(map[int]pair, 0)
	for vy := -pi.ymin; vy >= pi.ymin; vy-- {
		//fmt.Printf("y:%d\n", vy)
		interval := hitsY(pi, vy)
		if interval.start != -1 {
			yvals[vy] = interval
			// also if we fired up
			//i2 := pair{start: interval.start + 2*vy, stop: interval.start + 2*vy}
			//yvals[-vy] = i2
		}
	}
	//fmt.Println(yvals)

	count := 0
	for vx := 0; vx <= pi.xmax; vx++ {
		//fmt.Printf("x:%d\n", vx)
		interval := hitsX(pi, vx)
		if interval.start == -1 {
			continue
		}
		for _, yi := range yvals {
			if yi.stop < interval.start {
				continue
			}
			if interval.stop != -1 && interval.stop < yi.start {
				continue
			}
			// x gets in there before y stops, and after y starts; they overlap
			//fmt.Printf("%d, %d\n", vx, vy)
			count++
		}
	}
	return count
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	return countVelocities(pi), nil
}
