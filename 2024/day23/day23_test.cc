#include "day23_lib.h"
#include "day23.in.h"

#include <iostream>

std::string example = 
"kh-tc\n"
"qp-kh\n"
"de-cg\n"
"ka-co\n"
"yn-aq\n"
"qp-ub\n"
"cg-tb\n"
"vc-aq\n"
"tb-ka\n"
"wh-tc\n"
"yn-cg\n"
"kh-ub\n"
"ta-co\n"
"de-co\n"
"tc-td\n"
"tb-wq\n"
"wh-td\n"
"ta-ka\n"
"td-qp\n"
"aq-cg\n"
"wq-ub\n"
"ub-vc\n"
"de-ta\n"
"wq-aq\n"
"wq-vc\n"
"wh-yn\n"
"ka-de\n"
"kh-ta\n"
"co-tc\n"
"wh-qp\n"
"tb-vc\n"
"td-yn\n";

int parsing_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    return 0;
}

int example_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    auto result = parsed.count_t_cliques();
    if (result != 7) {
        std::cout << "Expected 7 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int part1_works() {
    auto parsed = Input::parse(day23_day23_input, day23_day23_input_len);
    auto result = parsed.count_t_cliques();
    if (result != 1284) {
        std::cout << "Expected 1284 but got " << result << std::endl;
        return 1;
    }
    return 0;
}

int example2_works() {
    auto parsed = Input::parse((const unsigned char*) example.c_str(), example.length());
    auto result = parsed.get_biggest_connected_component();
    if (result.size() != 4) {
        std::cout << "Should have got " << std::hex << ('c'<<8|'o') <<","<< ('d'<<8|'e')<<","<< ('k'<<8|'a')<<","<< ('t'<<8|'a') << std::dec << std::endl;
        std::cout << "Expected size 4 but got size " << result.size() << std::endl;
        return 1;
    }
    return 0;
}

int part2_works() {
    auto parsed = Input::parse(day23_day23_input, day23_day23_input_len);
    auto result = parsed.get_password();
    if (result != "bv,cm,dk,em,gs,jv,ml,oy,qj,ri,uo,xk,yw") {
        std::cout << "Expected bv,cm,dk,em,gs,jv,ml,oy,qj,ri,uo,xk,yw but got " << result << std::endl;
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

    if (failures >0) {
        std::cout << "Encountered " << failures << " failures!" <<std::endl;
    } else {
        std::cout << "All tests passed" << std::endl;
    }
    return failures;
}