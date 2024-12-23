#include "day23_lib.h"
#include "day23.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day23_day23_input, day23_day23_input_len);

    std::cout << "Part 1: " << input.count_t_cliques() << std::endl;

    std::cout << "Part 2: " << input.get_password()<< std::endl;

    return 0;
}