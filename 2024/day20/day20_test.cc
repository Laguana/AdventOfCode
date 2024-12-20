#include "day20_lib.h"
#include "day20.in.h"

#include <iostream>
#include <string>

std::string example = 
"###############\n"
"#...#...#.....#\n"
"#.#.#.#.#.###.#\n"
"#S#...#.#.#...#\n"
"#######.#.#.###\n"
"#######.#.#...#\n"
"#######.#.###.#\n"
"###..E#...#...#\n"
"###.#######.###\n"
"#...###...#...#\n"
"#.#####.#.###.#\n"
"#.#...#.#.#...#\n"
"#.#.#.#.#.#.###\n"
"#...#...#...###\n"
"###############\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    auto result = parsed.count_big_skips(32);
    if (result != 4) {
        std::cout << "Expected 4 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto parsed = Input::parse(day20_day20_input, day20_day20_input_len);
    auto result = parsed.count_big_skips(100);
    if (result != 1518) {
        std::cout << "Expected 1518 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    auto result = parsed.count_bigger_skips(70);
    if (result != 41) {
        std::cout << "Expected 41 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto parsed = Input::parse(day20_day20_input, day20_day20_input_len);
    auto result = parsed.count_bigger_skips(100);
    if (result != 1032257) {
        std::cout << "Expected 1032257 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();

    failures += example_works();

    failures += part1_works();

    failures += example2_works();

    failures += part2_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}