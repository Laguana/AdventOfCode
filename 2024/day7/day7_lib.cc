#include "day7_lib.h"

#include <iostream>

Input Input::parse(const unsigned char* start, const unsigned char*end){

    std::vector<std::vector<uint64_t>> rows;
    std::vector<uint64_t> row;

    const unsigned char* p = start;

    uint64_t t = 0;

    while (p != end) {
        auto c = *p++;
        if (c == ':' || c == ' ') {
            row.push_back(t);
            t = 0;
            if (c == ':') {
                p++;
            }
            continue;
        } else if (c == '\n') {
            row.push_back(t);
            t = 0;
            rows.push_back(row);
            row = std::vector<uint64_t>();
            continue;
        }

        t *= 10;
        t += c-'0';
    }

    return Input(rows);
}

bool can_sum(
    bool allow_concat,
    uint64_t goal,
    uint64_t acc,
    std::vector<uint64_t>::const_iterator it,
    std::vector<uint64_t>::const_iterator end) {
    if (it == end) {
        return acc == goal;
    }

    auto cur = *it++;
    if (can_sum(allow_concat, goal, acc + cur, it, end)) {
        return true;
    }
    if (can_sum(allow_concat, goal,acc*cur, it, end)) {
        return true;
    } if (allow_concat) {
        uint64_t t = cur;
        while (t > 0) {
            acc *= 10;
            t /= 10;
        }
        return can_sum(allow_concat, goal, acc + cur, it, end);
    }
    return false;
}

uint64_t Input::sum_workable(bool allow_concat) const {
    uint64_t result = 0;

    for (const auto &row : rows){
        auto it = row.cbegin();
        auto goal = *it++;
        auto first = *it++;
        if (can_sum(allow_concat, goal, first, it, row.cend())) {
            result += goal;
            //std::cout << "yes: " << goal << std::endl;
        } else {
            //std::cout << "no:  " << goal << std::endl;
        }
    }

    return result;
}