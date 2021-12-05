package day5

import (
	"AoC2021/common"
	"io"
	"strconv"
	"strings"
)

type Coordinate struct {
	x int
	y int
}

type LineSegment struct {
	start Coordinate
	stop  Coordinate
}

func parseCoordinate(s string) (Coordinate, error) {
	coords := strings.Split(s, ",")

	result := Coordinate{}
	x, err := strconv.Atoi(coords[0])
	if err != nil {
		return result, err
	}
	result.x = x

	y, err := strconv.Atoi(coords[1])
	if err != nil {
		return result, err
	}
	result.y = y

	return result, nil
}

func parseInput(input []string) ([]LineSegment, error) {
	result := make([]LineSegment, 0)

	for _, v := range input {
		parts := strings.Split(v, " ")

		start, err := parseCoordinate(parts[0])
		if err != nil {
			return nil, err
		}
		stop, err := parseCoordinate(parts[2])
		if err != nil {
			return nil, err
		}
		result = append(result, LineSegment{start: start, stop: stop})
	}

	return result, nil
}

func countOverlaps(segments []LineSegment, includeDiagonal bool) int {
	hits := &common.Set{}
	overlaps := &common.Set{}

	handlePoint := func(p Coordinate) {
		//fmt.Printf(" %v", p)
		if hits.Has(p) {
			//fmt.Printf(" X")
			overlaps = overlaps.Add(p)
		}
		//fmt.Printf("\n")
		hits = hits.Add(p)
	}

	for _, v := range segments {
		var dy int
		var dx int

		var start Coordinate
		var stop Coordinate
		if v.start.x > v.stop.x {
			start = v.stop
			stop = v.start
			dx = 1
		} else {
			start = v.start
			stop = v.stop
			if v.start.x == v.stop.x {
				dx = 0
			} else {
				dx = 1
			}
		}

		if start.y > stop.y {
			dy = -1
		} else if start.y < stop.y {
			dy = 1
		} else {
			dy = 0
		}

		if !includeDiagonal && dx != 0 && dy != 0 {
			continue
		}

		//fmt.Printf("%v %v %d %d\n", start, stop, dx, dy)

		x := start.x
		y := start.y

		for x <= stop.x && (dy == 0 || y != stop.y+dy) {
			p := Coordinate{x: x, y: y}
			if x < 0 || y < 0 {
				panic(p)
			}
			handlePoint(p)
			x += dx
			y += dy
		}
	}
	return overlaps.Size()
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)

	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	overlaps := countOverlaps(pi, false)

	return overlaps, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)

	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	overlaps := countOverlaps(pi, true)

	return overlaps, nil
}
