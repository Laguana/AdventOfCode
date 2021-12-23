package day23

import (
	"AoC2021/common"
	"io"
)

/*
#############
#01.2.3.4.56#
###D#B#D#A###
  #C#C#A#B#
  #########

*/
// state: there are 2 left spots, 3 middle spots, 2 right spots
// and then the wells

type roomState struct {
	hallway [7]int    // numbered according to comment above
	rooms   [4][4]int // room[x][0] is closer to corridoor
}

type ParsedInput struct {
	state roomState
}

func parseInput(s []string) (ParsedInput, error) {
	result := ParsedInput{}

	lookup := map[byte]int{
		'A': 1,
		'B': 2,
		'C': 3,
		'D': 4,
	}

	for i, l := range s[2:4] {
		room := 0
		for j := 3; j < 10; j += 2 {
			result.state.rooms[room][i] = lookup[l[j]]
			room++
		}
	}

	for i := 2; i < 4; i++ {
		for room := 0; room < 4; room++ {
			result.state.rooms[room][i] = room + 1
		}
	}

	return result, nil
}

func hallRoomMoveCost(s roomState, dest, hallway int) int {
	steps := 0
	i := hallway
	if i < 2 || (i == 2 && dest > 0) || (i == 3 && dest > 1) || i == 4 && dest == 3 {
		// going right
		switch i {
		case 0:
			if s.hallway[1] != 0 {
				return -1
			}
			steps++
			fallthrough
		case 1:
			steps += 2
			if dest == 0 {
				return steps
			}
			if s.hallway[2] != 0 {
				return -1
			}
			fallthrough
		case 2:
			steps += 2
			if dest == 0 || dest == 1 {
				return steps
			}
			if s.hallway[3] != 0 {
				return -1
			}
			fallthrough
		case 3:
			steps += 2
			if dest == 1 || dest == 2 {
				return steps
			}
			if dest == 0 {
				panic("Need to go left in go right")
			}
			if s.hallway[4] != 0 {
				return -1
			}
			fallthrough
		case 4:
			steps += 2
			if dest == 2 || dest == 3 {
				return steps
			}
			panic("Need to go left and think we're going right")
		default:
			panic("invalid for going right")
		}
	} else {
		// going left
		switch i {
		case 6:
			if s.hallway[5] != 0 {
				return -1
			}
			steps++
			fallthrough
		case 5:
			steps += 2
			if dest == 3 {
				return steps
			}
			if s.hallway[4] != 0 {
				return -1
			}
			fallthrough
		case 4:
			steps += 2
			if dest == 2 || dest == 3 {
				return steps
			}
			if s.hallway[3] != 0 {
				return -1
			}
			fallthrough
		case 3:
			steps += 2
			if dest == 1 || dest == 2 {
				return steps
			}
			if dest != 0 {
				panic("Need to go right when going left")
			}
			if s.hallway[2] != 0 {
				return -1
			}
			fallthrough
		case 2:
			steps += 2
			if dest == 0 || dest == 1 {
				return steps
			}
			panic("Need to go right and think we're going left")
		default:
			panic("invalid for going left")
		}
	}
}

func hallwayRoomCost(s roomState, i int) int {
	// steps from hallway i to room j, -1 if impassible
	steps := 0
	dest := s.hallway[i] - 1
	if dest < 0 {
		// not starting from a valid position
		//fmt.Printf("nothing to move\n")
		return -1
	}
	if s.rooms[dest][0] != 0 {
		// destination is full
		//fmt.Printf("full destination: %d\n", s.rooms[dest][0])
		return -1
	} else {
		for pos := 1; pos < 4; pos++ {
			pe := s.rooms[dest][pos]
			if pe != 0 && pe != dest+1 {
				// Can't go in the room, it's got others in it
				return -1
			}
			if pe == 0 {
				// move on back
				steps++
			}
		}
	}
	s2 := hallRoomMoveCost(s, dest, i)
	if s2 < 0 {
		return s2
	}
	return steps + s2
}

func roomHallwayCost(s roomState, room, hallway int) (int, int) {
	// the cost to move the top element of room to hallway
	steps := 0
	elt := s.rooms[room][0]
	if elt == 0 {
		steps++
		elt = s.rooms[room][1]
	}
	if elt == 0 {
		steps++
		elt = s.rooms[room][2]
	}
	if elt == 0 {
		steps++
		elt = s.rooms[room][3]
		if room == elt-1 {
			// this is in place, shouldn't be removed
			return -1, 0
		}
	}
	if elt == 0 {
		return -1, 0
	}
	if s.hallway[hallway] != 0 {
		// destination is full
		return -1, 0
	}
	s2 := hallRoomMoveCost(s, room, hallway)
	if s2 < 0 {
		return s2, elt
	}
	return steps + s2, elt
}

var costLookup [4]int = [4]int{1, 10, 100, 1000}

func (s roomState) Successors() <-chan common.SearchStateSuccessor {
	ch := make(chan common.SearchStateSuccessor)

	go func() {
		for i := 0; i < 7; i++ {
			steps := hallwayRoomCost(s, i)
			//fmt.Printf("moving %d? %d\n", i, steps)
			if steps < 0 {
				continue
			}
			// move the thing in spot i to its room
			elt := s.hallway[i]
			dCost := steps * costLookup[elt-1]
			t := roomState{}
			for j := 0; j < 7; j++ {
				t.hallway[j] = s.hallway[j]
			}
			t.hallway[i] = 0
			for j := 0; j < 4; j++ {
				for k := 0; k < 4; k++ {
					t.rooms[j][k] = s.rooms[j][k]
				}
			}
			if t.rooms[elt-1][3] == 0 {
				t.rooms[elt-1][3] = elt
			} else if t.rooms[elt-1][2] == 0 {
				t.rooms[elt-1][2] = elt
			} else if t.rooms[elt-1][1] == 0 {
				t.rooms[elt-1][1] = elt
			} else {
				t.rooms[elt-1][0] = elt
			}

			ch <- common.SearchStateSuccessor{t, dCost}
		}
		// moving from room to hallway
		for room := 0; room < 4; room++ {
			for hall := 0; hall < 7; hall++ {
				steps, elt := roomHallwayCost(s, room, hall)
				//fmt.Printf("%v -> %d/%d = %d,%d\n", s, room, hall, steps, elt)
				if steps < 0 {
					continue
				}
				dCost := steps * costLookup[elt-1]
				t := roomState{}
				for j := 0; j < 7; j++ {
					t.hallway[j] = s.hallway[j]
				}
				t.hallway[hall] = elt
				for j := 0; j < 4; j++ {
					for k := 0; k < 4; k++ {
						t.rooms[j][k] = s.rooms[j][k]
					}
				}
				if t.rooms[room][0] == elt {
					t.rooms[room][0] = 0
				} else if t.rooms[room][1] == elt {
					t.rooms[room][1] = 0
				} else if t.rooms[room][2] == elt {
					t.rooms[room][2] = 0
				} else if t.rooms[room][3] == elt {
					t.rooms[room][3] = 0
				} else {
					panic("Moving from wrong place?")
				}
				ch <- common.SearchStateSuccessor{t, dCost}
			}
		}
		close(ch)
	}()

	return ch
}

func estimateCost(ss common.SearchState) int {
	s := ss.(roomState)
	/*
		#01.2.3.4.56#
		###D#B#D#A###
	*/
	roomDistanceMap := [4][4]int{{0, 4, 6, 8}, {4, 0, 4, 6}, {6, 4, 0, 4}, {8, 6, 4, 0}}
	hallDistanceMap := [4][7]int{{3, 2, 2, 4, 6, 8, 9}, {5, 4, 2, 2, 4, 6, 7}, {7, 6, 4, 2, 2, 3}, {9, 8, 6, 4, 2, 2, 3}}
	cost := 0
	for i := 0; i < 7; i++ {
		elt := s.hallway[i] - 1
		if elt < 0 {
			continue
		}
		cost += hallDistanceMap[elt][i] * costLookup[elt]
	}
	for i := 0; i < 4; i++ {
		for j := 0; j < 4; j++ {
			elt := s.rooms[i][j] - 1
			if elt < 0 {
				continue
			}
			cost += (roomDistanceMap[i][elt] + j) * costLookup[elt]
		}
	}
	return cost

}

func (s roomState) Valid() bool {
	counts := [4]int{0, 0, 0, 0}
	for _, v := range s.hallway {
		if v != 0 {
			counts[v-1]++
		}
	}
	for _, v := range s.rooms {
		if v[0] != 0 {
			counts[v[0]-1]++
			if v[1] == 0 {
				return false
			}
		}
		if v[1] != 0 {
			counts[v[1]-1]++
			if v[2] == 0 {
				return false
			}
		}
		if v[2] != 0 {
			counts[v[2]-1]++
			if v[3] == 0 {
				return false
			}
		}
		if v[3] != 0 {
			counts[v[3]-1]++
		}
	}
	for _, v := range counts {
		if v != 4 {
			return false
		}
	}
	return true
}

func isSolved(ss common.SearchState) bool {
	s := ss.(roomState)
	for _, e := range s.hallway {
		if e != 0 {
			return false
		}
	}
	for room, e := range s.rooms {
		if e[0]-1 != room || e[1]-1 != room {
			return false
		}
	}
	return true
}

type searchNode struct {
	state     roomState
	cost      int
	heuristic int
	open      bool
	closed    bool
	parent    *searchNode
}

type searchNodeMap map[roomState]*searchNode

func (snm searchNodeMap) get(s roomState) *searchNode {
	v, ok := snm[s]
	if !ok {
		v = &searchNode{state: s}
		snm[s] = v
	}
	return v
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	result := common.Astar(pi.state, isSolved, estimateCost)

	return result.Cost, nil
}

func adjustForPart2(s roomState) roomState {
	// #D#C#B#A#
	// #D#B#A#C#
	for i := 0; i < 4; i++ {
		s.rooms[i][3] = s.rooms[i][1]
	}
	s.rooms[0][1] = 4
	s.rooms[0][2] = 4
	s.rooms[1][1] = 3
	s.rooms[1][2] = 2
	s.rooms[2][1] = 2
	s.rooms[2][2] = 1
	s.rooms[3][1] = 1
	s.rooms[3][2] = 3
	return s
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	p2s := adjustForPart2(pi.state)

	result := common.Astar(p2s, isSolved, estimateCost)

	return result.Cost, nil
}
