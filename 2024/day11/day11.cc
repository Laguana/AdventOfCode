#include "day11_lib.h"
#include "day11.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day11_day11_input, day11_day11_input_len);

    std::cout << "Part 1: " << input.count_stones(25) << std::endl;
    std::cout << "Part 2: " << input.count_stones_better(75) << std::endl;

    return 0;
}