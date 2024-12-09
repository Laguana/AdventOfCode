#include "day9_lib.h"
#include "day9.in.h"

#include <iostream>

int main() {
    Input input = Input::parse(day9_day9_input, day9_day9_input + day9_day9_input_len);

    std::cout << "Part 1: " << input.defrag_and_checksum() << std::endl;

    std::cout << "Part 2: " << input.defrag_blocks_and_checksum() << std::endl;
}