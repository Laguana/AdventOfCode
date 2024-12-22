#include "day22_lib.h"
#include "day22.in.h"

#include <iostream>

std::string example = "";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());

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