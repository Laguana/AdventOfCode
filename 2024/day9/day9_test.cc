#include "day9_lib.h"
#include "day9.in.h"

#include <iostream>

std::string example = "2333133121414131402\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), (const unsigned char*)example.c_str() + example.length());
    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), (const unsigned char*)example.c_str() + example.length());
    auto result = parsed.defrag_and_checksum();
    if (result != 1928) {
        std::cout << "Expected 1928 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto input = Input::parse(day9_day9_input, day9_day9_input + day9_day9_input_len);
    auto result = input.defrag_and_checksum();
    if (result != 6299243228569) {
        std::cout << "Expected 6299243228569 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), (const unsigned char*)example.c_str() + example.length());
    auto result = parsed.defrag_blocks_and_checksum();
    if (result != 2858) {
        std::cout << "Expected 2858 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto input = Input::parse(day9_day9_input, day9_day9_input + day9_day9_input_len);
    auto result = input.defrag_blocks_and_checksum();
    if (result != 6326952672104) {
        std::cout << "Expected 6326952672104 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();
    failures += example_works();

    failures += part1_works();

    failures += example2_works();

    failures += part2_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}