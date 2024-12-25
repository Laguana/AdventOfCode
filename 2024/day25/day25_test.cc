#include "day25_lib.h"
#include "day25.in.h"

#include <iostream>
#include <string>

std::string example =
"#####\n"
".####\n"
".####\n"
".####\n"
".#.#.\n"
".#...\n"
".....\n"
"\n"
"#####\n"
"##.##\n"
".#.##\n"
"...##\n"
"...#.\n"
"...#.\n"
".....\n"
"\n"
".....\n"
"#....\n"
"#....\n"
"#...#\n"
"#.#.#\n"
"#.###\n"
"#####\n"
"\n"
".....\n"
".....\n"
"#.#..\n"
"###..\n"
"###.#\n"
"###.#\n"
"#####\n"
"\n"
".....\n"
".....\n"
".....\n"
"#....\n"
"#.#..\n"
"#.#.#\n"
"#####\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    auto result = parsed.matching_combinations();
    if (result != 3) {
        std::cout << "Expected 3 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto parsed = Input::parse(day25_day25_input, day25_day25_input_len);
    auto result = parsed.matching_combinations();
    if (result != 2691) {
        std::cout << "Expected 2691 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();
    failures += example_works();

    failures += part1_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }
    return failures;
}