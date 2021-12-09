package common

import "fmt"

type Set struct {
	items map[interface{}]struct{}
}

func (s *Set) Add(e interface{}) *Set {
	if s.items == nil {
		s.items = make(map[interface{}]struct{})
	}
	s.items[e] = struct{}{}
	return s
}

func (s *Set) Remove(e interface{}) *Set {
	_, ok := s.items[e]
	if ok {
		delete(s.items, e)
	}
	return s
}

func (s *Set) Has(e interface{}) bool {
	_, ok := s.items[e]
	return ok
}

func (s *Set) Intersect(t *Set) {
	for v, _ := range s.items {
		if !t.Has(v) {
			delete(s.items, v)
		}
	}
}

func (s *Set) Exclude(t *Set) {
	for v, _ := range s.items {
		if t.Has(v) {
			delete(s.items, v)
		}
	}
}

func (s *Set) Size() int {
	if s.items == nil {
		return 0
	}
	return len(s.items)
}

func (s *Set) AsSlice() []interface{} {
	var result []interface{}
	for v := range s.items {
		result = append(result, v)
	}
	return result
}

func (s *Set) Any() interface{} {
	for v := range s.items {
		return v
	}
	panic("no elements")
}

func (s *Set) Print() {
	fmt.Println(s.items)
}
