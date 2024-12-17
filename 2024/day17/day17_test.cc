#include "day17_lib.h"
#include "day17.in.h"

#include <iostream>

std::string example = 
"Register A: 729\n"
"Register B: 0\n"
"Register C: 0\n"
"\n"
"Program: 0,1,5,4,3,0\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    auto result = parsed.run_machine();
    if (result != "4,6,3,5,6,3,5,2,1,0") {
        std::cout << "Expected 4,6,3,5,6,3,5,2,1,0 but got '" << result << "'" << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day17_day17_input, day17_day17_input_len);

    auto result = input.run_machine();
    if (result != "7,6,1,5,3,1,4,2,6") {
        std::cout << "Expected 7,6,1,5,3,1,4,2,6 but got '" << result << "'" << std::endl;
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