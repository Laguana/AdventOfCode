#include "day18_lib.h"
#include "day18.in.h"

#include <iostream>

std::string example = 
"5,4\n"
"4,2\n"
"4,5\n"
"3,0\n"
"2,1\n"
"6,3\n"
"2,4\n"
"1,5\n"
"0,6\n"
"3,3\n"
"2,6\n"
"5,1\n"
"1,2\n"
"5,5\n"
"2,5\n"
"6,5\n"
"1,4\n"
"0,4\n"
"6,4\n"
"1,1\n"
"6,1\n"
"1,0\n"
"0,5\n"
"1,6\n"
"2,0\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.length());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.length());

    auto result = parsed.shortest_path(6, 12);
    if (result != 22) {
        std::cout << "Expected 22 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day18_day18_input, day18_day18_input_len);

    auto result = input.shortest_path(70,1024);
    if (result != 276) {
        std::cout << "Expected 276 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.length());

    auto result = parsed.find_cutoff(6);
    if (result != Point(6,1)) {
        std::cout << "Expected 6,1 but got " << result.x << "," << result.y << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day18_day18_input, day18_day18_input_len);

    auto result = input.find_cutoff(70);
    if (result != Point(60,37)) {
        std::cout << "Expected 60,37 but got " << result.x << "," << result.y << std::endl;
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

    if (failures > 0){ 
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}