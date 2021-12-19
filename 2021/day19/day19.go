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
	beacons []triple
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
			result.scanners = append(result.scanners, relativeBeacons{beacons})
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
		result.scanners = append(result.scanners, relativeBeacons{beacons})
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

func alignAll(pi ParsedInput) []alignmentData {
	// alignment[i][j] = {(dx,dy,dz), (ax,ay,az)}
	// if scanner i
	n := len(pi.scanners)
	alignment := make([][]alignmentData, n)
	for i := 0; i < n; i++ {
		alignment[i] = make([]alignmentData, n)
	}

	zero := triple{0, 0, 0}

	for i := 0; i < n; i++ {
		// attempt to align i with others
		a := pi.scanners[i]
		for j := 0; j < n; j++ {
			if j == i {
				continue
			}
			if alignment[j][i].delta != zero {
				// Already aligned
				continue
			}
			data := align(a, pi.scanners[j])
			if data.delta != zero {
				alignment[i][j] = data
				alignment[j][i] = reverseAlign(data)
				fmt.Printf("%d %d\n%v\n%v\n", i, j, alignment[i][j], alignment[j][i])

				for k := 0; k < n; k++ {
					if k != i && k != j && alignment[j][k].delta != zero && alignment[i][k].delta == zero {
						alignment[i][k] = realign(alignment[i][j], alignment[j][k])
						alignment[k][i] = reverseAlign(alignment[i][k])
						//fmt.Printf("-- %d\n-- %v\n-- %v\n", k, alignment[j][k], alignment[i][k])
					}
					if k != i && k != j && alignment[k][i].delta != zero && alignment[k][j].delta == zero {
						alignment[k][j] = realign(alignment[k][i], alignment[i][j])
						alignment[j][k] = reverseAlign(alignment[k][j])
						//fmt.Printf("-- %d\n-- %v\n-- %v\n", k, alignment[k][i], alignment[k][j])
					}
				}
			}
		}
	}

	fmt.Println(alignment)

	return alignment[0]

}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	_, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	return 0, fmt.Errorf("not implemented")
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	_, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	return 0, fmt.Errorf("not implemented")
}
