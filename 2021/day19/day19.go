package day19

import (
	"AoC2021/common"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type axis int

const (
	unusedAxis axis = iota
	X
	Y
	Z
)

func (a axis) sameAxis(b axis) bool {
	return a == b || a == -b
}

func (a axis) abs() axis {
	if a < 0 {
		return -a
	}
	return a
}

func combineAxes(ax, ay, az, b axis) axis {
	switch b {
	case X:
		return ax
	case -X:
		return -ax
	case Y:
		return ay
	case -Y:
		return -ay
	case Z:
		return az
	case -Z:
		return -az
	default:
		panic("Invalid axis")
	}
}

func reverseAxes(ax, ay, az axis) (x, y, z axis) {
	// if frame A's x is frame B's ax, then
	// frame B's x is whichever of frame A's axes it maps to

	// This hurts my brain, hardcode basically

	if ax.abs() == X {
		return ax, ay, az
	} else if ax.abs() == Y {
		if az.abs() == Z {
			return ax, ay, az
		} else {
			// az == X, ay == Z
			return ay, az, ax
		}
	} else {
		if ay.abs() == Y {
			return ax, ay, az
		} else {
			return az, ax, ay
		}
	}
}

func allAxes() <-chan axis {
	ch := make(chan axis)

	go func() {
		ch <- X
		ch <- -X
		ch <- Y
		ch <- -Y
		ch <- Z
		ch <- -Z
		close(ch)
	}()

	return ch
}

type triple struct {
	x, y, z int
}

func (t triple) getCoord(ax axis) int {
	switch ax {
	case X:
		return t.x
	case -X:
		return -t.x
	case Y:
		return t.y
	case -Y:
		return -t.y
	case Z:
		return t.z
	case -Z:
		return -t.z
	default:
		panic("invalid axis")
	}
}

type relativeBeacons struct {
	position triple
	beacons  []triple
}

type ParsedInput struct {
	scanners []relativeBeacons
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	scanner := 0
	newScanner := true
	var beacons []triple
	for _, v := range s {
		if newScanner {
			expectedLine := fmt.Sprintf("--- scanner %d ---", scanner)
			if v != expectedLine {
				return result, fmt.Errorf("expected scanner %d, got '%s'", scanner, v)
			}
			newScanner = false
			beacons = make([]triple, 0)
			continue
		}
		if v == "" {
			result.scanners = append(result.scanners, relativeBeacons{triple{0, 0, 0}, beacons})
			scanner++
			newScanner = true
			continue
		}
		parts := strings.Split(v, ",")
		x, err := strconv.Atoi(parts[0])
		if err != nil {
			return result, err
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			return result, err
		}
		z, err := strconv.Atoi(parts[2])
		if err != nil {
			return result, err
		}
		beacons = append(beacons, triple{x, y, z})
	}
	if !newScanner {
		result.scanners = append(result.scanners, relativeBeacons{triple{0, 0, 0}, beacons})
	}

	return result, nil
}

func alignBeacon(a, b triple, bx, by, bz axis) triple {
	// Assuming that a's x/y/z maps to bx, by, bz in Bs coordinate system
	// what is the position of 0,0,0 in B's coordinate system from As perspective

	pbx := b.getCoord(bx)
	pby := b.getCoord(by)
	pbz := b.getCoord(bz)

	// we are assuming that after rotating to have beacon B match A's point of reference
	// that it sees the beacon at pbx,pby,pbz while A sees the point at a.x,a.y,a.z
	// That means that A->B = A->a - (B->b) = A->a - pbx, pby,pbz
	return triple{a.x - pbx, a.y - pby, a.z - pbz}
}

func backtrackingAlign(a, b []triple, x, y, z axis) triple {
	var canMatchOthers func(delta triple, matched int) bool
	canMatchOthers = func(delta triple, matched int) bool {
		if matched == 12 {
			return true
		}
		for i := matched; i < len(a); i++ {
			va := a[i]
			for j := matched; j < len(b); j++ {
				vb := b[j]
				if delta != alignBeacon(va, vb, x, y, z) {
					continue
				}
				//fmt.Printf("Considering %d:%v ~= %d:%v  @ %d\n", i, va, j, vb, matched)
				a[matched], a[i] = a[i], a[matched]
				b[matched], b[j] = b[j], b[matched]
				if canMatchOthers(delta, matched+1) {
					//fmt.Printf("matching on %v/%v\n", va, vb)
					return true
				}
			}
		}
		return false
	}

	for i := 0; i < len(a); i++ {
		va := a[i]
		for j := 0; j < len(b); j++ {
			vb := b[j]
			assumedDiff := alignBeacon(va, vb, x, y, z)
			a[0], a[i] = a[i], a[0]
			b[0], b[j] = b[j], b[0]
			if canMatchOthers(assumedDiff, 1) {
				//fmt.Printf("matching on %v/%v\n", va, vb)
				return assumedDiff
			}
		}
	}
	return triple{0, 0, 0}
}

type alignmentData struct {
	delta   triple
	x, y, z axis
}

func align(a, b relativeBeacons) alignmentData {
	// Attempt to find the relative position between two scanners
	// If they do not align, returns (0,0,0) for the delta
	// If they do align, then in A's coordinate system, B is at position delta
	// and A's x/y/z correspond to the returned x,y,z values for b
	result := alignmentData{}
	zero := triple{0, 0, 0}

	for result.x = range allAxes() {
		for result.y = range allAxes() {
			if result.x.sameAxis(result.y) {
				continue
			}
			for result.z = range allAxes() {
				if result.x.sameAxis(result.z) || result.y.sameAxis(result.z) {
					continue
				}
				// If we assume that the conversion from A to B is that
				// B's x is A's 'x', etc, can we match 12 beacons
				result.delta = backtrackingAlign(a.beacons, b.beacons, result.x, result.y, result.z)
				if result.delta != zero {
					return result
				}
			}
		}
	}

	return result
}

func adjustRelativeBeacons(b *relativeBeacons, d alignmentData) {
	b.position = triple{x: d.delta.x, y: d.delta.y, z: d.delta.z}
	for i, v := range b.beacons {
		// d.delta comes from triple{a.x - pbx, a.y - pby, a.z - pbz}
		// to apply it, we would need to be doing it in as coordinate system
		//
		// For aligned cooridinates, we want (pbx,pby,pbz)+delta = (a.x,a.y,a.z)
		//                                                       = (pbx+a.x-pbx,...) checks out?
		x := v.getCoord(d.x)
		y := v.getCoord(d.y)
		z := v.getCoord(d.z)
		b.beacons[i] = triple{x: x + d.delta.x, y: y + d.delta.y, z: z + d.delta.z}
	}
}

func realign(a, b alignmentData) alignmentData {
	result := alignmentData{}
	zero := triple{0, 0, 0}
	if a.delta == zero || b.delta == zero {
		return result
	}
	// a is the delta between 0 and 1 in reference frame 0
	// and b is the delta between 1 and 2 in reference frame 1

	result.delta = triple{
		a.delta.x + b.delta.getCoord(a.x),
		a.delta.y + b.delta.getCoord(a.y),
		a.delta.z + b.delta.getCoord(a.z),
	}
	// a.x is the axis conversion from 1 to 0
	// and b.x is the axis conversion from 2 to 1
	// so to convert from 2 to 0 we need to convert twice
	result.x = combineAxes(a.x, a.y, a.z, b.x)
	result.y = combineAxes(a.x, a.y, a.z, b.y)
	result.z = combineAxes(a.x, a.y, a.z, b.z)
	return result
}

func reverseAlign(a alignmentData) alignmentData {
	result := alignmentData{}
	result.x, result.y, result.z = reverseAxes(a.x, a.y, a.z)
	result.delta = triple{-a.delta.getCoord(result.x), -a.delta.getCoord(result.y), -a.delta.getCoord(result.z)}
	return result
}

func alignAll(pi ParsedInput) {
	zero := triple{0, 0, 0}

	toAlign := &common.Set{}
	for i, _ := range pi.scanners {
		toAlign.Add(i)
	}
	alignedUnprocessed := &common.Set{}
	toAlign.Remove(0)
	alignedUnprocessed.Add(0)

	for toAlign.Size() > 0 && alignedUnprocessed.Size() > 0 {
		ai := alignedUnprocessed.Any().(int)
		alignedUnprocessed.Remove(ai)
		a := pi.scanners[ai]

		for _, bi := range toAlign.AsSlice() {
			b := pi.scanners[bi.(int)]
			data := align(a, b)
			if data.delta != zero {
				adjustRelativeBeacons(&b, data)
				pi.scanners[bi.(int)] = b
				toAlign.Remove(bi)
				alignedUnprocessed.Add(bi)
			}
		}
	}
	if toAlign.Size() > 0 {
		panic("Failed to align some")
	}
}

func countBeacons(pi ParsedInput) int {
	beacons := &common.Set{}
	for _, v := range pi.scanners {
		for _, b := range v.beacons {
			beacons.Add(b)
		}
	}
	return beacons.Size()
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	alignAll(pi)

	return countBeacons(pi), nil
}

func iabs(i int) int {
	if i < 0 {
		return -i
	} else {
		return i
	}
}

func manhattan(a, b triple) int {
	return iabs(a.x-b.x) + iabs(a.y-b.y) + iabs(a.z-b.z)
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	alignAll(pi)

	max := 0
	for _, a := range pi.scanners {
		for _, b := range pi.scanners {
			dist := manhattan(a.position, b.position)
			if dist > max {
				max = dist
			}
		}
	}

	return max, nil
}
