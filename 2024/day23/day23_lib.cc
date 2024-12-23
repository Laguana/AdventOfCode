#include "day23_lib.h"

#include <algorithm>

#include <iostream>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start+len;
    std::unordered_map<int, std::unordered_set<int>> connections;

    auto p = start;

    while(p != end) {
        int left = p[0] << 8 | p[1];
        int right = p[3] << 8 | p[4];
        connections[left].insert(right);
        connections[right].insert(left);
        p += 6;
    }

    return Input(connections);
}

int Input::count_t_cliques() const {
    int answer =0;
    for(auto e: connections) {
        auto p1 = e.first;
        auto &s1 = e.second;
        if ((p1 & 0xFF00) == ('t' << 8)) {
            for (auto p2: s1) {
                if ((p2 & 0xFF00) == ('t' << 8) && p2 <= p1) {
                    // avoid double counting
                    continue;
                }
                auto &s2 = connections.at(p2);
                for(auto p3: s2) {
                    if ((p3 & 0xFF00) == ('t' << 8) && p3 <= p1) {
                        continue;
                    }
                    if (p3 <= p2) {
                        continue;
                    }
                    if (s1.contains(p3)) {
                        // p1 p2 p3 is a clique of 3 where WLOG p1 starts with t,
                        // p2 < p3, and if p2 or p3 start with t then they are
                        // strictly greater than p1, so this uniquely identifies them
                        ++answer;
                    }
                }
            }
        }
    }

    return answer;
}

// wiki go https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm

void Input::BronKerbosh(std::unordered_set<int> & result, std::unordered_set<int> & R, std::unordered_set<int> &P, std::unordered_set<int> & X) const {
    if (P.empty() && X.empty()) {
        if (R.size() > result.size()) {
            result = R;
        }
        return;
    } else if (P.empty()) {
        std::cout << "Empty, backtracking" << std::endl;
        return;
    }

    auto pivot = (*P.begin());
    auto pivot_neighbors = connections.at(pivot);

    for(auto v: P) {
        if (pivot_neighbors.contains(v)) {
            continue;
        }
        std::unordered_set<int> P2(P);
        auto vn = connections.at(v);
        for(auto it = P2.begin(); it != P2.end();) {
            if (!vn.contains(*it)) {
                it = P2.erase(it);
            } else {
                ++it;
            }
        }
        if (result.size() > R.size() + P2.size()+1) {
            // found a better candidate
            continue;
        }
        std::unordered_set<int> X2(X);
        for(auto it = X2.begin(); it != X2.end();) {
            if (!vn.contains(*it)) {
                it = X2.erase(it);
            } else {
                ++it;
            }
        }
        R.insert(v);
        BronKerbosh(result, R, P2, X2);
        R.erase(v);
    }
}

std::unordered_set<int> Input::get_biggest_connected_component() const {
    std::unordered_set<int> result;
    std::unordered_set<int> R, P, X;
    for(auto v: connections) {
        P.insert(v.first);
    }

    BronKerbosh(result, R, P, X);

    return result;
}

std::string Input::get_password() const {
    auto component = get_biggest_connected_component();
    std::vector<int> ordered(component.begin(), component.end());
    std::sort(ordered.begin(), ordered.end());
    std::string result = "";
    for(auto e: ordered) {
        if (result.length() > 0) {
            result += ",";
        }
        result += (e>>8);
        result += e&0xff;
    }
    return result;
}