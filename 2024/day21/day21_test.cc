#include "day21_lib.h"
#include "day21.in.h"

#include <iostream>

std::string example = 
"029A\n"
"980A\n"
"179A\n"
"456A\n"
"379A\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.size());

    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*)example.c_str(), example.size());
    auto result = parsed.score_codes();
    if (result != 126384) {
        std::cout << "Expected 126384 but got " << result << std::endl;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();

    failures += example_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}