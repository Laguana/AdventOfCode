#include "day13_lib.h"
#include "day13.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day13_day13_input, day13_day13_input_len);

    std::cout << "Part 1: " << input.solve_tokens() << std::endl;
    std::cout << "Part 2: " << input.solve_tokens(true) << std::endl;
}