#include "day8_lib.h"

#include <unordered_set>

#include <iostream>

Input Input::parse(const unsigned char * start, const unsigned char * end) {
    int width=-1;
    Input::AntennaMap map;

    int x = 0, y = 0;
    auto p = start;

    while(p != end) {
        auto c = *p++;
        if (c == '.') {
            ++x;
            continue;
        }
        if (c == '\n') {
            if (width == -1) {
                width = x;
            }
            x = 0;
            ++y;
            continue;
        }
        auto &entry = map[c];
        entry.emplace_back(x,y);

        ++x;
    }

    return Input(width, y, map);
}



void Input::add_anodes(bool pt2, std::unordered_set<Point> &set, cviter it, cviter end) const {
    if (it == end) {
        return;
    }
    Point cur = *it++;

    add_anodes(pt2, set, it, end);

    while(it != end) {
        Point other = *it++;
        auto dx = other.x - cur.x;
        auto dy = other.y - cur.y;

        if (!pt2) {
            auto x1 = cur.x - dx, y1 = cur.y-dy;
            auto x2 = other.x + dx, y2 = other.y + dy;

            if (x1 >= 0 && x1 < width && y1 >= 0 && y1 < width) {
                set.emplace(x1,y1);
            }

            if (x2 >= 0 && x2 < width && y2 >= 0 && y2 < width) {
                set.emplace(x2,y2);
            }
        } else {
            auto x1 = cur.x, y1 = cur.y;
            while (x1 >= 0 && x1 < width && y1 >= 0 && y1 < width) {
                set.emplace(x1,y1);
                x1 -= dx;
                y1 -= dy;
            }

            auto x2 = other.x, y2= other.y;
            while (x2 >= 0 && x2 < width && y2 >= 0 && y2 < width) {
                set.emplace(x2,y2);
                x2 += dx;
                y2 += dy;
            }
        }
    }
}

int Input::count_anodes(bool pt2) const {
    std::unordered_set<Point> anodes {};

    for (auto &e : antennae) {
        auto &points = e.second;
        add_anodes(pt2, anodes, points.cbegin(), points.cend());
    }

    return anodes.size();
}