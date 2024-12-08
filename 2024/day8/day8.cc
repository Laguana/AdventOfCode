#include "day8_lib.h"
#include "day8.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day8_day8_input, day8_day8_input + day8_day8_input_len);

    std::cout << "Part 1: " << input.count_anodes() << std::endl;

    std::cout << "Part 2: " << input.count_anodes(true) << std::endl;

    return 0;
}