#include "day14_lib.h"
#include "day14.in.h"

#include <iostream>

std::string example = 
"p=0,4 v=3,-3\n"
"p=6,3 v=-1,-3\n"
"p=10,3 v=-1,2\n"
"p=2,0 v=2,-1\n"
"p=0,0 v=1,3\n"
"p=3,0 v=-2,-2\n"
"p=7,6 v=-1,-3\n"
"p=3,0 v=-1,-2\n"
"p=9,3 v=2,3\n"
"p=7,3 v=-1,2\n"
"p=2,4 v=2,-3\n"
"p=9,5 v=-3,-3\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

    auto result = parsed.mult_quadrants(100, 11, 7);
    if (result != 12) {
        std::cout << "Expected 12, got " << result << std::endl;
        return 1;
    }

    return 0;
}

int part1_works() {
    auto input = Input::parse(day14_day14_input,day14_day14_input_len);

    auto result = input.mult_quadrants(100, 11, 7);
    if (result != 228457125) {
        std::cout << "Expected 228457125, got " << result << std::endl;
        return 1;
    }

    return 0;
}



int main() {
    int failures = 0;

    failures += parsing_works();
    failures += example_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}