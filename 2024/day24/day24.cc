#include "day24_lib.h"
#include "day24.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day24_day24_input, day24_day24_input_len);

    std::cout << "Part 1: " << input.get_z_number() << std::endl;
    std::cout << "Part 2: " << 0 << std::endl;
    return 0;
}