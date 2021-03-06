package main

import (
	"AoC2021/day1"
	"AoC2021/day10"
	"AoC2021/day11"
	"AoC2021/day12"
	"AoC2021/day13"
	"AoC2021/day14"
	"AoC2021/day15"
	"AoC2021/day16"
	"AoC2021/day17"
	"AoC2021/day18"
	"AoC2021/day19"
	"AoC2021/day2"
	"AoC2021/day20"
	"AoC2021/day21"
	"AoC2021/day22"
	"AoC2021/day23"
	"AoC2021/day3"
	"AoC2021/day4"
	"AoC2021/day5"
	"AoC2021/day6"
	"AoC2021/day7"
	"AoC2021/day8"
	"AoC2021/day9"
	"flag"
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
		d2p2, err := day2.Part2(d2i)
		if err != nil {
			fmt.Println("Day 2 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 2 Part 2: %d\n", d2p2)
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
		d3p2, err := day3.Part2(d3i)
		if err != nil {
			fmt.Println("Day 3 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 3 Part 2: %d\n", d3p2)
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

	{
		d4i, err := os.Open("day4/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d4i.Close()
		d4p2, err := day4.Part2(d4i)
		if err != nil {
			fmt.Println("Day 4 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 4 Part 2: %d\n", d4p2)
	}
}

func doDay5() {
	{
		d5i, err := os.Open("day5/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d5i.Close()
		d5p1, err := day5.Part1(d5i)
		if err != nil {
			fmt.Println("Day 5 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 5 Part 1: %d\n", d5p1)
	}

	{
		d5i, err := os.Open("day5/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d5i.Close()
		d5p2, err := day5.Part2(d5i)
		if err != nil {
			fmt.Println("Day 5 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 5 Part 2: %d\n", d5p2)
	}
}

func doDay6() {
	{
		d6i, err := os.Open("day6/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d6i.Close()
		d6p1, err := day6.Part1(d6i)
		if err != nil {
			fmt.Println("Day 6 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 6 Part 1: %d\n", d6p1)
	}

	{
		d6i, err := os.Open("day6/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d6i.Close()
		d6p2, err := day6.Part2(d6i)
		if err != nil {
			fmt.Println("Day 6 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 6 Part 2: %d\n", d6p2)
	}
}

func doDay7() {
	{
		d7i, err := os.Open("day7/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d7i.Close()
		d7p1, err := day7.Part1(d7i)
		if err != nil {
			fmt.Println("Day 7 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 7 Part 1: %d\n", d7p1)
	}

	{
		d7i, err := os.Open("day7/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d7i.Close()
		d7p2, err := day7.Part2(d7i)
		if err != nil {
			fmt.Println("Day 7 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 7 Part 2: %d\n", d7p2)
	}
}

func doDay8() {
	{
		d8i, err := os.Open("day8/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d8i.Close()
		d8p1, err := day8.Part1(d8i)
		if err != nil {
			fmt.Println("Day 8 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 8 Part 1: %d\n", d8p1)
	}

	{
		d8i, err := os.Open("day8/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d8i.Close()
		d8p2, err := day8.Part2(d8i)
		if err != nil {
			fmt.Println("Day 8 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 8 Part 2: %d\n", d8p2)
	}
}

func doDay9() {
	{
		d9i, err := os.Open("day9/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d9i.Close()
		d9p1, err := day9.Part1(d9i)
		if err != nil {
			fmt.Println("Day 9 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 9 Part 1: %d\n", d9p1)
	}

	{
		d9i, err := os.Open("day9/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d9i.Close()
		d9p2, err := day9.Part2(d9i)
		if err != nil {
			fmt.Println("Day 9 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 9 Part 2: %d\n", d9p2)
	}
}

func doDay10() {
	{
		d10i, err := os.Open("day10/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d10i.Close()
		d10p1, err := day10.Part1(d10i)
		if err != nil {
			fmt.Println("Day 10 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 10 Part 1: %d\n", d10p1)
	}

	{
		d10i, err := os.Open("day10/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d10i.Close()
		d10p2, err := day10.Part2(d10i)
		if err != nil {
			fmt.Println("Day 10 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 10 Part 2: %d\n", d10p2)
	}
}

func doDay11() {
	{
		d11i, err := os.Open("day11/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d11i.Close()
		d11p1, err := day11.Part1(d11i)
		if err != nil {
			fmt.Println("Day 11 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 11 Part 1: %d\n", d11p1)
	}

	{
		d11i, err := os.Open("day11/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d11i.Close()
		d11p2, err := day11.Part2(d11i)
		if err != nil {
			fmt.Println("Day 11 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 11 Part 2: %d\n", d11p2)
	}
}

func doDay12() {
	{
		d12i, err := os.Open("day12/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d12i.Close()
		d12p1, err := day12.Part1(d12i)
		if err != nil {
			fmt.Println("Day 12 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 12 Part 1: %d\n", d12p1)
	}

	{
		d12i, err := os.Open("day12/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d12i.Close()
		d12p2, err := day12.Part2(d12i)
		if err != nil {
			fmt.Println("Day 12 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 12 Part 2: %d\n", d12p2)
	}
}

func doDay13() {
	{
		d13i, err := os.Open("day13/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d13i.Close()
		d13p1, err := day13.Part1(d13i)
		if err != nil {
			fmt.Println("Day 13 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 13 Part 1: %d\n", d13p1)
	}

	{
		d13i, err := os.Open("day13/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d13i.Close()
		d13p2, err := day13.Part2(d13i)
		if err != nil {
			fmt.Println("Day 13 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 13 Part 2: \n")
		day13.PrintGrid(d13p2)
	}
}

func doDay14() {
	{
		d14i, err := os.Open("day14/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d14i.Close()
		d14p1, err := day14.Part1(d14i)
		if err != nil {
			fmt.Println("Day 14 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 14 Part 1: %d\n", d14p1)
	}

	{
		d14i, err := os.Open("day14/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d14i.Close()
		d14p2, err := day14.Part2(d14i)
		if err != nil {
			fmt.Println("Day 14 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 14 Part 2: %d\n", d14p2)
	}
}

func doDay15() {
	{
		d15i, err := os.Open("day15/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d15i.Close()
		d15p1, err := day15.Part1(d15i)
		if err != nil {
			fmt.Println("Day 15 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 15 Part 1: %d\n", d15p1)
	}

	{
		d15i, err := os.Open("day15/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d15i.Close()
		d15p2, err := day15.Part2(d15i)
		if err != nil {
			fmt.Println("Day 15 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 15 Part 2: %d\n", d15p2)
	}
}

func doDay16() {
	{
		d16i, err := os.Open("day16/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d16i.Close()
		d16p1, err := day16.Part1(d16i)
		if err != nil {
			fmt.Println("Day 16 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 16 Part 1: %d\n", d16p1)
	}

	{
		d16i, err := os.Open("day16/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d16i.Close()
		d16p2, err := day16.Part2(d16i)
		if err != nil {
			fmt.Println("Day 16 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 16 Part 2: %d\n", d16p2)
	}
}

func doDay17() {
	{
		d17i, err := os.Open("day17/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d17i.Close()
		d17p1, err := day17.Part1(d17i)
		if err != nil {
			fmt.Println("Day 17 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 17 Part 1: %d\n", d17p1)
	}

	{
		d17i, err := os.Open("day17/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d17i.Close()
		d17p2, err := day17.Part2(d17i)
		if err != nil {
			fmt.Println("Day 17 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 17 Part 2: %d\n", d17p2)
	}
}

func doDay18() {
	{
		d18i, err := os.Open("day18/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d18i.Close()
		d18p1, err := day18.Part1(d18i)
		if err != nil {
			fmt.Println("Day 18 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 18 Part 1: %d\n", d18p1)
	}

	{
		d18i, err := os.Open("day18/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d18i.Close()
		d18p2, err := day18.Part2(d18i)
		if err != nil {
			fmt.Println("Day 18 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 18 Part 2: %d\n", d18p2)
	}
}

func doDay19() {
	{
		d19i, err := os.Open("day19/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d19i.Close()
		d19p1, p2i, err := day19.Part1(d19i)
		if err != nil {
			fmt.Println("Day 18 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 18 Part 1: %d\n", d19p1)

		d19p2, err := day19.Part2(p2i)
		if err != nil {
			fmt.Println("Day 18 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 18 Part 2: %d\n", d19p2)
	}
}

func doDay20() {
	{
		d20i, err := os.Open("day20/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d20i.Close()
		d20p1, err := day20.Part1(d20i)
		if err != nil {
			fmt.Println("Day 20 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 20 Part 1: %d\n", d20p1)
	}

	{
		d20i, err := os.Open("day20/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d20i.Close()
		d20p2, err := day20.Part2(d20i)
		if err != nil {
			fmt.Println("Day 20 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 20 Part 2: %d\n", d20p2)
	}
}

func doDay21() {
	{
		d21i, err := os.Open("day21/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d21i.Close()
		d21p1, err := day21.Part1(d21i)
		if err != nil {
			fmt.Println("Day 21 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 21 Part 1: %d\n", d21p1)
	}

	{
		d21i, err := os.Open("day21/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d21i.Close()
		d21p2, err := day21.Part2(d21i)
		if err != nil {
			fmt.Println("Day 21 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 21 Part 2: %d\n", d21p2)
	}
}

func doDay22() {
	{
		d22i, err := os.Open("day22/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d22i.Close()
		d22p1, err := day22.Part1(d22i)
		if err != nil {
			fmt.Println("Day 22 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 22 Part 1: %d\n", d22p1)
	}

	{
		d22i, err := os.Open("day22/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d22i.Close()
		d22p2, err := day22.Part2(d22i)
		if err != nil {
			fmt.Println("Day 22 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 22 Part 2: %d\n", d22p2)
	}
}

func doDay23() {
	{
		d23i, err := os.Open("day23/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d23i.Close()
		d23p1, err := day23.Part1(d23i)
		if err != nil {
			fmt.Println("Day 23 Part 1 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 23 Part 1: %d\n", d23p1)
	}

	{
		d23i, err := os.Open("day23/input.txt")
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		defer d23i.Close()
		d23p2, err := day23.Part2(d23i)
		if err != nil {
			fmt.Println("Day 23 Part 2 had an error: ", err)
			os.Exit(1)
		}
		fmt.Printf("Day 23 Part 2: %d\n", d23p2)
	}
}

var includeSlow = flag.Bool("includeSlow", false, "Run slow days")

func main() {
	flag.Parse()
	doDay1()
	doDay2()
	doDay3()
	doDay4()
	doDay5()
	doDay6()
	doDay7()
	doDay8()
	doDay9()
	doDay10()
	doDay11()
	doDay12()
	doDay13()
	doDay14()
	doDay15()
	doDay16()
	doDay17()
	doDay18()
	if *includeSlow {
		doDay19()
	} else {
		fmt.Println("Skipping day19 as slow")
	}
	doDay20()
	doDay21()
	doDay22()
	doDay23()
}
