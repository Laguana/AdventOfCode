package day15

import (
	"AoC2021/common"
	"fmt"
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

func astar(pi ParsedInput, start, stop pair) int {
	openList := make([]searchNode, 0)
	closedList := make(map[pair]searchNode)

	openList = append(openList, searchNode{
		pos:       start,
		cost:      0,
		heuristic: manhattanDistance(start, stop),
	})

	progress := 0

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

		progress++
		if progress%1000 == 0 {
			fmt.Printf("%d: %d, %v\n", progress, len(openList), sn)
		}

		if sn.pos == stop {
			// Found the end
			/*t := &sn
			for t != nil {
				fmt.Println(t)
				t = t.parent
			}*/
			return sn.cost
		}
		// generate successors
		addSucc := func(nPos pair) {
			if nPos.x < 0 || nPos.y < 0 || nPos.x >= pi.width || nPos.y >= pi.height {
				return
			}
			s := searchNode{
				pos:       nPos,
				cost:      sn.cost + pi.grid[nPos.y][nPos.x],
				heuristic: manhattanDistance(nPos, stop),
				parent:    &sn,
			}
			{
				ce, ok := closedList[s.pos]
				if ok && ce.cost+ce.heuristic <= s.cost+s.heuristic {
					//fmt.Printf("Already have better path for %v\n", nPos)
					return
				}
			}

			ce, ok := closedList[nPos]
			if ok && ce.cost+ce.heuristic < s.cost+s.heuristic {
				//fmt.Printf("Already have better path for %v\n", nPos)
				return
			}
			openList = append(openList, s)
		}
		addSucc(pair{x: sn.pos.x + 1, y: sn.pos.y})
		addSucc(pair{x: sn.pos.x - 1, y: sn.pos.y})
		addSucc(pair{x: sn.pos.x, y: sn.pos.y + 1})
		addSucc(pair{x: sn.pos.x, y: sn.pos.y - 1})

		old, ok := closedList[sn.pos]
		if !ok || old.cost+old.heuristic > sn.cost+sn.heuristic {
			closedList[sn.pos] = sn
		}
	}

	return -1
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	cost := astar(pi, pair{x: 0, y: 0}, pair{x: pi.width - 1, y: pi.height - 1})

	return cost, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	_, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	return 0, fmt.Errorf("not implemented")
}
