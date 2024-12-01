#include "day1_lib.h"
#include "day1.in.h"

#include <sstream>
#include <iostream>

const std::string example_input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";

int example_parses() {
    std::istringstream istream(example_input);
    std::vector<int> left, right;
    std::tie(left, right) = parse_input(istream);

    int errors = 0;

    std::vector<int> expected_left{ 3, 4, 2, 1, 3, 3},
                     expected_right{ 4, 3, 5, 3, 9, 3};

    if (left.size() != expected_left.size()) {
        std::cout << "Expected left to be " << expected_left.size() << " but got " << left.size() << " elements" << std::endl;
        errors++;
    }
    if (right.size() != expected_right.size()) {
        std::cout << "Expected right to be " << expected_right.size() << " but got " << right.size() << " elements" << std::endl;
        errors++;
    }

    for(auto it = left.cbegin(), eit = expected_left.cbegin(); it != left.end() && eit != expected_left.cend(); it++, eit++) {
        if (*it != *eit) {
            std::cout << "Mismatch in left vector: " << *it << " != " << *eit << std::endl;
            errors++;
        }
    }

    for(auto it = right.cbegin(), eit = expected_right.cbegin(); it != right.end(); it++, eit++) {
        if (*it != *eit) {
            std::cout << "Mismatch in right vector: " << *it << " != " << *eit << std::endl;
            errors++;
        }
    }

    return errors;
}

int list_sorts() {
    std::vector<int> v{ 3, 4, 2, 1, 3, 3};
    std::vector<int> expected{ 1, 2, 3, 3, 3, 4};
    sort_list(v);

    int errors = 0;

    for(auto it = v.cbegin(), eit = expected.cbegin(); it != v.end(); it++, eit++) {
        if (*it != *eit) {
            std::cout << "Mismatch in sorted vector: " << *it << " != " << *eit << std::endl;
            errors++;
        }
    }

    return errors;
}

int part1_example_works() {
    std::istringstream istream(example_input);
    std::vector<int> left, right;
    std::tie(left, right) = parse_input(istream);

    int distance = compute_distance(left, right);

    if (distance != 11) {
        std::cout << "Expected example distance to be 11, got " << distance << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    uint64_t expected_result = 1258579;

    std::string input = std::string(day1_day1_input, day1_day1_input + day1_day1_input_len);
    std::stringstream input_stream(input);
    std::vector<int> left, right;
    std::tie(left, right) = parse_input(input_stream);

    uint64_t result = compute_distance(left, right);

    if (result != expected_result) {
        std::cout << "Part1 Expected " << expected_result << " but got " << result << std::endl;
        return 1;
    }

    return 0;
}

int part2_example_works() {
    std::istringstream istream(example_input);
    std::vector<int> left, right;
    std::tie(left, right) = parse_input(istream);

    uint64_t similarity = compute_similarity(left, right);

    if (similarity != 31) {
        std::cout << "Expected example similarity to be 31, got " << similarity << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    uint64_t expected_result = 23981443;

    std::string input = std::string(day1_day1_input, day1_day1_input + day1_day1_input_len);
    std::stringstream input_stream(input);
    std::vector<int> left, right;
    std::tie(left, right) = parse_input(input_stream);

    uint64_t result = compute_similarity(left, right);

    if (result != expected_result) {
        std::cout << "Part2 Expected " << expected_result << " but got " << result << std::endl;
        return 1;
    }

    return 0;
}

int main() {

    int failures = 0;

    failures += example_parses();
    failures += list_sorts();
    failures += part1_example_works();
    failures += part1_works();

    failures += part2_example_works();
    failures += part2_works();

    if (failures == 0)
    {
        std::cout << "All tests passed" << std::endl;
    }
    else
    {
        std::cout << "There were " << failures << " failures!" << std::endl;
    }
    return failures;
}