#include "day6_lib.h"
#include "day6.in.h"

#include <iostream>

const std::string example =
"....#.....\n"
".........#\n"
"..........\n"
"..#.......\n"
".......#..\n"
"..........\n"
".#..^.....\n"
"........#.\n"
"#.........\n"
"......#...\n";

int parsing_works() {
    int errors = 0;

    Input input = Input::parse((unsigned char *)example.c_str(), (unsigned char *)example.c_str() + example.length());

    return errors;
}

int example_works() {
    Input input = Input::parse((unsigned char *)example.c_str(), (unsigned char *)example.c_str() + example.length());

    auto result = input.count_steps();
    if (result != 41) {
        std::cout << "Expected 41 positions, got " << result << std::endl;
        return 1;
    } else {
        return 0;
    }
}

int part1_works() {
    auto input = Input::parse(day6_day6_input, day6_day6_input + day6_day6_input_len);
    auto result = input.count_steps();
    if (result != 5239) {
        std::cout << "Expected 5239, got " << result << std::endl;
        return 1;
    } else {
        return 0;
    }
}

int example2_works() {
Input input = Input::parse((unsigned char *)example.c_str(), (unsigned char *)example.c_str() + example.length());

    auto result = input.count_opportunities();
    if (result != 6) {
        std::cout << "Expected 6 positions, got " << result << std::endl;
        return 1;
    } else {
        return 0;
    }

}

int part2_works() {
    auto input = Input::parse(day6_day6_input, day6_day6_input + day6_day6_input_len);
    auto result = input.count_opportunities();
    if (result != 1753) {
        std::cout << "Expected 1753, got " << result << std::endl;
        return 1;
    } else {
        return 0;
    }
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