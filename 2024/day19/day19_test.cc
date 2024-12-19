#include "day19_lib.h"
#include "day19.in.h"

#include <iostream>

std::string example = 
"r, wr, b, g, bwu, rb, gb, br\n"
"\n"
"brwrr\n"
"bggr\n"
"gbbr\n"
"rrbgbr\n"
"ubwu\n"
"bwurrg\n"
"brgr\n"
"bbrgwb\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.size());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.size());
    auto result = parsed.count_possible();
    if (result != 6) {
        std::cout << "Expected 6 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day19_day19_input, day19_day19_input_len);
    auto result = input.count_possible();
    if (result != 311) {
        std::cout << "Expected 311 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.size());
    auto result = parsed.count_how_possible();
    if (result != 16) {
        std::cout << "Expected 16 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
auto input = Input::parse(day19_day19_input, day19_day19_input_len);
    auto result = input.count_how_possible();
    if (result != 616234236468263) {
        std::cout << "Expected 616234236468263 but got " << result << std::endl;
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