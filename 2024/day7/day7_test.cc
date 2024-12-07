#include "day7_lib.h"
#include "day7.in.h"

#include <iostream>

std::string example = 
"190: 10 19\n"
"3267: 81 40 27\n"
"83: 17 5\n"
"156: 15 6\n"
"7290: 6 8 6 15\n"
"161011: 16 10 13\n"
"192: 17 8 14\n"
"21037: 9 7 18 13\n"
"292: 11 6 16 20\n";

int parsing_works() {
    auto parsed = Input::parse((unsigned char*)example.c_str(), (unsigned char*)example.c_str()+example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((unsigned char*)example.c_str(), (unsigned char*)example.c_str()+example.length());

    auto result = parsed.sum_workable();
    if (result != 3749) {
        std::cout << "Expected 3749 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day7_day7_input, day7_day7_input + day7_day7_input_len);

    auto result = input.sum_workable();

    if (result != 1260333054159) {
        std::cout << "Expected 1260333054159 but got " << result << std::endl;
        return 1;
    }
    return 0;
    
}

int example2_works() {
    auto parsed = Input::parse((unsigned char*)example.c_str(), (unsigned char*)example.c_str()+example.length());

    auto result = parsed.sum_workable(true);
    if (result != 11387) {
        std::cout << "Expected 11387 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day7_day7_input, day7_day7_input + day7_day7_input_len);

    auto result = input.sum_workable(true);

    if (result != 162042343638683) {
        std::cout << "Expected 162042343638683 but got " << result << std::endl;
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
        std::cout << "Encountered " << failures << " test failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}