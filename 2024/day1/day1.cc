#include "day1_lib.h"
#include "day1.in.h"

#include <iostream>
#include <sstream>

#include <cstdio>

int main() {
    std::string input = std::string(day1_day1_input, day1_day1_input + day1_day1_input_len);
    std::stringstream input_stream(input);
    std::vector<int> left, right;
    std::tie(left, right) = parse_input(input_stream);

    uint64_t result = compute_distance(left, right);

    std::cout << "Part 1: " << result << std::endl;

    uint64_t part2 = compute_similarity(left, right);

    std::cout << "Part 2: " << part2 << std::endl;

    return 0;
}