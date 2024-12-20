#include "day18_lib.h"

#include <iostream>
#include <unordered_set>
#include <unordered_map>
#include <queue>

#include <utility>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start + len;
    std::vector<Point> blocks;

    auto p = start;
    while(p != end) {
        int x = 0;
        unsigned char c;
        while ((c = *p++) != ',') {
            x *= 10;
            x += c-'0';
        }
        int y = 0;
        while(p != end && (c = *p++) != '\n') {
            y *= 10;
            y += c-'0';
        }
        blocks.emplace_back(x,y);
    }
    return Input(blocks);
}

template<>
struct std::hash<Point>
{
    std::size_t operator()(const Point& s) const noexcept
    {
        std::size_t h1 = std::hash<int>{}(s.x);
        std::size_t h2 = std::hash<int>{}(s.y);
        return h1 ^ (h2 << 8);
    }
};

uint64_t Input::shortest_path(int size, unsigned int horizon) const {
    std::unordered_set<Point> blocks;

    for(auto i = 0UL; i < this->blocks.size() && i < horizon; ++i) {
        blocks.insert(this->blocks[i]);
    }

    std::unordered_map<Point, uint64_t> g_score;
    std::unordered_map<Point, uint64_t> f_score;

    auto comparator = [&f_score](const Point &a, const Point &b) -> bool { return f_score[a] > f_score[b];};

    const std::vector<Point> deltas = {{-1,0}, {1,0}, {0,-1}, {0,1}};

    std::priority_queue<Point, std::vector<Point>, decltype(comparator)> open_set(comparator);

    auto h = [this, size](const Point &p) -> uint64_t {
        return std::abs(p.x - size) + std::abs(p.y - size);
    };

    {
        g_score[{0,0}] = 0;
        f_score[{0,0}] = h({0,0});
        open_set.emplace(0,0);
    }
    

    while(!open_set.empty()) {
        auto e = open_set.top();
        open_set.pop();
        if (e.x == size && e.y == size) {
            return f_score[e];
        }

        auto g = g_score[e];

        for (const auto delta: deltas) {
            auto next = e+delta;
            if (next.x < 0 || next.y < 0 || next.x > size || next.y > size) {
                continue;
            }

            if (blocks.contains(next)) {
                continue;
            }

            auto found = g_score.find(next);
            if (found == g_score.end() || found->second > g+1) {
                g_score[next] = g+1;
                f_score[next] = g+1 + h(next);
                open_set.push(next);
            }
        }
    }

    return 0;
}

Point Input::find_cutoff(int size) const {
    std::size_t hi = blocks.size(), lo = 0;

    while(hi > lo+1) {
        auto mid = (hi+lo)/2;
        if (shortest_path(size, mid) == 0) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    return blocks[hi-1];

    /*
    for(unsigned int i = 1; i < blocks.size(); ++i) {
        if (shortest_path(size, i) == 0) {
            return blocks[i-1];
        }
    }
    std::unreachable();
    */
}