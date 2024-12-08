#include "day8_lib.h"
#include "day8.in.h"

#include <iostream>

std::string example =
"............\n"
"........0...\n"
".....0......\n"
".......0....\n"
"....0.......\n"
"......A.....\n"
"............\n"
"............\n"
"........A...\n"
".........A..\n"
"............\n"
"............\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), (const unsigned char*) example.c_str() + example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), (const unsigned char*) example.c_str() + example.length());

    auto result = parsed.count_anodes();
    if (result != 14) {
        std::cout << "Expected 14 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day8_day8_input, day8_day8_input + day8_day8_input_len);
    
    auto result = input.count_anodes();
    if (result != 354) {
        std::cout << "Expected 354 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), (const unsigned char*) example.c_str() + example.length());

    auto result = parsed.count_anodes(true);
    if (result != 34) {
        std::cout << "Expected 34 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day8_day8_input, day8_day8_input + day8_day8_input_len);
    
    auto result = input.count_anodes(true);
    if (result != 1263) {
        std::cout << "Expected 1263 but got " << result << std::endl;
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
        std::cout << "Encountered " << failures << " errors!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}