package main

import (
	"AoC2021/day1"
	"AoC2021/day2"
	"AoC2021/day3"
	"AoC2021/day4"
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

func doDay2() {
	{
		d2i, err := os.Open("day2/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d2i.Close()
		d2p1, err := day2.Part1(d2i)
		if err != nil {
			fmt.Println("Day 2 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 2 Part 1: %d\n", d2p1)
	}

	{
		d2i, err := os.Open("day2/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d2i.Close()
		d2p1, err := day2.Part2(d2i)
		if err != nil {
			fmt.Println("Day 2 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 2 Part 2: %d\n", d2p1)
	}
}

func doDay3() {
	{
		d3i, err := os.Open("day3/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d3i.Close()
		d3p1, err := day3.Part1(d3i)
		if err != nil {
			fmt.Println("Day 3 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 3 Part 1: %d\n", d3p1)
	}

	{
		d3i, err := os.Open("day3/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d3i.Close()
		d3p1, err := day3.Part2(d3i)
		if err != nil {
			fmt.Println("Day 3 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 3 Part 2: %d\n", d3p1)
	}
}

func doDay4() {
	{
		d4i, err := os.Open("day4/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d4i.Close()
		d4p1, err := day4.Part1(d4i)
		if err != nil {
			fmt.Println("Day 4 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 4 Part 1: %d\n", d4p1)
	}

}

func main() {
	doDay1()
	doDay2()
	doDay3()
	doDay4()
}
