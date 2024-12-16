#include "day16_lib.h"
#include "day16.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day16_day16_input, day16_day16_input_len);

    std::cout << "Part 1: " << input.shortest_path() << std::endl;
    std::cout << "Part 2: " << 0 << std::endl;

    return 0;
}