#include "day10_lib.h"
#include "day10.in.h"

#include <iostream>
int main() {
    auto input = Input::parse(day10_day10_input, day10_day10_input + day10_day10_input_len);

    std::cout << "Part 1: " << input.score_trailheads() << std::endl; 
    std::cout << "Part 2: " << input.rate_trailheads() << std::endl;
    return 0;
}