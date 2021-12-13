package day13

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
	"strings"
)

// Lets just allocate a border that we can index to but otherwise completely ignore
type Grid struct {
	width, height int
	grid          [][]bool
	folds         []int // positive for y axis, negative for x axis
}

type ParsedInput struct {
	grid Grid
}

type Point struct {
	x, y int
}

func PrintGrid(g Grid) {
	for y, r := range g.grid {
		if y >= g.height {
			break
		}
		fmt.Printf("[")
		for x, v := range r {
			if x >= g.width {
				break
			}
			if v {
				fmt.Printf("X")
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Printf("]\n")
	}
	fmt.Printf("--\n")
}

func parseInput(s []string) (ParsedInput, error) {
	grid := Grid{width: 0, height: 0}
	result := ParsedInput{grid: grid}

	dots := make([]Point, 0)

	foldStart := -1
	for i, v := range s {
		if len(v) == 0 {
			foldStart = i + 1
			break
		}
		parts := strings.Split(v, ",")
		x, err := strconv.Atoi(parts[0])
		if err != nil {
			return result, err
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			return result, err
		}
		dots = append(dots, Point{x: x, y: y})

		// x/y are 0 indexed, width/height is 1 more than the largest
		if x >= grid.width {
			grid.width = x + 1
		}
		if y >= grid.height {
			grid.height = y + 1
		}
	}

	grid.grid = make([][]bool, grid.height)
	for y := 0; y < grid.height; y++ {
		grid.grid[y] = make([]bool, grid.width)
	}

	for _, v := range dots {
		grid.grid[v.y][v.x] = true
	}

	for i := foldStart; i < len(s); i++ {
		parts := strings.Split(s[i], "=")
		fold, err := strconv.Atoi(parts[1])
		if err != nil {
			return result, err
		}
		if parts[0][len(parts[0])-1] == 'x' {
			fold *= -1
		}
		grid.folds = append(grid.folds, fold)
	}

	result.grid = grid

	return result, nil
}

func fold(g Grid, f int) (Grid, error) {

	if f < 0 {
		// fold along x
		// from -f through width
		// there are
		tofold := g.width + f - 1
		for y := 0; y < g.height; y++ {
			for dx := 1; dx <= tofold; dx++ {
				g.grid[y][-f-dx] = g.grid[y][-f-dx] || g.grid[y][-f+dx]
			}
		}
		g.width = -f
	} else {
		// fold along y
		tofold := g.height - f - 1
		for dy := 1; dy <= tofold; dy++ {
			//fmt.Printf("Mapping (%d,x) to (%d, x)\n", f+dy, f-dy)
			for x := 0; x < g.width; x++ {
				g.grid[f-dy][x] = g.grid[f-dy][x] || g.grid[f+dy][x]
			}
		}
		g.height = f
	}

	return g, nil
}

func countPresent(g Grid) int {
	count := 0
	for y := 0; y < g.height; y++ {
		for x := 0; x < g.width; x++ {
			if g.grid[y][x] {
				count++
			}
		}
	}
	return count
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	g, err := fold(pi.grid, pi.grid.folds[0])
	if err != nil {
		return 0, err
	}

	return countPresent(g), nil
}

func Part2(r io.Reader) (Grid, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return pi.grid, err
	}

	g := pi.grid
	for _, f := range pi.grid.folds {
		g, err = fold(g, f)
		if err != nil {
			return g, err
		}
	}

	return g, nil
}
