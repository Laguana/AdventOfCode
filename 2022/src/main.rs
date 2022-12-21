use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

fn main() {
    let now = Instant::now();
    let part1 = day1::part1();
    let duration = now.elapsed().as_micros();
    println!("Day1 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day1::part2();
    let duration = now.elapsed().as_micros();
    println!("Day1 Part 2 result in {}us: {}", duration, part2);
    
    let now = Instant::now();
    let part1 = day2::part1();
    let duration = now.elapsed().as_micros();
    println!("Day2 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day2::part2();
    let duration = now.elapsed().as_micros();
    println!("Day2 Part 2 result in {}us: {}", duration, part2);
    
    let now = Instant::now();
    let part1 = day3::part1();
    let duration = now.elapsed().as_micros();
    println!("Day3 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day3::part2();
    let duration = now.elapsed().as_micros();
    println!("Day3 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day4::part1();
    let duration = now.elapsed().as_micros();
    println!("Day4 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day4::part2();
    let duration = now.elapsed().as_micros();
    println!("Day4 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day5::part1();
    let duration = now.elapsed().as_micros();
    println!("Day5 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day5::part2();
    let duration = now.elapsed().as_micros();
    println!("Day5 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day6::part1();
    let duration = now.elapsed().as_micros();
    println!("Day6 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day6::part2();
    let duration = now.elapsed().as_micros();
    println!("Day6 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day7::part1();
    let duration = now.elapsed().as_micros();
    println!("Day7 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day7::part2();
    let duration = now.elapsed().as_micros();
    println!("Day7 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day8::part1();
    let duration = now.elapsed().as_micros();
    println!("Day8 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day8::part2();
    let duration = now.elapsed().as_micros();
    println!("Day8 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day9::part1();
    let duration = now.elapsed().as_micros();
    println!("Day9 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day9::part2();
    let duration = now.elapsed().as_micros();
    println!("Day9 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day10::part1();
    let duration = now.elapsed().as_micros();
    println!("Day10 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day10::part2();
    let duration = now.elapsed().as_micros();
    println!("Day10 Part 2 result in {}us: \n{}", duration, part2);

    let now = Instant::now();
    let part1 = day11::part1();
    let duration = now.elapsed().as_micros();
    println!("Day11 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day11::part2();
    let duration = now.elapsed().as_micros();
    println!("Day11 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day12::part1();
    let duration = now.elapsed().as_micros();
    println!("Day12 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day12::part2();
    let duration = now.elapsed().as_micros();
    println!("Day12 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day13::part1();
    let duration = now.elapsed().as_micros();
    println!("Day13 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day13::part2();
    let duration = now.elapsed().as_micros();
    println!("Day13 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day14::part1();
    let duration = now.elapsed().as_micros();
    println!("Day14 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day14::part2();
    let duration = now.elapsed().as_micros();
    println!("Day14 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day15::part1();
    let duration = now.elapsed().as_micros();
    println!("Day15 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day15::part2();
    let duration = now.elapsed().as_micros();
    println!("Day15 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day16::part1();
    let duration = now.elapsed().as_micros();
    println!("Day16 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day16::part2();
    let duration = now.elapsed().as_micros();
    println!("Day16 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day17::part1();
    let duration = now.elapsed().as_micros();
    println!("Day17 Part 1 result in {}us: {}", duration, part1);
    println!("Please hold for Day17 part 2... [~90s]");
    let now = Instant::now();
    let part2 = day17::part2();
    let duration = now.elapsed().as_micros();
    println!("Day17 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day18::part1();
    let duration = now.elapsed().as_micros();
    println!("Day18 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day18::part2();
    let duration = now.elapsed().as_micros();
    println!("Day18 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day19::part1();
    let duration = now.elapsed().as_micros();
    println!("Day19 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day19::part2();
    let duration = now.elapsed().as_micros();
    println!("Day19 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day20::part1();
    let duration = now.elapsed().as_micros();
    println!("Day20 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day20::part2();
    let duration = now.elapsed().as_micros();
    println!("Day20 Part 2 result in {}us: {}", duration, part2);

    let now = Instant::now();
    let part1 = day21::part1();
    let duration = now.elapsed().as_micros();
    println!("Day21 Part 1 result in {}us: {}", duration, part1);
    let now = Instant::now();
    let part2 = day21::part2();
    let duration = now.elapsed().as_micros();
    println!("Day21 Part 2 result in {}us: {}", duration, part2);
}
