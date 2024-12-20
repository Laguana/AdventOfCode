#include "day20_lib.h"
#include "day20.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day20_day20_input, day20_day20_input_len);

    std::cout << "Part 1: " << input.count_big_skips(100) << std::endl;
    std::cout << "Part 2: " << input.count_bigger_skips(100) << std::endl;
}