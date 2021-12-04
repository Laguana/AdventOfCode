package common

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

func (s *Set) Has(e interface{}) bool {
	_, ok := s.items[e]
	return ok
}
