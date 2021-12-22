package day22

import (
	"fmt"
	"os"
	"strings"
	"testing"
)

var sampleInput = strings.Split(
	`on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10`, "\n")

func TestDay22Part1Sample(t *testing.T) {
	pi, err := parseInput(sampleInput)
	if err != nil {
		t.Error(err)
	}

	o := newOctree(-64, -64, -64, 128, false)
	for _, instr := range pi.steps {
		o.update(instr.on, instr.x, instr.y, instr.z)
		//fmt.Printf("* %d\n", o.countOn())
	}

	on := o.countOn()
	if on != 39 {
		t.Errorf("Should have been 39 cubes on, had %d", on)
	}
}

func TestDay22Part1(t *testing.T) {
	d22i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d22i.Close()
	result, err := Part1(d22i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := 537042
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

var sampleInput2 = strings.Split(
	`on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15`, "\n")

func TestDay22Part2Sample(t *testing.T) {
	pi, err := parseInput(sampleInput2)
	if err != nil {
		t.Error(err)
	}

	universe := pair{-(1 << 31), 1 << 31}
	cs := &constructiveScene{on: false, bounds: prism{x: universe, y: universe, z: universe}}
	for _, instr := range pi.steps {
		cs.add(instr.on, prism{instr.x, instr.y, instr.z})
		size, other, _ := cs.count()
		fmt.Printf("* %d %d \n", size, other)
	}

	_, on, _ := cs.count()
	if on != 2758514936282235 {
		t.Errorf("Should have been 2758514936282235 cubes on, had %d", on)
	}
}

func TestDay22Part2(t *testing.T) {
	d22i, err := os.Open("input.txt")
	if err != nil {
		t.Error(err)
		return
	}
	defer d22i.Close()
	result, err := Part2(d22i)
	if err != nil {
		t.Error(err)
		return
	}
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
