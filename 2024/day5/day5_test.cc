#include "day5_lib.h"
#include "day5.in.h"

#include <iostream>

const std::string example = 
"47|53\n"
"97|13\n"
"97|61\n"
"97|47\n"
"75|29\n"
"61|13\n"
"75|53\n"
"29|13\n"
"97|29\n"
"53|29\n"
"61|53\n"
"97|53\n"
"61|29\n"
"47|13\n"
"75|47\n"
"97|75\n"
"47|61\n"
"75|61\n"
"47|29\n"
"75|13\n"
"53|13\n"
"\n"
"75,47,61,53,29\n"
"97,61,53,29,13\n"
"75,29,13\n"
"75,97,47,61,53\n"
"61,13,29\n"
"97,13,75,29,47\n";

unsigned int parsing_works() {
    Input input = Input::parse((unsigned char*)example.c_str(), (unsigned char*)example.c_str() + example.length());

    return 0;
}

unsigned int example_works() {
    Input input = Input::parse((unsigned char*)example.c_str(), (unsigned char*)example.c_str() + example.length());

    int result = input.get_ordered_midpoints();
    if (result != 143) {
        std::cout << "Expected 143, got " << result << std::endl;
        return 1;
    }
    return 0;
}

unsigned int part1_works() {
    Input input = Input::parse(day5_day5_input, day5_day5_input + day5_day5_input_len);

    int result = input.get_ordered_midpoints();
    if (result != 5732) {
        std::cout << "Expected 5732, got " << result << std::endl;
        return 1;
    }
    return 0;
}

unsigned int example2_works() {
    Input input = Input::parse((unsigned char*)example.c_str(), (unsigned char*)example.c_str() + example.length());

    int result = input.get_reordered_midpoints();
    if (result != 123) {
        std::cout << "Expected 123, got " << result << std::endl;
        return 1;
    }
    return 0;
}

unsigned int part2_works() {
    Input input = Input::parse(day5_day5_input, day5_day5_input + day5_day5_input_len);

    int result = input.get_reordered_midpoints();
    if (result != 4716) {
        std::cout << "Expected 4716, got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    unsigned int failures = 0;

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
}