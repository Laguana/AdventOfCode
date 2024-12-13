#include "day13_lib.h"
#include "day13.in.h"

#include <iostream>

std::string example = 
"Button A: X+94, Y+34\n"
"Button B: X+22, Y+67\n"
"Prize: X=8400, Y=5400\n"
"\n"
"Button A: X+26, Y+66\n"
"Button B: X+67, Y+21\n"
"Prize: X=12748, Y=12176\n"
"\n"
"Button A: X+17, Y+86\n"
"Button B: X+84, Y+37\n"
"Prize: X=7870, Y=6450\n"
"\n"
"Button A: X+69, Y+23\n"
"Button B: X+27, Y+71\n"
"Prize: X=18641, Y=10279\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    auto result = parsed.solve_tokens();
    if (result != 480) {
        std::cout << "Expected 480, got " << result << std::endl;
    }
    return 0;
}

int part1_works() {
    auto parsed = Input::parse(day13_day13_input, day13_day13_input_len);
    auto result = parsed.solve_tokens();
    if (result != 37128) {
        std::cout << "Expected 37128, got " << result << std::endl;
    }
    return 0;
}

int part2_works() {
    auto parsed = Input::parse(day13_day13_input, day13_day13_input_len);
    auto result = parsed.solve_tokens(true);
    if (result != 74914228471331) {
        std::cout << "Expected 74914228471331, got " << result << std::endl;
    }
    return 0;
}

int main() {
    int failures = 0 ;

    failures += parsing_works();

    failures += example_works();

    failures += part1_works();
    failures += part2_works();
    

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}