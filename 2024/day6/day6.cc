#include "day6_lib.h"
#include "day6.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day6_day6_input, day6_day6_input + day6_day6_input_len);

    std::cout << "Part 1: " << input.count_steps() << std::endl;

    std::cout << "Part 2: " << input.count_opportunities() << std::endl;

    return 0;
}