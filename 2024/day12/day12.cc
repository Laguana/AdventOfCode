#include "day12_lib.h"
#include "day12.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day12_day12_input, day12_day12_input_len);

    std::cout << "Part 1: " << input.cost_field() << std::endl;
    std::cout << "Part 2: " << 0 << std::endl;
}