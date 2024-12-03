#include "day3_lib.h"
#include "day3.in.h"

#include <iostream>
#include <sstream>

int parsing_works() {
    std::string good = "mul(3,999)";
    const char * p = good.c_str();
    const char * pend = p + good.length();
    auto result = Instruction::parse(&p, pend);

    int errors = 0;

    if (!result.has_value()) {
        std::cout << "Expected to parse successfully, got null" << std::endl;
        ++errors;
    } else if (result.value().product() != 3*999) {
        std::cout << "Expected product to be " << (3 * 999) << " but got " << result.value().product() << std::endl;
        ++errors;
    } if (p != pend) {
        std::cout << "Expected the whole string to be parsed, left with '" << p << "'" << std::endl;
        ++errors;
    }

    std::string mixed = " mul(2,3)mul(5,7mul(11,13) ";
    p = mixed.c_str();
    pend = p+mixed.length();

    result = Instruction::parse(&p, pend);
    if (result.has_value()) {
        std::cout << "Should not have had a value, but did" << std::endl;
        ++errors;
    }

    result = Instruction::parse(&p, pend);
    if (!result.has_value()) {
        std::cout << "Expected to parse successfully, got null" << std::endl;
        ++errors;
    } else if (result.value().product() != 2*3) {
        std::cout << "Expected product to be " << 2*3 << " but got " << result.value().product() << std::endl;
        ++errors;
    }

    result = Instruction::parse(&p, pend);
    if (result.has_value()) {
        std::cout << "Should not have had a value, but did" << std::endl;
        ++errors;
    }

    result = Instruction::parse(&p, pend);
    if (!result.has_value()) {
        std::cout << "Expected to parse successfully, got null" << std::endl;
        ++errors;
    } else if (result.value().product() != 11*13) {
        std::cout << "Expected product to be " << 11*13 << " but got " << result.value().product() << std::endl;
        ++errors;
    }

    if (p == pend) {
        std::cout << "Did not expect to have read to the end yet" << std::endl;
        ++errors;
    }
    result = Instruction::parse(&p, pend);
    if (result.has_value()) {
        std::cout << "Should not have had a value, but did" << std::endl;
        ++errors;
    }
    if (p != pend) {
        std::cout << "Should have reached the end now, but did not" << std::endl;
        ++errors;
    }

    std::string dodont = "do()mul(2,3)don't()mul(5,7)";
    p = dodont.c_str();
    pend = p+dodont.length();

    result = Instruction::parse(&p, pend);
    if (!result.has_value()) {
        std::cout << "Should have had a value but did not" << std::endl;
        ++errors;
    } else if (result.value().get_kind() != Instruction::Kind::Do) {
        std::cout << "Expected Do, got " << result.value().get_kind() << std::endl;
        ++errors;
    }
    result = Instruction::parse(&p, pend);
    if (!result.has_value()) {
        std::cout << "Should have had a value but did not" << std::endl;
        ++errors;
    } else if (result.value().get_kind() != Instruction::Kind::Mul) {
        std::cout << "Expected Mul, got " << result.value().get_kind() << std::endl;
        ++errors;
    }
    result = Instruction::parse(&p, pend);
    if (!result.has_value()) {
        std::cout << "Should have had a value but did not" << std::endl;
        ++errors;
    } else if (result.value().get_kind() != Instruction::Kind::Dont) {
        std::cout << "Expected Dont, got " << result.value().get_kind() << std::endl;
        ++errors;
    }
    result = Instruction::parse(&p, pend);
    if (!result.has_value()) {
        std::cout << "Should have had a value but did not" << std::endl;
        ++errors;
    } else if (result.value().get_kind() != Instruction::Kind::Mul) {
        std::cout << "Expected Mul, got " << result.value().get_kind() << std::endl;
        ++errors;
    } 

    return errors;
}

int example_works() {
    std::string example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    auto result = sum_products(example.c_str(), example.c_str() + example.length());

    if (result != 161) {
        std::cout << "Expected 161, but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    const char* input = (const char*)day3_day3_input;
    const char* end = input + day3_day3_input_len;

    auto result = sum_products(input, end);
    if (result != 175700056) {
        std::cout << "Expected part1 to be 175700056 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    const char* input = (const char*)day3_day3_input;
    const char* end = input + day3_day3_input_len;

    auto result = sum_enabled_products(input, end);

    if (result != 71668682) {
        std::cout << "Expected part2 to be 71668682 but got " << result << std::endl;
        return 1;
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