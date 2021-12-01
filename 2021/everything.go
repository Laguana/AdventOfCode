package main

import (
	"AoC2021/day1"
	"fmt"
	"os"
)

func doDay1() {
	d1p1i, err := os.Open("day1/p1.input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer d1p1i.Close()
	d1p1, err := day1.Part1(d1p1i)
	if err != nil {
		fmt.Println("Day 1 Part 1 had an error: ", err)
		os.Exit(1)
	}
	fmt.Printf("Day 1 Part 1: %d\n", d1p1)

	d1p2i, err := os.Open("day1/p1.input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer d1p2i.Close()
	d1p2, err := day1.Part2(d1p2i)
	if err != nil {
		fmt.Println("Day 1 Part 2 had an error: ", err)
		os.Exit(1)
	}
	fmt.Printf("Day 1 Part 2: %d\n", d1p2)
}

func main() {
	doDay1()
}
