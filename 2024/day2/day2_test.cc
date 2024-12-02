#include "day2_lib.h"
#include "day2.in.h"

#include <sstream>
#include <iostream>

std::string example_input = 
"7 6 4 2 1\n"
"1 2 7 8 9\n"
"9 7 6 2 1\n"
"1 3 2 4 5\n"
"8 6 4 4 1\n"
"1 3 6 7 9";

int example_parses() {
    std::stringstream example_input_stream(example_input);
    auto parsed = parse_input(example_input_stream);

    int errors = 0;

    if (parsed.size() != 6) {
        std::cout << "Expected 6 rows, got " << parsed.size() << std::endl;
        errors++;
    }

    for(decltype(parsed)::size_type i = 0; i < parsed.size(); ++i) {
        if (parsed[i].size() != 5) {
            std::cout << "Expected row " << i << " to have 5 entries, got " << parsed[i].size() << std::endl;
            ++errors;
        }
    }

    return errors;
}

int safety_works() {
    std::stringstream example_input_stream(example_input);
    auto parsed = parse_input(example_input_stream);

    int errors = 0;

    if (!is_safe(parsed[0])) {
        std::cout << "expected first example to be safe" << std::endl;
        errors++;
    }
    if (is_safe(parsed[1])) {
        std::cout << "expected second example to be unsafe" << std::endl;
        errors++;
    }

    return errors;
}

int example_works() {
    std::stringstream example_input_stream(example_input);
    auto parsed = parse_input(example_input_stream);

    auto result = count_safe(parsed);
    if (result != 2) {
        std::cout << "Expected 2 safe, got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    std::string input_string(day2_day2_input, day2_day2_input + day2_day2_input_len);
    std::stringstream input_stream(input_string);
    auto input = parse_input(input_stream);

    auto result = count_safe(input);
    if (result != 510) {
        std::cout << "Expected part1 to be 510, got " << result << std::endl;
        return 1;
    }

    return 0;
}

int part2_example_works() {
    std::stringstream example_input_stream(example_input);
    auto parsed = parse_input(example_input_stream);

    auto result = count_damped_safe(parsed);
    if (result != 4) {
        std::cout << "Expected 4 safe, got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    std::string input_string(day2_day2_input, day2_day2_input + day2_day2_input_len);
    std::stringstream input_stream(input_string);
    auto input = parse_input(input_stream);

    auto result = count_damped_safe(input);
    if (result != 553) {
        std::cout << "Expected 553 damped safe, got " << result << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    int failures = 0;

    failures += example_parses();
    failures += safety_works();
    failures += example_works();

    failures += part1_works();

    failures += part2_example_works();
    failures += part2_works();

    if (failures > 0) {
        std::cout << "Encountered " << failures << " failures!" << std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }

    return failures;
}