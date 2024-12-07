#include "day7_lib.h"
#include "day7.in.h"

#include <iostream>

int main () {
    auto input = Input::parse(day7_day7_input, day7_day7_input + day7_day7_input_len);

    std::cout << "Part 1: " << input.sum_workable() << std::endl;

    std::cout << "Part 2: " << input.sum_workable(true) << std::endl;
}