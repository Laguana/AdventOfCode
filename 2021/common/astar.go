package common

import (
	"container/heap"
	"fmt"
)

type SearchStateSuccessor struct {
	State SearchState
	Cost  int
}

type SearchState interface {
	Successors() <-chan SearchStateSuccessor
	Valid() bool
}

type SearchNode struct {
	State     SearchState
	Cost      int
	heuristic int
	open      bool
	closed    bool
	Parent    *SearchNode
	index     int
}

type searchNodeMap map[SearchState]*SearchNode

func (snm searchNodeMap) get(s SearchState) *SearchNode {
	v, ok := snm[s]
	if !ok {
		v = &SearchNode{State: s}
		snm[s] = v
	}
	return v
}

type priorityQueue []*SearchNode

func (pq priorityQueue) Len() int { return len(pq) }
func (pq priorityQueue) Less(i, j int) bool {
	return pq[i].Cost+pq[i].heuristic < pq[j].Cost+pq[j].heuristic
}
func (pq priorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}
func (pq *priorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*SearchNode)
	item.index = n
	*pq = append(*pq, item)
}
func (pq *priorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

// update?

func Astar(start SearchState, done func(SearchState) bool, heuristic func(SearchState) int) *SearchNode {
	snm := searchNodeMap{}
	openList := make(priorityQueue, 0)

	startn := snm.get(start)
	startn.open = true
	startn.heuristic = heuristic(start)
	heap.Push(&openList, startn)

	for openList.Len() > 0 {
		sni := heap.Pop(&openList)
		sn := sni.(*SearchNode)

		if sn.closed {
			continue
		}

		sn.closed = true

		/*
			steps++
			if steps%1000 == 0 {
				fmt.Printf("%d: %d, %v\n", steps, len(openList), sn)
			}
		*/

		if done(sn.State) {
			return sn
		}
		for next := range sn.State.Successors() {
			if !next.State.Valid() {
				fmt.Printf("%v lead to invalid state %v\n", sn.State, next.State)
				panic("Invalid state")
			}
			//fmt.Printf("Succ: %v\n", next)
			s := snm.get(next.State)
			if s.closed {
				continue
			}
			cost := sn.Cost + next.Cost
			heuristic := heuristic(next.State)
			if s.open && s.Cost+s.heuristic < cost+heuristic {
				// already have a better path
				continue
			}

			s.Parent = sn
			s.Cost = cost
			s.heuristic = heuristic

			if s.open {
				// this is a better path to something already in the heap
				heap.Fix(&openList, s.index)
			} else {
				s.open = true
				heap.Push(&openList, s)
			}
		}
	}

	return nil
}
