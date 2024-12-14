#include "day14_lib.h"
#include "day14.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day14_day14_input, day14_day14_input_len);

    std::cout << "Part 1: " << input.mult_quadrants(100,101,103) << std::endl;
    std::cout << "Part 2: " << std::endl;
    
    //input.execute_until_stop(101,103);

    // There are artifacts at 29 % 101 and 4 % 103
    // these will coincide at some t % 10403 such that t%101 = 29 and t%103 = 4
    // rather than think too hard, i just looked at all of them and 6493 was that t
    input.print_at(6493, 101, 103);
}