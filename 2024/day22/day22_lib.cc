#include "day22_lib.h"

#include <iostream>

#include <unordered_set>
#include <unordered_map>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start + len;

    std::vector<int64_t> secrets;

    auto p = start;
    while(p != end) {
        int64_t v = 0;
        unsigned char c;
        while((c=*p++) != '\n') {
            v *= 10;
            v += c-'0';
        }
        secrets.push_back(v);
    }

    return Input(secrets);
}

int64_t evolve(uint64_t seed, int iterations) {
    while(iterations--) {
        seed = (seed ^ (seed << 6)) & (0xFFFFFF);
        seed = (seed ^ (seed >> 5)) & (0xFFFFFF);
        seed = (seed ^ (seed << 11)) & (0xFFFFFF);
    }
    return seed;
}

int64_t Input::sum_evolved() const {
    int64_t result = 0;

    for (auto s: secrets) {
        result += evolve(s, 2000);
    }

    return result;
}


// If we care only about the low 10s digit, then how does that evolve?
// every bit can contribute to the low digit because 5 is coprime with 2
// the starting secret doesn't matter so much as finding which cycle it is part of.
// precompute all 24 bits and which cycle they are in?
// but just knowing the cycle we don't know what the best 4-sequence is
// since a sequence may appear multiple times and result in different values

uint64_t make_key(int d0, int d1, int d2, int d3) {
    return (d0 < 0 ? 1<<28 : 0) | (std::abs(d0) << 24)
        |  (d1 < 0 ? 1<<20 : 0) | (std::abs(d1) << 16)
        |  (d2 < 0 ? 1<<12 : 0) | (std::abs(d2) <<  8)
        |  (d3 < 0 ? 1<<4  : 0) | (std::abs(d3) <<  0);
}

// what if i just be dumb?
void find_sequences(int64_t seed, std::unordered_map<uint64_t, int64_t> &delta_results) {
    std::unordered_set<uint64_t> seen;

    int d0=-10, d1=-10, d2=-10, d3=-10;
    int steps = 2000;
    while (--steps != 0) {
        auto next = evolve(seed, 1);

        auto digit = next%10;
        d3 = d2;
        d2 = d1;
        d1 = d0;
        d0 = digit-(seed%10);

        if (d3 > -10) {
            auto key = make_key(d0, d1, d2, d3);
            if(!seen.contains(key)) {
                delta_results[key] += digit;
            }
            seen.insert(key);
        }
        
        seed = next;
    }
}

int64_t Input::buy_bananas() const {
    int64_t result = 0;

    std::unordered_map<uint64_t, int64_t> delta_results;
    for(auto seed: secrets) {
        //std::cout << "Computing deltas for " << seed << std::endl;
        find_sequences(seed, delta_results);
    }

    for (auto delta: delta_results) {
        //std::cout << std::hex << delta.first << std::dec << " = " << delta.second << std::endl;
        result = std::max(result, delta.second);
    }

    return result;
}