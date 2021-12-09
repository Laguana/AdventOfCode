package day9

import (
	"AoC2021/common"
	"io"
	"sort"
)

type ParsedInput struct {
	grid   [][]int
	width  int
	height int
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{
		height: len(s),
		width:  len(s[0]),
		grid:   make([][]int, 0),
	}

	for _, v := range s {
		line := make([]int, 0)
		for _, r := range v {
			line = append(line, int(r-'0'))
		}
		result.grid = append(result.grid, line)
	}

	return result, nil
}

func getRisk(x, y int, pi ParsedInput) int {
	cell := pi.grid[y][x]
	if x > 0 {
		if cell >= pi.grid[y][x-1] {
			return 0
		}
	}
	if x < pi.width-1 {
		if cell >= pi.grid[y][x+1] {
			return 0
		}
	}
	if y > 0 {
		if cell >= pi.grid[y-1][x] {
			return 0
		}
	}
	if y < pi.height-1 {
		if cell >= pi.grid[y+1][x] {
			return 0
		}
	}

	//fmt.Printf("low risk: (%d, %d) %d\n", x, y, cell)

	return cell + 1
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	lowRisk := 0

	for y, l := range pi.grid {
		for x, _ := range l {
			lowRisk += getRisk(x, y, pi)
		}
	}

	return lowRisk, nil
}

type Pair struct {
	x, y int
}

func floodFill(x, y int, pi ParsedInput) int {
	visited := &common.Set{}
	excluded := &common.Set{}
	queue := &common.Set{}
	queue.Add(Pair{x, y})

	for queue.Size() != 0 {
		p := queue.Any().(Pair)
		queue.Remove(p)
		//fmt.Printf("%v: ", p)
		v := pi.grid[p.y][p.x]
		if v < 9 {
			visited.Add(p)
			//fmt.Printf("+\n")
		} else {
			excluded.Add(p)
			//fmt.Printf("-\n")
			continue
		}
		if p.x > 0 {
			p2 := Pair{x: p.x - 1, y: p.y}
			if !visited.Has(p2) && !excluded.Has(p2) {
				queue.Add(p2)
			}
		}
		if p.x < pi.width-1 {
			p2 := Pair{x: p.x + 1, y: p.y}
			if !visited.Has(p2) && !excluded.Has(p2) {
				queue.Add(p2)
			}
		}
		if p.y > 0 {
			p2 := Pair{x: p.x, y: p.y - 1}
			if !visited.Has(p2) && !excluded.Has(p2) {
				queue.Add(p2)
			}
		}
		if p.y < pi.height-1 {
			p2 := Pair{x: p.x, y: p.y + 1}
			if !visited.Has(p2) && !excluded.Has(p2) {
				queue.Add(p2)
			}
		}
	}

	return visited.Size()
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	regions := make([]int, 0)

	for y, l := range pi.grid {
		for x, _ := range l {
			if getRisk(x, y, pi) > 0 {
				regions = append(regions, floodFill(x, y, pi))
			}
		}
	}

	sort.Slice(regions, func(i, j int) bool {
		return regions[i] > regions[j]
	})

	return regions[0] * regions[1] * regions[2], nil
}
