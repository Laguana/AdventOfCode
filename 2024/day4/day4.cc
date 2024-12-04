#include "day4_lib.h"
#include "day4.in.h"

#include <print>

int main() {
    Input input = Input::parse((const char*)day4_day4_input, (const char*)day4_day4_input + day4_day4_input_len);

    std::print("Part 1: {0}\n", input.count_xmas());

    std::print("Part 2: {0}\n", input.count_x_mas());

    return 0;
}