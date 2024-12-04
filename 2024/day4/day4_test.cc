#include "day4_lib.h"
#include "day4.in.h"

#include <print>

std::string example = "MMMSXXMASM\n"
"MSAMXMSMSA\n"
"AMXSXMAAMM\n"
"MSAMASMSMX\n"
"XMASAMXAMM\n"
"XXAMMXXAMA\n"
"SMSMSASXSS\n"
"SAXAMASAAA\n"
"MAMMMXMMMM\n"
"MXMXAXMASX\n";

int parsing_works() {
    int errors = 0;

    Input input = Input::parse(example.c_str(), example.c_str() + example.length());

    if (input.get(0,0) != 'M') {
        std::print("Expected 0,0 to be M, got {0}\n", input.get(0,0));
        ++errors;
    }
    if (input.get(1,0) != 'M') {
        std::print("Expected 1,0 to be M, got {0}\n", input.get(1,0));
        ++errors;
    }
    if (input.get(1,1) != 'S') {
        std::print("Expected 1,1 to be S, got {0}\n", input.get(1,1));
        ++errors;
    }
    if (input.get(9,9) != 'X') {
        std::print("Expected 9,9 to be S, got {0}\n", input.get(9,9));
        ++errors;
    }
    if (input.get(10,10) != 0) {
        std::print("Expected 10,10 to be \\0, got {0}\n", input.get(10,10));
        ++errors;
    }

    return errors;
}

int example_works() {
    Input input = Input::parse(example.c_str(), example.c_str() + example.length());
    if (input.count_xmas() != 18) {
        std::print("Expected 18 XMAS but got {0}\n", input.count_xmas());
        return 1;
    }
    return 0;
}

int part1_works() {
    Input input = Input::parse((const char*)day4_day4_input, (const char*)day4_day4_input + day4_day4_input_len);

    int result = input.count_xmas();
    int expected = 2575;
    if (result != expected) {
        std::print("Expected part1 to be {0} but got {1}", expected, result);
        return 1;
    }
    return 0;
    
}

int part2_works() {
    Input input = Input::parse((const char*)day4_day4_input, (const char*)day4_day4_input + day4_day4_input_len);

    int result = input.count_x_mas();

    int expected = 2041;
        if (result != expected) {
        std::print("Expected part1 to be {0} but got {1}", expected, result);
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;
    
    failures += parsing_works();
    failures += example_works();

    failures += part1_works();
    failures += part2_works();

    if (failures > 0) {
        std::print("Encountered {0} failures\n", failures);
    } else {
        std::print("All tests passed\n");
    }

    return failures;
}