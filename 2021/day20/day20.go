package day20

import (
	"AoC2021/common"
	"fmt"
	"io"
)

type pair struct {
	x, y int
}

type goLGrid map[pair]bool

func (g goLGrid) get(p pair) bool {
	b, ok := g[p]
	return ok && b
}

type ParsedInput struct {
	rule                   string
	iteration              int
	xmin, xmax, ymin, ymax int
	grid                   goLGrid
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}
	result.grid = make(goLGrid)
	result.rule = s[0]
	result.xmin = 0
	result.xmax = 0
	result.iteration = 0

	s = s[2:]
	for y, l := range s {
		if y > result.ymax {
			result.ymax = y
		}
		for x, c := range l {
			if c == '#' {
				result.grid[pair{x, y}] = true
			}
			if x > result.xmax {
				result.xmax = x
			}
		}

	}

	return result, nil
}

/*
Starting with a single 1x1 spot, showing 1 boundary condition

...
.#.
...

after one step it becomes where the unknown has expanded in 1 each direction

#####
#???#
#???#
#???#
#####

then one more step

.......
.?????.
.?????.
.?????.
.?????.
.?????.
.......

*/

func evolvePoint(pi ParsedInput, p pair) bool {
	lookup := 0
	for y := p.y - 1; y <= p.y+1; y++ {
		for x := p.x - 1; x <= p.x+1; x++ {
			lookup <<= 1
			if x < pi.xmin-1 || x > pi.xmax+1 || y < pi.ymin-1 || y > pi.ymax+1 {
				// if we are outside the observable universe, then we may need to infer the state
				if pi.rule[0] == '.' {
					// no change
					continue
				} else if pi.rule[511] == '#' && pi.iteration > 0 {
					// it saturates at 'on forever'
					lookup |= 1
					continue
				} else {
					// the unobserved universe toggles on and off every step
					if pi.iteration%2 == 1 {
						lookup |= 1
						continue
					}
				}
			}
			if pi.grid.get(pair{x, y}) {
				lookup |= 1
			}
		}
	}
	return pi.rule[lookup] == '#'
}

func evolveGrid(pi ParsedInput) ParsedInput {
	result := ParsedInput{}
	result.grid = make(goLGrid)
	result.rule = pi.rule
	result.xmin = pi.xmin - 2
	result.xmax = pi.xmax + 2
	result.ymin = pi.ymin - 2
	result.ymax = pi.ymax + 2
	result.iteration = pi.iteration + 1

	/*for p, _ := range pi.grid {
		for dx := -1; dx < 2; dx++ {
			for dy := -1; dy < 2; dy++ {
				pc := pair{p.x + dx, p.y + dy}
				if evolvePoint(pi, pc) {
					result.grid[pc] = true
				}
			}
		}
	}*/

	for x := result.xmin; x <= result.xmax; x++ {
		for y := result.ymin; y <= result.ymax; y++ {
			p := pair{x, y}
			for dx := -1; dx < 2; dx++ {
				for dy := -1; dy < 2; dy++ {
					pc := pair{p.x + dx, p.y + dy}
					if evolvePoint(pi, pc) {
						result.grid[pc] = true
					}
				}
			}
		}
	}

	return result
}

func printGrid(pi ParsedInput) {
	fmt.Printf("%d,%d -> %d,%d\n", pi.xmin, pi.ymin, pi.xmax, pi.ymax)
	for y := pi.ymin; y <= pi.ymax; y++ {
		for x := pi.xmin; x <= pi.xmax; x++ {
			if pi.grid.get(pair{x, y}) {
				fmt.Printf("#")
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Printf("\n")
	}
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	p1 := evolveGrid(pi)
	p2 := evolveGrid(p1)

	for p, v := range p2.grid {
		if !v {
			fmt.Printf("Had non-on entry? %v %v\n", p, v)
		}
	}

	return len(p2.grid), nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	for i := 0; i < 50; i++ {
		pi = evolveGrid(pi)
	}

	return len(pi.grid), nil
}
