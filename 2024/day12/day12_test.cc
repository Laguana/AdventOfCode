#include "day12_lib.h"
#include "day12.in.h"

#include <iostream>

std::string example = 
"RRRRIICCFF\n"
"RRRRIICCCF\n"
"VVRRRCCFFF\n"
"VVRCCCJFFF\n"
"VVVVCJJCFE\n"
"VVIVCCJJEE\n"
"VVIIICJJEE\n"
"MIIIIIJJEE\n"
"MIIISIJEEE\n"
"MMMISSJEEE\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.length());

    auto result = parsed.cost_field();
    if (result != 1930) {
        std::cout << "Expected 1930 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day12_day12_input, day12_day12_input_len);

    auto result = input.cost_field();
    if (result != 1400386) {
        std::cout << "Expected 1400386 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.length());

    auto result = parsed.cost_field(true);
    if (result != 1206) {
        std::cout << "Expected 1206 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day12_day12_input, day12_day12_input_len);

    auto result = input.cost_field(true);
    if (result != 851994) {
        std::cout << "Expected 851994 but got " << result << std::endl;
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

    if (failures >0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}