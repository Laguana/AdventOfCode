package day15

import (
	"AoC2021/common"
	"io"
)

// Lets just allocate a border that we can index to but otherwise completely ignore
type ParsedInput struct {
	width, height int
	grid          [][]int
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	result.height = len(s)
	result.width = len(s[0])
	result.grid = make([][]int, result.height)

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

type searchNodeMap map[pair]*searchNode

func (snm searchNodeMap) get(p pair) *searchNode {
	v, ok := snm[p]
	if !ok {
		v = &searchNode{pos: p}
		snm[p] = v
	}
	return v
}

func astar(pi ParsedInput, start, stop pair, long bool) int {
	width := pi.width
	height := pi.height
	if long {
		width *= 5
		height *= 5
	}

	snm := searchNodeMap{}
	openList := make([]*searchNode, 0)

	startn := snm.get(start)
	startn.open = true
	startn.heuristic = manhattanDistance(start, stop)
	openList = append(openList, startn)

	//progress := 0

	for len(openList) > 0 {
		mini := 0
		minf := 99999999
		for i, v := range openList {
			if v.cost+v.heuristic < minf {
				mini = i
				minf = v.cost + v.heuristic
			}
		}
		sn := openList[mini]
		openList = append(openList[:mini], openList[mini+1:]...)

		if sn.closed {
			continue
		}

		sn.closed = true

		/*
			progress++
			if progress%1000 == 0 {
				fmt.Printf("%d: %d, %v\n", progress, len(openList), sn)
			}
			// */

		if sn.pos == stop {
			// Found the end
			/*
				t := sn
				for t != nil {
					fmt.Println(t)
					t = t.parent
				} // */
			return sn.cost
		}
		// generate successors
		addSucc := func(nPos pair) {
			if nPos.x < 0 || nPos.y < 0 || nPos.x >= width || nPos.y >= height {
				return
			}
			s := snm.get(nPos)
			if s.closed {
				return
			}
			cost := sn.cost + tileCost(pi, nPos)
			heuristic := manhattanDistance(nPos, stop)
			if s.open && s.cost+s.heuristic < cost+heuristic {
				// already have a better path
				return
			}
			s.parent = sn
			s.cost = cost
			s.heuristic = heuristic
			s.open = true

			openList = append(openList, s)
		}
		addSucc(pair{x: sn.pos.x + 1, y: sn.pos.y})
		addSucc(pair{x: sn.pos.x - 1, y: sn.pos.y})
		addSucc(pair{x: sn.pos.x, y: sn.pos.y + 1})
		addSucc(pair{x: sn.pos.x, y: sn.pos.y - 1})
	}

	return -1
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

	fmap := floodMap(pi, pair{x: pi.width - 1, y: pi.height - 1}, false)

	return fmap[0][0] - pi.grid[0][0], nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	cost := astar(pi, pair{x: 0, y: 0}, pair{x: pi.width*5 - 1, y: pi.height*5 - 1}, true)
	//fmap := floodMap(pi, pair{x: pi.width*5 - 1, y: pi.height*5 - 1}, true)

	//return fmap[0][0] - pi.grid[0][0], nil
	return cost, nil
}
