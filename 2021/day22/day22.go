package day22

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type pair struct {
	a, b int
}

type step struct {
	on bool
	x  pair
	y  pair
	z  pair
}

type ParsedInput struct {
	steps []step
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}
	result.steps = make([]step, 0)

	parsePair := func(s string) (pair, error) {
		// strip off x=
		s = s[2:]
		parts := strings.Split(s, "..")
		pair := pair{}
		low, err := strconv.Atoi(parts[0])
		if err != nil {
			return pair, err
		}
		pair.a = low
		high, err := strconv.Atoi(parts[1])
		if err != nil {
			return pair, err
		}
		pair.b = high
		return pair, nil
	}

	for _, l := range s {
		e := step{}
		onoff := strings.Split(l, " ")
		if onoff[0] == "on" {
			e.on = true
		} else {
			e.on = false
		}
		coords := strings.Split(onoff[1], ",")
		x, err := parsePair(coords[0])
		if err != nil {
			return result, err
		}
		e.x = x
		y, err := parsePair(coords[1])
		if err != nil {
			return result, err
		}
		e.y = y
		z, err := parsePair(coords[2])
		if err != nil {
			return result, err
		}
		e.z = z
		result.steps = append(result.steps, e)
	}

	return result, nil
}

type octree struct {
	x, y, z  int // minimum position is x,y,z
	size     int // maximum position is x+size,y+size,z+size
	on       bool
	simple   bool
	children [8]*octree
	// 4*x+2*y+z children octrees
}

func newOctree(x, y, z, size int, on bool) *octree {
	result := &octree{}
	result.x = x
	result.y = y
	result.z = z
	result.size = size
	result.on = on
	result.simple = true

	//fmt.Printf("-- New child %v\n", result)

	return result
}

func (p pair) clamp(min, max int) pair {
	result := p
	if p.a < min {
		result.a = min
	}
	if p.b > max {
		result.b = max
	}
	return result
}

func (o *octree) reduce() {
	if o.simple {
		return
	}

	// all same
	simpleChildren := true
	for i := 0; i < 8; i++ {
		simpleChildren = simpleChildren && o.children[i].simple
	}
	if !simpleChildren {
		return
	}
	childState := o.children[0].on
	for i := 1; i < 8; i++ {
		if childState != o.children[i].on {
			return
		}
	}
	// all children are the same state. become simple
	o.on = childState
	o.simple = true
	for i := 0; i < 8; i++ {
		o.children[i] = nil
	}
}

func (o *octree) update(on bool, x, y, z pair) bool {
	// Given an octree, update the x,y,z prism to be on
	// Is this prism contained in this tree?

	//fmt.Printf("?(%d,%d,%d)x%d / %v %v %v\n", o.x, o.y, o.z, o.size, x, y, z)
	if o.x > x.b || o.y > y.b || o.z > z.b ||
		o.x+o.size-1 < x.a || o.y+o.size-1 < y.a || o.z+o.size-1 < z.a {
		return false
	}

	// part of the prism intersects with this octree
	x = x.clamp(o.x, o.x+o.size-1)
	y = y.clamp(o.y, o.y+o.size-1)
	z = z.clamp(o.z, o.z+o.size-1)
	// the new x/y/z are now strictly within bounds

	//fmt.Printf("(%d,%d,%d)x%d / %v %v %v\n", o.x, o.y, o.z, o.size, x, y, z)

	if o.x == x.a && o.y == y.a && o.z == z.a &&
		x.b-x.a == o.size-1 && y.b-y.a == o.size-1 && z.b-z.a == o.size-1 {
		// the whole of this cube is modifed
		o.on = on
		o.simple = true
		for i := range o.children {
			o.children[i] = nil
		}
		return true
	}
	// A subset of the cube is modified
	if o.on == on && o.simple {
		// but it is a no-op
		return false
	}

	if o.size == 1 {
		// we are a single cube anyway
		// how did we get here??
		panic("should have hit the 'entire cube' check")
	}

	o.simple = false

	updated := false

	for dx := 0; dx < 2; dx++ {
		for dy := 0; dy < 2; dy++ {
			for dz := 0; dz < 2; dz++ {
				idx := 4*dx + 2*dy + dz
				child := o.children[idx]
				if child == nil {
					childsize := o.size / 2
					cx := o.x + dx*childsize
					cy := o.y + dy*childsize
					cz := o.z + dz*childsize
					child = newOctree(cx, cy, cz, childsize, o.on)
					o.children[idx] = child
				}
				updated = child.update(on, x, y, z) || updated

			}
		}
	}

	if updated {
		o.reduce()
	}

	return updated
}

func (o *octree) countOn() int {
	if o.simple {
		if !o.on {
			return 0
		}
		//fmt.Printf("** %v\n", o)
		return o.size * o.size * o.size
	}
	count := 0
	for i := 0; i < 8; i++ {
		count += o.children[i].countOn()
	}
	return count
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	o := newOctree(-64, -64, -64, 128, false)
	for _, instr := range pi.steps {
		o.update(instr.on, instr.x, instr.y, instr.z)
	}

	return o.countOn(), nil
}

func iabs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func (a pair) intersect(b pair) pair {
	result := a
	if a.b < b.a || b.b < a.a {
		return pair{1, -1}
	}
	if b.b < result.b {
		result.b = b.b
	}
	if result.a < b.a {
		result.a = b.a
	}
	return result
}

type prism struct {
	x, y, z pair
}

func (a prism) intersect(b prism) prism {
	result := prism{}
	result.x = a.x.intersect(b.x)
	result.y = a.y.intersect(b.y)
	result.z = a.z.intersect(b.z)
	//fmt.Printf("%v (+) %v = %v\n", a, b, result)
	return result
}

func (a prism) collides(b prism) bool {
	t := a.intersect(b)
	return t.x.a <= t.x.b && t.y.a <= t.y.b && t.z.a <= t.z.b
}

func (a prism) subtract(b prism) []prism {
	result := make([]prism, 0)

	if !(a.collides(b)) {
		return append(result, a)
	}

	// we want to subdivide a into a number of prisms, none of which is in b
	// if there is a segment of a in the -x direction of b, create a prism for that

	if b.x.a > a.x.a {
		t := prism{
			x: pair{a.x.a, b.x.a - 1},
			y: a.y,
			z: a.z}
		result = append(result, t)
		// shrink the prism under consideration to exclude that now
		a.x.a = b.x.a
	}
	// if there is a segment in the +x direction, create that too
	if b.x.b < a.x.b {
		t := prism{
			x: pair{b.x.b + 1, a.x.b},
			y: a.y,
			z: a.z}
		result = append(result, t)
		// shrink the prism under consideration to exclude that now
		a.x.b = b.x.b
	}
	// now we just need the top and sides
	// if there is -y
	if b.y.a > a.y.a {
		t := prism{
			x: a.x,
			y: pair{a.y.a, b.y.a - 1},
			z: a.z}
		result = append(result, t)
		// shrink the prism under consideration to exclude that now
		a.y.a = b.y.a
	}
	if b.y.b < a.y.b {
		t := prism{
			x: a.x,
			y: pair{b.y.b + 1, a.y.b},
			z: a.z}
		result = append(result, t)
		// shrink the prism under consideration to exclude that now
		a.y.b = b.y.b
	}
	// finally, the front/back segments
	if b.z.a > a.z.a {
		t := prism{
			x: a.x,
			y: a.y,
			z: pair{a.z.a, b.z.a - 1}}
		result = append(result, t)
		// shrink the prism under consideration to exclude that now
		a.z.a = b.z.a
	}
	if b.z.b < a.z.b {
		t := prism{
			x: a.x,
			y: a.y,
			z: pair{b.z.b + 1, a.z.b}}
		result = append(result, t)
		// shrink the prism under consideration to exclude that now
		a.z.b = b.z.b
	}

	return result
}

type constructiveScene struct {
	on       bool
	bounds   prism
	children []*constructiveScene
}

// prisms are inclusive of their endpoints
func (p prism) valid() bool {
	return p.x.b >= p.x.a && p.y.b >= p.y.a && p.z.b >= p.z.a
}

func (cs *constructiveScene) count() (size, other int, on bool) {
	//fmt.Printf("%v\n", *cs)
	size = (cs.bounds.x.b - cs.bounds.x.a + 1) * (cs.bounds.y.b - cs.bounds.y.a + 1) * (cs.bounds.z.b - cs.bounds.z.a + 1)
	other = 0
	on = cs.on

	for _, c := range cs.children {
		if c != nil {
			csize, cother, con := c.count()
			if con != on {
				other += csize - cother
			} else {
				panic("should be alternating")
				//other += cother
			}

		}
	}
	return
}

func (cs *constructiveScene) add(on bool, p prism) (modified bool, removed bool) {
	modified = false
	removed = false
	p = p.intersect(cs.bounds)
	if !p.valid() {
		return
	}
	if p == cs.bounds {
		//fmt.Printf("Whole region: %v\n", p)
		// this whole area is now set to this value
		if on == cs.on {
			// remove all children
			if len(cs.children) > 0 {
				cs.children = make([]*constructiveScene, 0)
				modified = true
			}
		} else {
			// remove this element
			modified = true
			removed = true
		}
		//fmt.Printf("Whole region: %v %v \n", removed, *cs)
		return
	}
	if on == cs.on {
		// it is filling out some space; remove from children
		// which potentially may remove children

		notRemoved := 0
		for i := range cs.children {
			if cs.children[i] == nil {
				continue
			}
			cmod, cremove := cs.children[i].add(on, p)
			if cremove {
				cs.children[i] = nil
			} else {
				notRemoved++
			}
			if cmod {
				modified = true
			}
		}
		if notRemoved == 0 {
			// everything removed
			cs.children = make([]*constructiveScene, 0)
		}
	} else {
		// This is removing some space; add a child probably
		// Invariant: children do not overlap
		// so we first split the incoming prism into existing children,
		// and then add the remainder

		prisms := &common.Set{}
		prisms.Add(p)

		for _, v := range cs.children {
			if v == nil {
				continue
			}
			for _, pint := range prisms.AsSlice() {
				pe := pint.(prism)
				if pe.collides(v.bounds) {
					cmod, _ := v.add(on, pe)
					if cmod {
						modified = true
					}
					prisms.Remove(pint)
					for _, pnew := range pe.subtract(v.bounds) {
						if !pnew.valid() {
							fmt.Printf("%v - %v has %v?\n", pe, v.bounds, pnew)
							panic("Invalid prism somehow")
						}
						prisms.Add(pnew)
					}
				}
			}
		}

		for _, vint := range prisms.AsSlice() {
			pe := vint.(prism)
			if !pe.valid() {
				fmt.Printf("%v %v %v \n", pe, *cs, p)
				panic("Invalid prism somehow???")
			}
			//fmt.Printf("Adding child %v\n", pe)
			cs.children = append(cs.children, &constructiveScene{on: on, bounds: pe, children: nil})
			modified = true
		}

	}

	return
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	x := pair{0, 0}
	y := pair{0, 0}
	z := pair{0, 0}
	for _, v := range pi.steps {
		if v.x.a < x.a {
			x.a = v.x.a
		}
		if v.x.b > x.b {
			x.b = v.x.b
		}
		if v.y.a < y.a {
			y.a = v.y.a
		}
		if v.y.b > y.b {
			y.b = v.y.b
		}
		if v.z.a < z.a {
			z.a = v.z.a
		}
		if v.z.b > z.b {
			z.b = v.z.b
		}
	}

	space := &constructiveScene{on: false, bounds: prism{x: x, y: y, z: z}}

	for _, instr := range pi.steps {
		space.add(instr.on, prism{instr.x, instr.y, instr.z})
		//fmt.Printf("Step %d\n", i)
	}

	_, on, _ := space.count()

	return on, nil
}
