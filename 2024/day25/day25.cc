#include "day25_lib.h"
#include "day25.in.h"

#include <iostream>

int main() {
    auto parsed = Input::parse(day25_day25_input, day25_day25_input_len);

    std::cout << "Part 1: " << parsed.matching_combinations() << std::endl;

    return 0;
}