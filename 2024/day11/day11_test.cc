#include "day11_lib.h"
#include "day11.in.h"

#include <iostream>

const std::string example = "125 17\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char *)example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char *)example.c_str(), example.length());

    auto result = parsed.count_stones(25);
    if (result != 55312) {
        std::cout << "Expected 55312 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day11_day11_input, day11_day11_input_len);
    
    auto result = input.count_stones(25);
    if (result != 193269) {
        std::cout << "Expected 193269 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char *)example.c_str(), example.length());

    auto result = parsed.count_stones_better(25);
    if (result != 55312) {
        std::cout << "Expected 55312 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day11_day11_input, day11_day11_input_len);
    
    auto result = input.count_stones_better(75);
    if (result != 228449040027793) {
        std::cout << "Expected 228449040027793 but got " << result << std::endl;
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