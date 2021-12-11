package day11

import (
	"AoC2021/common"
	"fmt"
	"io"
)

// Lets just allocate a border that we can index to but otherwise completely ignore
type ParsedInput struct {
	grid [12][12]int
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	for y, r := range s {
		for x, v := range r {
			result.grid[y+1][x+1] = int(v - '0')
		}
	}

	return result, nil
}

func printGrid(pi ParsedInput) {
	for y := 0; y < 10; y++ {
		fmt.Println(pi.grid[y+1][1:11])
	}
	fmt.Println()
}

type Pair struct {
	x, y int
}

func step(pi *ParsedInput) int {
	flashes := 0
	exploding := &common.Set{}
	for y := 0; y < 10; y++ {
		for x := 0; x < 10; x++ {
			pi.grid[y+1][x+1]++
			if pi.grid[y+1][x+1] > 9 {
				exploding.Add(Pair{x: x + 1, y: y + 1})
			}
		}
	}

	for exploding.Size() > 0 {
		p := exploding.Any().(Pair)
		//fmt.Printf("Exploding %v", p)
		exploding.Remove(p)

		if p.x == 0 || p.y == 0 || p.x == 11 || p.y == 11 {
			// Boundry condition
			continue
		}

		for dx := -1; dx < 2; dx++ {
			for dy := -1; dy < 2; dy++ {
				pi.grid[p.y+dy][p.x+dx]++
				if pi.grid[p.y+dy][p.x+dx] == 10 {
					//fmt.Printf(" triggering (%d,%d)", p.x+dx, p.y+dy)
					exploding.Add(Pair{x: p.x + dx, y: p.y + dy})
				}
			}
		}
		//fmt.Printf("\n")
	}

	for y := 0; y < 10; y++ {
		for x := 0; x < 10; x++ {
			if pi.grid[y+1][x+1] > 9 {
				pi.grid[y+1][x+1] = 0
				flashes++
			}
		}
	}

	//printGrid(*pi)

	return flashes
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	totalFlashes := 0

	for i := 0; i < 100; i++ {
		stepFlashes := step(&pi)
		totalFlashes += stepFlashes
	}

	return totalFlashes, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	i := 0
	for {
		i++
		stepFlashes := step(&pi)
		if stepFlashes == 100 {
			return i, nil
		}
	}
}
