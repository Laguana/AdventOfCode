#include "day22_lib.h"
#include "day22.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day22_day22_input, day22_day22_input_len);

    std::cout << "Part 1: " << input.sum_evolved() << std::endl;
    std::cout << "Part 2: " << input.buy_bananas() << std::endl;
}