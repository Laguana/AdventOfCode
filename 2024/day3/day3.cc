#include "day3_lib.h"
#include "day3.in.h"

#include <sstream>
#include <iostream>

int main() {
    const char* input = (const char*)day3_day3_input;
    const char* end = input + day3_day3_input_len;

    auto result = sum_products(input, end);
    std::cout << "Part 1: " << result << std::endl;

    result = sum_enabled_products(input, end);
    std::cout << "Part 2: " << result << std::endl;

    return 0;
}