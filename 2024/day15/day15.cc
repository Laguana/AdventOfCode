#include "day15_lib.h"
#include "day15.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day15_day15_input, day15_day15_input_len);

    std::cout << "Part 1: " << input.run_instructions_and_score() << std::endl;
    std::cout << "Part 2: " << input.run_instructions_on_double_wide() << std::endl;
}