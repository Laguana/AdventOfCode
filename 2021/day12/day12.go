package day12

import (
	"AoC2021/common"
	"io"
	"regexp"
	"strings"
)

// Lets just allocate a border that we can index to but otherwise completely ignore
type ParsedInput struct {
	graph map[string]*common.Set
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{graph: make(map[string]*common.Set)}

	for _, l := range s {
		parts := strings.Split(l, "-")
		if result.graph[parts[0]] == nil {
			result.graph[parts[0]] = &common.Set{}
		}
		if result.graph[parts[1]] == nil {
			result.graph[parts[1]] = &common.Set{}
		}
		result.graph[parts[0]].Add(parts[1])
		result.graph[parts[1]].Add(parts[0])
	}

	return result, nil
}

func isSmallCave(s string) bool {
	r := regexp.MustCompile(`[a-z]+`)
	return r.MatchString(s)
}

type Path struct {
	smallCavesLeft *common.Set
	dupedOne       bool
	visited        []string
	location       string
}

func countPaths(pi ParsedInput, noRevisit bool) int {
	fringe := make([]Path, 0)

	basePath := Path{smallCavesLeft: &common.Set{}, dupedOne: noRevisit, location: "start"}

	for k, _ := range pi.graph {
		if isSmallCave(k) {
			basePath.smallCavesLeft.Add(k)
		}
	}
	fringe = append(fringe, basePath)

	endPaths := make([]Path, 0)

	visit := func(p Path) {
		if p.location == "end" {
			//fmt.Printf("%v - end\n", p.visited)
			endPaths = append(endPaths, p)
			return
		}

		nextSmall := p.smallCavesLeft.Copy()
		nextDuped := p.dupedOne
		if isSmallCave(p.location) {
			if !p.smallCavesLeft.Has(p.location) {

				if p.dupedOne || p.location == "start" {
					return
				}
				nextDuped = true
			}
			nextSmall.Remove(p.location)
		}

		nextVisited := append(p.visited, p.location)
		//fmt.Println(nextVisited)

		//fmt.Printf("Extending: ")
		for _, nextI := range pi.graph[p.location].AsSlice() {
			next := nextI.(string)
			//fmt.Printf("%s;", next)
			newPath := Path{smallCavesLeft: nextSmall, dupedOne: nextDuped, location: next, visited: nextVisited}
			fringe = append(fringe, newPath)
		}
		//fmt.Printf("\n")
	}

	for len(fringe) > 0 {
		//fmt.Printf("-- %v\n", fringe)
		p := fringe[len(fringe)-1]
		fringe = fringe[:len(fringe)-1]
		visit(p)
	}

	//fmt.Println(endPaths)

	return len(endPaths)
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	result := countPaths(pi, true)

	return result, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	result := countPaths(pi, false)

	return result, nil
}
