#include "day10_lib.h"
#include "day10.in.h"

#include <iostream>

std::string example = 
"89010123\n"
"78121874\n"
"87430965\n"
"96549874\n"
"45678903\n"
"32019012\n"
"01329801\n"
"10456732\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), (const unsigned char*)example.c_str() + example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), (const unsigned char*)example.c_str() + example.length());
    auto result = parsed.score_trailheads();
    if (result != 36) {
        std::cout << "Expected 36 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day10_day10_input, day10_day10_input + day10_day10_input_len);

    auto result = input.score_trailheads();
    if (result != 825) {
        std::cout << "Expected 825 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), (const unsigned char*)example.c_str() + example.length());
    auto result = parsed.rate_trailheads();
    if (result != 81) {
        std::cout << "Expected 81 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day10_day10_input, day10_day10_input + day10_day10_input_len);

    auto result = input.rate_trailheads();
    if (result != 1805) {
        std::cout << "Expected 1805 but got " << result << std::endl;
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