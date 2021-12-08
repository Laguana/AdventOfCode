package day8

import (
	"AoC2021/common"
	"io"
	"sort"
	"strings"
)

type Display struct {
	combinations []string
	result       []string
}

type ParsedInput []Display

func sortString(s string) string {
	var r []rune
	for _, runeValue := range s {
		r = append(r, runeValue)
	}

	sort.Slice(r, func(i, j int) bool {
		return r[i] < r[j]
	})
	return string(r)
}

func parseInput(s []string) (ParsedInput, error) {
	result := make([]Display, 0)

	for _, v := range s {
		t := strings.Split(v, " | ")
		left := strings.Split(t[0], " ")
		right := strings.Split(t[1], " ")

		sortedLeft := make([]string, 0)
		for _, l := range left {
			sortedLeft = append(sortedLeft, sortString(l))
		}
		sortedRight := make([]string, 0)
		for _, l := range right {
			sortedRight = append(sortedRight, sortString(l))
		}

		result = append(result, Display{combinations: sortedLeft, result: sortedRight})
	}

	return result, nil
}

func is1478(s string) bool {
	switch len(s) {
	case 2:
		// 1
		return true
	case 4:
		// 4
		return true
	case 3:
		// 7
		return true
	case 7:
		return true
	default:
		return false
	}
}

//  1111
// 2    3
// 2    3
//  4444
// 5    6
// 5    6
//  7777

func setOfChars(s string) *common.Set {
	z := &common.Set{}

	for _, r := range s {
		z.Add(r)
	}
	return z
}

func decodeDigits(d Display) int {
	digitMap := make(map[string]int)

	// I'm just going to ignore 0 since I went with 1 indexed
	var charSegmentOptions [8]common.Set
	segmentCharMap := make(map[int]rune)

	// We begin with maximum uncertainty
	for i := 1; i <= 7; i++ {
		for _, c := range "abcdefg" {
			(&charSegmentOptions[i]).Add(c)
		}
	}

	for _, v := range d.combinations {
		if len(v) == 7 {
			// we learn nothing from 8s
			digitMap[v] = 8
			continue
		}
		if len(v) == 2 {
			digitMap[v] = 1
			// a 1 has both segments 3 and 6, but we don't know which
			oneSet := setOfChars(v)
			(&charSegmentOptions[3]).Intersect(oneSet)
			(&charSegmentOptions[6]).Intersect(oneSet)
		}
		if len(v) == 3 {
			digitMap[v] = 7
			// a 7 hahs segments 1, 3 and 6
			sevenSet := setOfChars(v)
			(&charSegmentOptions[1]).Intersect(sevenSet)
			(&charSegmentOptions[3]).Intersect(sevenSet)
			(&charSegmentOptions[6]).Intersect(sevenSet)
		}
		if len(v) == 4 {
			digitMap[v] = 4
			// a 4 has segments 2,3,4,6
			fourSet := setOfChars(v)
			(&charSegmentOptions[2]).Intersect(fourSet)
			(&charSegmentOptions[3]).Intersect(fourSet)
			(&charSegmentOptions[4]).Intersect(fourSet)
			(&charSegmentOptions[6]).Intersect(fourSet)
		}
	}
	// The 7 and the 1 differ only in segment 1, so we can identify that
	(&charSegmentOptions[1]).Exclude(&charSegmentOptions[3])
	for i := 2; i <= 7; i++ {
		(&charSegmentOptions[i]).Exclude(&charSegmentOptions[1])
	}
	segment1 := (&charSegmentOptions[1]).AsSlice()[0]
	segmentCharMap[1] = segment1.(rune)

	// 4 is a superset of 1, so we can exclude segments
	// 2 and 4 from being the possibilities of segments 3 and 6
	(&charSegmentOptions[2]).Exclude(&charSegmentOptions[3])
	(&charSegmentOptions[4]).Exclude(&charSegmentOptions[3])

	// Now the harder numbers
	for _, v := range d.combinations {
		if len(v) == 6 {
			// This is either a 0, 6 or 9
			// 9 is missing a segment which is not
			// called out by any of the known things so far

			// 0 is missing something in common with the 4,
			// 6 is missing something in common with the 1,

			allChars := setOfChars("abcdefg")
			chars := setOfChars(v)
			allChars.Exclude(chars)
			missingChar := allChars.AsSlice()[0].(rune)
			if (&charSegmentOptions[3]).Has(missingChar) {
				//The missing segment must be 3, this must be a 6
				(&charSegmentOptions[3]).Intersect(allChars)
				segmentCharMap[3] = missingChar
				digitMap[v] = 6

				(&charSegmentOptions[6]).Exclude(&charSegmentOptions[3])
				segmentCharMap[6] = (&charSegmentOptions[6]).AsSlice()[0].(rune)
				continue
			} else if (&charSegmentOptions[4]).Has(missingChar) {
				// The missing segment must be 4, this must be a 0
				(&charSegmentOptions[4]).Intersect(allChars)
				segmentCharMap[4] = missingChar
				digitMap[v] = 0

				(&charSegmentOptions[2]).Exclude(&charSegmentOptions[4])
				segmentCharMap[2] = (&charSegmentOptions[2]).AsSlice()[0].(rune)
			} else {
				// This must be a 9, and the missing segment must be 5
				(&charSegmentOptions[5]).Intersect(allChars)
				segmentCharMap[5] = missingChar
				digitMap[v] = 9
			}
		}
	}
	// Just 2 3 5 left
	// Lets thin down some of the known combinations
	for i := 1; i <= 7; i++ {
		if (&charSegmentOptions[i]).Size() == 1 {
			for j := 1; j <= 7; j++ {
				if i != j {
					(&charSegmentOptions[j]).Exclude(&charSegmentOptions[i])
				}
			}
		}
	}
	// At this point we know segments 1, 2, 3, 4, 5 and 6, so we also know 7 by extension

	for _, v := range d.combinations {
		if len(v) == 5 {
			chars := setOfChars(v)
			if chars.Has(segmentCharMap[2]) {
				// Must be a 5
				digitMap[v] = 5
			} else if chars.Has(segmentCharMap[5]) {
				digitMap[v] = 2
			} else {
				digitMap[v] = 3
			}
		}
	}

	//fmt.Println(digitMap)
	//fmt.Println(segmentCharMap)

	sum := 0
	for _, v := range d.result {
		sum = 10*sum + digitMap[v]
	}

	return sum
}

func Part1(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	n1478 := 0

	for _, v := range pi {

		for _, d := range v.result {
			if is1478(d) {
				n1478++
			}
		}
	}

	return n1478, nil
}

func Part2(r io.Reader) (int, error) {
	input := common.ReadLinesToSlice(r)
	pi, err := parseInput(input)
	if err != nil {
		return 0, err
	}

	sum := 0

	for _, v := range pi {
		sum += decodeDigits(v)
	}

	return sum, nil
}
