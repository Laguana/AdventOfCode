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
std::unordered_map<uint64_t, int> find_sequences(int64_t seed) {
    std::unordered_set<int64_t> seen;
    std::unordered_map<uint64_t, int> delta_results;

    int d0=-10, d1=-10, d2=-10, d3=-10;
    int steps = 2000;
    while (!seen.contains(seed) && --steps != 0) {
        seen.insert(seed);
        auto next = evolve(seed, 1);

        auto digit = next%10;
        d3 = d2;
        d2 = d1;
        d1 = d0;
        d0 = digit-(seed%10);

        if (d3 > -10) {
            auto key = make_key(d0, d1, d2, d3);
            if(!delta_results.contains(key)) {
                if (d1 < -5 && d2 < -5) {
                    std::cout << seed << " -> " << next << std::endl;
                }
                delta_results[key] = digit;
            }
        }
        
        seed = next;
    }

    return delta_results;
}

int64_t Input::buy_bananas() const {
    int64_t result = 0;

    std::vector<std::unordered_map<uint64_t, int>> delta_results;
    for(auto seed: secrets) {
        //std::cout << "Computing deltas for " << seed << std::endl;
        delta_results.push_back(find_sequences(seed));
    }

    for(int d1 = -9; d1 < 10; ++d1) {
        for (int d2 = -9; d2 < 10; ++d2) {
            for (int d3 = -9; d3 < 10; ++d3) {
                for (int d4 = -9; d4 < 100; ++d4) {
                    int64_t sum = 0;
                    auto key = make_key(d1, d2, d3, d4);
                    for(const auto &m: delta_results ) {
                        auto it = m.find(key);
                        if (it != m.end()) {
                            sum += it->second;
                        }
                    }
                    if (sum > result) {
                        std::cout << d1 << "," << d2 << "," << d3 << "," << d4 << "~" << std::hex << key << std::dec << " new best " << sum << std::endl;
                    }
                    result = std::max(result, sum);
                }
            }
        }
    }

    return result;
}