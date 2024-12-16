#include "day16_lib.h"
#include "day16.in.h"

#include <iostream>

std::string example = 
"#################\n"
"#...#...#...#..E#\n"
"#.#.#.#.#.#.#.#.#\n"
"#.#.#.#...#...#.#\n"
"#.#.#.#.###.#.#.#\n"
"#...#.#.#.....#.#\n"
"#.#.#.#.#.#####.#\n"
"#.#...#.#.#.....#\n"
"#.#.#####.#.###.#\n"
"#.#.#.......#...#\n"
"#.#.###.#####.###\n"
"#.#.#...#.....#.#\n"
"#.#.#.#####.###.#\n"
"#.#.#.........#.#\n"
"#.#.#.#########.#\n"
"#S#.............#\n"
"#################\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    return 0;
}

int example1_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    auto result = parsed.shortest_path();
    if (result != 11048) {
        std::cout << "Expected 11048 but got " << result << std::endl;
        return -1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day16_day16_input, day16_day16_input_len);

    auto result = input.shortest_path();
    if (result != 143580) {
        std::cout << "Expected 143580 but got " << result << std::endl;
        return -1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    auto result = parsed.shortest_paths();
    if (result != 64) {
        std::cout << "Expected 64 but got " << result << std::endl;
        return -1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();

    failures += example1_works();

    failures += part1_works();

    failures += example2_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}