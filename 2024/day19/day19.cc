#include "day19_lib.h"
#include "day19.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day19_day19_input, day19_day19_input_len);

    std::cout << "Part 1: " << input.count_possible() << std::endl;
    std::cout << "Part 2: " << input.count_how_possible() << std::endl;
}