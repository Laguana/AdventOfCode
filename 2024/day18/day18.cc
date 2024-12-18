#include "day18_lib.h"
#include "day18.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day18_day18_input, day18_day18_input_len);

    std::cout << "Part 1: " << input.shortest_path(70,1024) << std::endl;
    auto first_unreachable = input.find_cutoff(70);
    std::cout << "Part 2: " << first_unreachable.x <<"," << first_unreachable.y << std::endl;
    return 0;
}