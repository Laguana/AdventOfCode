#include "day21_lib.h"
#include "day21.in.h"

#include <iostream>

std::string example = 
"029A\n"
"980A\n"
"179A\n"
"456A\n"
"379A\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.size());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.size());
    auto result = parsed.score_codes();
    if (result != 126384) {
        std::cout << "Expected 126384 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto parsed = Input::parse(day21_day21_input, day21_day21_input_len);
    auto result = parsed.score_codes();
    if (result != 248684) {
        std::cout << "Expected 248684 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example_works_for_part2() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.size());
    auto result = parsed.score_codes2(2);
    if (result != 126384) {
        std::cout << "Expected 126384 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works_for_part_1() {
    auto parsed = Input::parse(day21_day21_input, day21_day21_input_len);
    auto result = parsed.score_codes2(2);
    if (result != 248684) {
        std::cout << "Expected 248684 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example_depth_4_works() {
    auto parsed = Input::parse((const unsigned char*) "026A\n", 5);
    auto result = parsed.score_codes2(4);
    if (result != 402 * 26) {
        std::cout << "Expected " << (402 * 26) << " but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto parsed = Input::parse(day21_day21_input, day21_day21_input_len);
    auto result = parsed.score_codes2(25);
    if (result != 307055584161760) {
        std::cout << "Expected 307055584161760 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();

    failures += example_works();

    failures += part1_works();
    failures += example_works_for_part2();

    failures += part2_works_for_part_1();

    failures += example_depth_4_works();

    failures += part2_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}