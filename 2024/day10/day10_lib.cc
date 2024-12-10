#include "day10_lib.h"

#include <unordered_set>
#include <queue>

#include <iostream>

Input Input::parse(const unsigned char* start, const unsigned char* end) {
    std::vector<std::vector<int>> map {};
    std::vector<Point> origins {};
    std::vector<Point> endpoints {};

    std::vector<int> row {};
    auto p = start;
    while(p != end) {
        auto c = *p++;
        if (c == '\n') {
            map.push_back(row);
            row = std::vector<int>();
        } else {
            if (c == '0') {
                origins.emplace_back(row.size(), map.size());
            } else if (c == '9') {
                endpoints.emplace_back(row.size(), map.size());
            }

            row.push_back(c-'0');
        }
    }

    if (row.size() != 0) {
        map.push_back(row);
    }

    return Input(map, origins, endpoints);
}

int Input::score_trailheads() const {
    int result = 0;

    std::unordered_set<Point> fringe(endpoints.cbegin(), endpoints.cend());

    std::vector<std::vector<std::unordered_set<Point>>> reachable = {};

    for(auto &v : map) {
        reachable.emplace_back(v.size());
    }



    while(!fringe.empty()) {
        auto e = fringe.extract(fringe.begin()).value();
        
        auto height = map[e.y][e.x];
        if (height == 9) {
            reachable[e.y][e.x].insert(e);
        } else if (height == 0) {
            continue;
        }

        auto &here_reachable = reachable[e.y][e.x];
        if (e.y > 0 && map[e.y-1][e.x] == height-1) {
            auto &there_reachable = reachable[e.y-1][e.x];
            auto size_before = there_reachable.size();
            there_reachable.insert(here_reachable.begin(), here_reachable.end());
            if (there_reachable.size() != size_before) {
                fringe.emplace(e.x,e.y-1);
            }
        }
        if ((std::size_t)e.y+1 < map.size() && map[e.y+1][e.x] == height-1) {
            auto &there_reachable = reachable[e.y+1][e.x];
            auto size_before = there_reachable.size();
            there_reachable.insert(here_reachable.begin(), here_reachable.end());
            if (there_reachable.size() != size_before) {
                fringe.emplace(e.x,e.y+1);
            }
        }
        if (e.x > 0 && map[e.y][e.x-1] == height-1) {
            auto &there_reachable = reachable[e.y][e.x-1];
            auto size_before = there_reachable.size();
            there_reachable.insert(here_reachable.begin(), here_reachable.end());
            if (there_reachable.size() != size_before) {
                fringe.emplace(e.x-1,e.y);
            }
        }
        if ((std::size_t)e.x+1 < map[0].size() && map[e.y][e.x+1] == height-1) {
            auto &there_reachable = reachable[e.y][e.x+1];
            auto size_before = there_reachable.size();
            there_reachable.insert(here_reachable.begin(), here_reachable.end());
            if (there_reachable.size() != size_before) {
                fringe.emplace(e.x+1,e.y);
            }
        }
    }

    for (auto &p : origins) {
        result += reachable[p.y][p.x].size();
    }

    return result;
}

int Input::rate_trailheads() const {
    int result = 0;

    std::queue<Point> fringe(endpoints.cbegin(), endpoints.cend());
    std::unordered_set<Point> seen = {};

    std::vector<std::vector<int>> score = {};

    for(auto &v : map) {
        score.emplace_back(v.size());
    }



    while(!fringe.empty()) {
        auto e = fringe.front();

        if (seen.contains(e)) {
            fringe.pop();
            continue;
        }
        seen.insert(e);

        auto height = map[e.y][e.x];
        if (height == 9) {
            score[e.y][e.x] = 1;
        } else if (height == 0) {
            fringe.pop();
            continue;
        }

        auto here_score = score[e.y][e.x];
        /*
        std::cout << "(" << e.x << "," << e.y << "):" << map[e.y][e.x]
            << " " << here_score
            << std::endl;
        */
       
        if (e.y > 0 && map[e.y-1][e.x] == height-1) {
            score[e.y-1][e.x] += here_score;
            fringe.emplace(e.x, e.y-1);
        }
        if ((std::size_t)e.y+1 < map.size() && map[e.y+1][e.x] == height-1) {
            score[e.y+1][e.x] += here_score;
            fringe.emplace(e.x,e.y+1);
        }
        if (e.x > 0 && map[e.y][e.x-1] == height-1) {
            score[e.y][e.x-1] += here_score;
            fringe.emplace(e.x-1,e.y);
        }
        if ((std::size_t)e.x+1 < map[0].size() && map[e.y][e.x+1] == height-1) {
            score[e.y][e.x+1] += here_score;
            fringe.emplace(e.x+1,e.y);
        }
        fringe.pop();
    }

    for (auto &p : origins) {
        result += score[p.y][p.x];
        //std::cout << p.x << " " << p.y << ": " << score[p.y][p.x] << std::endl;
    }

    return result;
}