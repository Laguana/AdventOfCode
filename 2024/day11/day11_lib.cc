#include "day11_lib.h"

#include <unordered_map>
#include <tuple>

#include <iostream>

Input Input::parse(const unsigned char* start, std::size_t len) {
    const unsigned char* end = start+len;

    std::vector<uint64_t> stones;

    uint64_t temp = 0;

    auto *p = start;
    while(p != end) {
        auto c = *p++;
        if (c == ' ' || c == '\n') {
            stones.push_back(temp);
            temp = 0;
        } else {
            temp *= 10;
            temp += c-'0';
        }
    }

    if (temp != 0) {
        stones.push_back(temp);
    }

    return Input(stones);
}

int count_digits(uint64_t v) {
    if (v < 10) {
        return 1;
    } else if (v < 100) {
        return 2;
    } else if (v < 1000) {
        return 3;
    } else if (v < 10000) {
        return 4;
    } else if (v < 100000) {
        return 5;
    } else if (v < 1000000) {
        return 6;
    } else if (v < 10000000) {
        return 7;
    } else if (v < 100000000) {
        return 8;
    } else if (v < 1000000000) {
        return 9;
    } else if (v < 10000000000) {
        return 10;
    } else if (v < 100000000000) {
        return 11;
    } else if (v < 1000000000000) {
        return 12;
    } else if (v < 10000000000000) {
        return 13;
    } else if (v < 100000000000000) {
        return 14;
    } else if (v < 1000000000000000) {
        return 15;
    } else if (v < 10000000000000000) {
        return 16;
    } else if (v < 100000000000000000) {
        return 17;
    } else if (v < 1000000000000000000) {
        return 18;
    } else if (v < 10000000000000000000u) {
        return 19;
    } else {
        return 20;
    }
}

uint64_t digits_mask(int d) {
    switch(d) {
    case 1:  return 10;
    case 2:  return 100;
    case 3:  return 1000;
    case 4:  return 10000;
    case 5:  return 100000;
    case 6:  return 1000000;
    case 7:  return 10000000;
    case 8:  return 100000000;
    case 9:  return 1000000000;
    case 10: return 10000000000;
    default:
        abort();
    }
}

uint64_t Input::count_stones(int steps) const {
    std::vector<uint64_t> current = stones;
    std::vector<uint64_t> next;

    while(steps--) {
        for(auto e : current) {
            if (e == 0) {
                next.push_back(1);
                continue;
            }
            auto digits = count_digits(e);
            if (digits %2 == 1) {
                next.push_back(e*2024);
                continue;
            }
            auto mask = digits_mask(digits/2);
            auto left = e / mask;
            auto right = e % mask;
            next.push_back(left);
            next.push_back(right);
        }

        current = next;
        next.clear();

        //std::cout << steps << ":" << current.size() << std::endl;
    }

    return current.size();
}

template<>
struct std::hash<std::tuple<uint64_t, int>>
{
    std::size_t operator()(const std::tuple<uint64_t, int>& v) const noexcept
    {
        uint64_t l;
        int r;
        std::tie(l,r) = v;
        std::size_t h1 = std::hash<uint64_t>{}(l);
        std::size_t h2 = std::hash<int>{}(r);
        return h1 ^ (h2 << 1);
    }
};

uint64_t count_stones_single(
    std::unordered_map<std::tuple<uint64_t, int>,uint64_t> &memoized,
    uint64_t e,
    int n) {
    if (n == 0) {
        return 1;
    }
    auto key = std::tuple<uint64_t,int>(e,n);
    auto found = memoized.find(key);
    if (found != memoized.end()) {
        return (*found).second;
    }

    if (e == 0) {
        auto answer = count_stones_single(memoized, 1, n-1);
        memoized[key] = answer;
        return answer;
    }
    auto digits = count_digits(e);
    if (digits%2 == 1) {
        auto answer = count_stones_single(memoized, e*2024, n-1);
        memoized[key] = answer;
        return answer;
    }

    auto mask = digits_mask(digits/2);
    auto left = e / mask;
    auto right = e % mask;

    auto answer = count_stones_single(memoized, left, n-1) +
                  count_stones_single(memoized, right, n-1);
    memoized[key] = answer;
    return answer;
}

uint64_t Input::count_stones_better(int steps) const {
    // so we can't just do the naive thing.
    // All stones operate in isolation, so we could
    // do things one input stone at a time, but we need to find a pattern
    
    // We need to work out when things will split, i.e. when they get even digits
    // and what they split to, so we can work out when they split

    // There are only 3 rules, and only 2 really matter
    // even digits split, adding 1 to the number of stones
    //    -- almost want to work in log space, but modulo and floor don't play nice
    // non-even digits multiply by 2024, which may or may not make the digits even
    //    -- this is adding log10(2024) in log space,
    //       and if floor(log10(n)) is odd then n has even digits

    // Maybe this is the wrong approach and instead we
    // should look at memoizing or dynamic programming, but
    // I'm not sure what the thing to look at is.
    // perhaps something like
    // f(v, 0) = 1
    // f(0, n) = f(1, n-1)
    // f(v, n) = f(vl, n-1) + f(vr, n-1) if v even digits; vl = v/10^(d/2), vr = v%10^(d/2)
    // f(v, n) = f(2024*v, n-1) otherwise
    // can memoize that and see how it goes?

    std::unordered_map<std::tuple<uint64_t, int>,uint64_t> memoized {};
    uint64_t result = 0;
    for(auto e: stones) {
        result += count_stones_single(memoized, e, steps);
    }

    return result;
}