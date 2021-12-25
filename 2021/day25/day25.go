package day25

import (
	"AoC2021/common"
	"fmt"
	"io"
)

const (
	cEmpty int = iota
	cDown
	cRight
)

type ParsedInput struct {
	grid          [][]int
	height, width int
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	result.height = len(s)
	result.width = len(s[0])
	result.grid = make([][]int, result.height)
	for y, l := range s {
		result.grid[y] = make([]int, result.width)
		for x, c := range l {
			if c == '.' {
				result.grid[y][x] = cEmpty
			} else if c == '>' {
				result.grid[y][x] = cRight
			} else if c == 'v' {
				result.grid[y][x] = cDown
			} else {
				return result, fmt.Errorf("invalid character '%c'", c)
			}
		}
	}

	return result, nil
}

type pair struct {
	x, y int
}

func evolveRight(pi ParsedInput) int {
	toMove := make(map[pair]struct{})
	for y := 0; y < len(pi.grid); y++ {
		for x := 0; x < len(pi.grid[y]); x++ {
			if pi.grid[y][x] == cRight && pi.grid[y][(x+1)%pi.width] == cEmpty {
				toMove[pair{x, y}] = struct{}{}
			}
		}
	}
	for p := range toMove {
		pi.grid[p.y][p.x] = cEmpty
		pi.grid[p.y][(p.x+1)%pi.width] = cRight
	}
	return len(toMove)
}
func evolveDown(pi ParsedInput) int {
	toMove := make(map[pair]struct{})
	for y := 0; y < len(pi.grid); y++ {
		for x := 0; x < len(pi.grid[y]); x++ {
			if pi.grid[y][x] == cDown && pi.grid[(y+1)%pi.height][x] == cEmpty {
				toMove[pair{x, y}] = struct{}{}
			}
		}
	}
	for p := range toMove {
		pi.grid[p.y][p.x] = cEmpty
		pi.grid[(p.y+1)%pi.height][p.x] = cDown
	}
	return len(toMove)
}

func evolve(pi ParsedInput) int {
	right := evolveRight(pi)
	down := evolveDown(pi)
	return right + down
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	i := 0
	for {
		i++
		moved := evolve(pi)
		if moved == 0 {
			break
		}
		if i%100 == 0 {
			fmt.Printf("Step %d moved %d\n", i, moved)
		}
	}

	return i, nil
}
