#include "day5_lib.h"
#include "day5.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day5_day5_input, day5_day5_input + day5_day5_input_len);
    
    std::cout << "Part 1: " << input.get_ordered_midpoints() << std::endl;

    std::cout << "Part 2: " << input.get_reordered_midpoints() << std::endl;
}