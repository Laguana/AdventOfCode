#include "day1_lib.h"

#include <algorithm>
#include <cmath>

#include <iostream>

std::tuple<std::vector<int>, std::vector<int>> parse_input(std::basic_istream<char> &input) {
    std::vector<int> left, right;

    while(input)
    {
        int l=-1, r=-1;
        input >> l >> r;
        left.push_back(l);
        right.push_back(r);
        if (input.peek() == '\r') {
            input.get();
        }
        if (input.peek() == '\n') {
            input.get();
        }
        if (input.peek() == -1) {
            break;
        };
    }

    return std::make_tuple(left,right);
}

void sort_list(std::vector<int> &v) {
    std::sort(v.begin(), v.end());
}

uint64_t compute_distance(std::vector<int> &left, std::vector<int> &right) {
    sort_list(left);
    sort_list(right);

    uint64_t distance = 0;

    int i = 0;

    for(auto il = left.cbegin(), ir = right.cbegin(); il != left.cend() && ir != right.cend(); ++il, ++ir, ++i) {
        distance += std::abs(*il-*ir);
        //std::cout << *il << "-" << *ir << "=" << std::abs(*il-*ir) << " -- " << distance << ": " << i <<  std::endl;
    }

    return distance;
}

uint64_t compute_similarity(std::vector<int> &left, std::vector<int> &right) {
    sort_list(left);
    sort_list(right);

    uint64_t similarity = 0;

    auto il = left.cbegin();
    auto ir = right.cbegin();
    while (il != left.cend() && ir != right.cend()) {
        // We want to find left things in the right, so the left will drive our search
        while (*il < *ir) {
            //std::cout << *il << " < " << *ir << std::endl;
            ++il;
        }
        int count = 0;
        if (*il != *ir) {
            ++ir;
            continue;
        }
        while (ir != right.cend() && *il == *ir) {
            //std::cout << *il << " = " << *ir << std::endl;
            ++ir;
            ++count;
        }
        int element_similarity = *il * count;
        //std::cout << *il << " ~ " << element_similarity << std::endl;
        int e = *il;
        while(il != left.cend() && *il == e) {
            similarity += element_similarity;
            ++il;
        }
    }

    return similarity;
}