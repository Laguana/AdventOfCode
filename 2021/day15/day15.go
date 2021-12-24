package day15

import (
	"AoC2021/common"
	"io"
)

type ParsedInput struct {
	width, height int
	grid          [][]int
	larger        bool
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	result.height = len(s)
	result.width = len(s[0])
	result.grid = make([][]int, result.height)
	result.larger = false

	for y, v := range s {
		result.grid[y] = make([]int, result.width)
		for x, r := range v {
			result.grid[y][x] = int(r - '0')
		}
	}

	return result, nil
}

type pair struct {
	x, y int
	grid *ParsedInput
}

type searchNode struct {
	pos       pair
	cost      int
	heuristic int
	open      bool
	closed    bool
	parent    *searchNode
}

func iabs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func manhattanDistance(a, b pair) int {
	return iabs(a.x-b.x) + iabs(a.y-b.y)
}

func isDest(dest pair) func(common.SearchState) bool {
	return func(a common.SearchState) bool {
		return a.(pair) == dest
	}
}

func manhattanToDest(dest pair) func(common.SearchState) int {
	return func(a common.SearchState) int {
		return manhattanDistance(a.(pair), dest)
	}
}

func (p pair) Valid() bool {
	width := p.grid.width
	height := p.grid.height
	if p.grid.larger {
		width *= 5
		height *= 5
	}
	return p.x >= 0 && p.y >= 0 && p.x < width && p.y < height
}

func (p pair) Successors() <-chan common.SearchStateSuccessor {
	ch := make(chan common.SearchStateSuccessor)
	go func() {
		maybeSucc := func(q pair) *common.SearchStateSuccessor {
			if !q.Valid() {
				return nil
			}
			return &common.SearchStateSuccessor{q, tileCost(*q.grid, q)}
		}
		if candidate := maybeSucc(pair{x: p.x + 1, y: p.y, grid: p.grid}); candidate != nil {
			ch <- *candidate
		}
		if candidate := maybeSucc(pair{x: p.x - 1, y: p.y, grid: p.grid}); candidate != nil {
			ch <- *candidate
		}
		if candidate := maybeSucc(pair{x: p.x, y: p.y + 1, grid: p.grid}); candidate != nil {
			ch <- *candidate
		}
		if candidate := maybeSucc(pair{x: p.x, y: p.y - 1, grid: p.grid}); candidate != nil {
			ch <- *candidate
		}
		close(ch)
	}()
	return ch
}

func tileCost(pi ParsedInput, p pair) int {
	tx := p.x / pi.width
	ty := p.y / pi.height
	qx := p.x % pi.width
	qy := p.y % pi.height

	return 1 + ((pi.grid[qy][qx]-1)%9+tx+ty)%9
}

func floodMap(pi ParsedInput, stop pair, larger bool) [][]int {
	queue := &common.Set{}
	done := &common.Set{}

	height := pi.height
	width := pi.width
	if larger {
		height *= 5
		width *= 5
	}

	result := make([][]int, height)
	for y := 0; y < height; y++ {
		result[y] = make([]int, width)
		for x := 0; x < width; x++ {
			result[y][x] = 99999999
		}
	}

	result[stop.y][stop.x] = tileCost(pi, stop)

	queue.Add(stop)

	for queue.Size() > 0 {
		minv := 9999999
		mine := pair{}
		for _, ei := range queue.AsSlice() {
			e := ei.(pair)
			if result[e.y][e.x] < minv {
				minv = result[e.y][e.x]
				mine = e
			}
		}
		queue.Remove(mine)
		done.Add(mine)

		succ := func(nPos pair) {
			if nPos.x < 0 || nPos.y < 0 || nPos.x >= width || nPos.y >= height {
				return
			}
			if done.Has(nPos) {
				return
			}
			cost := minv + tileCost(pi, nPos)
			if result[nPos.y][nPos.x] > cost {
				result[nPos.y][nPos.x] = cost
				queue.Add(nPos)
			}
		}
		succ(pair{x: mine.x + 1, y: mine.y})
		succ(pair{x: mine.x - 1, y: mine.y})
		succ(pair{x: mine.x, y: mine.y + 1})
		succ(pair{x: mine.x, y: mine.y - 1})
	}
	return result
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	fmap := floodMap(pi, pair{x: pi.width - 1, y: pi.height - 1, grid: &pi}, false)

	return fmap[0][0] - pi.grid[0][0], nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	pi.larger = true
	goal := pair{x: pi.width*5 - 1, y: pi.height*5 - 1, grid: &pi}
	result := common.Astar(pair{x: 0, y: 0, grid: &pi}, isDest(goal), manhattanToDest(goal))
	//fmap := floodMap(pi, pair{x: pi.width*5 - 1, y: pi.height*5 - 1}, true)

	//return fmap[0][0] - pi.grid[0][0], nil
	return result.Cost, nil
}
