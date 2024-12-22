#include "day22_lib.h"
#include "day22.in.h"

#include <iostream>

std::string example = 
"1\n"
"10\n"
"100\n"
"2024\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    auto result = parsed.sum_evolved();
    if (result != 37327623) {
        std::cout << "Expected 37327623 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto parsed = Input::parse(day22_day22_input, day22_day22_input_len);

    auto result = parsed.sum_evolved();
    if (result != 21147129593) {
        std::cout << "Expected 21147129593 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

std::string example2 =
"1\n"
"2\n"
"3\n"
"2024\n";

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example2.c_str(), example2.length());

    auto result = parsed.buy_bananas();
    if (result != 23) {
        std::cout << "Expected 23 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto parsed = Input::parse(day22_day22_input, day22_day22_input_len);

    auto result = parsed.buy_bananas();
    if (result != 2445) {
        std::cout << "Expected 2445 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();
    failures += part1_works();

    failures += example2_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}