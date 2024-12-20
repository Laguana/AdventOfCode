#include "day20_lib.h"

#include <iostream>
#include <string>

std::string example = "";

int parsing_works() {
    auto result = Input::parse((const unsigned char*) example.c_str(), example.length());

    return 0;
}

int main() {
    int failures = 0;

    failures += parsing_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}