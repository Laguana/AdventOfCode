#include "day2_lib.h"

#include "day2.in.h"

#include <sstream>
#include <iostream>

int main() {
    std::string input_string(day2_day2_input, day2_day2_input + day2_day2_input_len);
    std::stringstream input_stream(input_string);
    auto input = parse_input(input_stream);

    auto result = count_safe(input);
    std::cout << "Part 1: " << result << std::endl;

    result = count_damped_safe(input);
    std::cout << "Part 2: " << result << std::endl;

    return 0;
}