package day12

import (
	"AoC2021/common"
	"io"
	"regexp"
	"strings"
)

// Lets just allocate a border that we can index to but otherwise completely ignore
type ParsedInput struct {
	idLookup   map[string]int
	nameLookup map[int]string
	graph      map[int]*common.Set
}

func isSmallCave(s string) bool {
	r := regexp.MustCompile(`[a-z]+`)
	return r.MatchString(s)
}

func isSmallCaveId(c int) bool {
	return c < 0
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{
		idLookup:   make(map[string]int),
		nameLookup: make(map[int]string),
		graph:      make(map[int]*common.Set),
	}

	result.idLookup = make(map[string]int)
	result.idLookup["start"] = -1
	result.idLookup["end"] = -2
	result.nameLookup[-1] = "start"
	result.nameLookup[-2] = "end"
	nextId := 3

	lookup := func(s string) int {
		v, ok := result.idLookup[s]
		if ok {
			return v
		}
		v = nextId
		nextId++
		if isSmallCave(s) {
			v = -v
		}
		result.idLookup[s] = v
		result.nameLookup[v] = s
		return v
	}

	for _, l := range s {
		parts := strings.Split(l, "-")
		p0id := lookup(parts[0])
		p1id := lookup(parts[1])
		if result.graph[p0id] == nil {
			result.graph[p0id] = &common.Set{}
		}
		if result.graph[p1id] == nil {
			result.graph[p1id] = &common.Set{}
		}
		result.graph[p0id].Add(p1id)
		result.graph[p1id].Add(p0id)
	}

	return result, nil
}

type Path struct {
	smallCavesVisited *common.Set
	dupedOne          bool
	location          int
	// visited []string
}

func countPaths(pi ParsedInput, noRevisit bool) int {
	fringe := make([]Path, 0)

	basePath := Path{
		smallCavesVisited: &common.Set{},
		dupedOne:          noRevisit,
		location:          -1}

	fringe = append(fringe, basePath)

	endPaths := make([]Path, 0)

	visit := func(p Path) {
		if p.location == -2 {
			//fmt.Printf("%v - end\n", p.visited)
			endPaths = append(endPaths, p)
			return
		}

		nextSmall := p.smallCavesVisited.Copy()
		nextDuped := p.dupedOne
		if isSmallCaveId(p.location) {
			if p.smallCavesVisited.Has(p.location) {

				if p.dupedOne || p.location == -1 {
					return
				}
				nextDuped = true
			}
			nextSmall.Add(p.location)
		}

		//nextVisited := append(p.visited, p.location)
		//fmt.Println(nextVisited)

		//fmt.Printf("Extending: ")
		for _, nextI := range pi.graph[p.location].AsSlice() {
			next := nextI.(int)
			//fmt.Printf("%s;", next)
			newPath := Path{
				smallCavesVisited: nextSmall,
				dupedOne:          nextDuped,
				location:          next,
				//	visited:        nextVisited,
			}
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
