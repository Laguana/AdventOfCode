#include "day21_lib.h"
#include "day21.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day21_day21_input, day21_day21_input_len);

    std::cout << "Part 1: " << input.score_codes() << std::endl;
    std::cout << "Part 2: " << 0 << std::endl;
}