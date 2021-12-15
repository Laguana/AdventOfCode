package day15

import (
	"AoC2021/common"
	"container/heap"
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

func iabs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func manhattanDistance(a, b pair) int {
	return iabs(a.x-b.x) + iabs(a.y-b.y)
}

type PosItemMap map[pair]*searchNode

func (pim PosItemMap) get(p pair) *searchNode {
	n, ok := pim[p]
	if !ok {
		n = &searchNode{pos: p}
		pim[p] = n
	}
	return n
}

func astar(pi ParsedInput, start, stop pair, large bool) int {
	width := pi.width
	height := pi.height
	if large {
		width *= 5
		height *= 5
	}

	// Copied in large part from https://github.com/kkulak/golang-astar/blob/master/astar-impl.go

	posItemMap := PosItemMap{}
	priorityQueue := &PriorityQueue{}
	heap.Init(priorityQueue)

	startNode := posItemMap.get(start)
	startNode.open = true
	startNode.cost = 0
	startNode.heuristic = manhattanDistance(start, stop)
	startNode.Priority = startNode.heuristic

	priorityQueue.Push(startNode)

	//progress := 0

	for priorityQueue.Len() > 0 {
		sn := heap.Pop(priorityQueue).(*searchNode)

		/*progress++
		if progress%1000 == 0 {
			fmt.Printf("%d: %d, %v\n", progress, priorityQueue.Len(), sn)
		}*/

		if sn.pos == stop {
			// Found the end
			/*t := sn
			for t != nil {
				fmt.Println(t)
				t = t.parent
			}*/
			return sn.cost
		}
		// generate successors
		addSucc := func(nPos pair) {
			if nPos.x < 0 || nPos.y < 0 || nPos.x >= width || nPos.y >= height {
				return
			}
			s := posItemMap.get(nPos)
			if s.closed {
				return
			}
			sCost := sn.cost + tileCost(pi, nPos)
			if !s.open {
				s.cost = sCost
				s.open = true
				s.heuristic = manhattanDistance(nPos, stop)
				s.Priority = s.cost + s.heuristic
				s.parent = sn
				heap.Push(priorityQueue, s)
			} else if sCost < s.cost {
				s.cost = sCost
				s.open = true
				s.heuristic = manhattanDistance(nPos, stop)
				s.Priority = s.cost + s.heuristic
				s.parent = sn
				heap.Fix(priorityQueue, s.Index)
			}
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
