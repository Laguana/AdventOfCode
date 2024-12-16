#include "day16_lib.h"

#include <iostream>
#include <queue>
#include <unordered_map>
#include <cmath>
#include <functional>
#include <utility>
#include <unordered_set>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start+len;

    std::vector<std::vector<bool>> walls;
    Point startpoint {-1,-1};
    Point endpoint {-1,-1};

    std::vector<bool> row;

    auto p = start;
    while(p != end) {
        auto c = *p++;
        if (c == '\n') {
            walls.push_back(row);
            row.clear();
        } else {
            if (c == 'S') {
                startpoint.x = row.size();
                startpoint.y = walls.size();
                row.push_back(false);
            } else if (c == 'E') {
                endpoint.x = row.size();
                endpoint.y = walls.size();
                row.push_back(false);
            } else {
                row.push_back(c == '#');
            }
        }
    }

    return Input(walls, startpoint, endpoint);
}

enum Facing {
    Up,
    Down,
    Left,
    Right
};

Point delta(Facing f) {
    switch (f) {
    case Facing::Up:
        return {0,-1};
    case Facing::Down:
        return {0,1};
    case Facing::Right:
        return {1,0};
    case Facing::Left:
        return {-1,0};
    default:
        std::unreachable();
    }
}

Facing turnLeft(Facing f) {
    switch (f) {
    case Facing::Up:
        return Facing::Left;
    case Facing::Down:
        return Facing::Right;
    case Facing::Right:
        return Facing::Up;
    case Facing::Left:
        return Facing::Down;
    default:
        std::unreachable();
    }
}
Facing turnRight(Facing f) {
    switch (f) {
    case Facing::Up:
        return Facing::Right;
    case Facing::Down:
        return Facing::Left;
    case Facing::Right:
        return Facing::Down;
    case Facing::Left:
        return Facing::Up;
    default:
        std::unreachable();
    }
}

struct SearchNode {
    Point p;
    Facing f;

    SearchNode(Point p, Facing f): p(p), f(f) {};
    SearchNode(): p({-1,-1}), f(Facing::Right) {};

    bool operator==(const SearchNode& other) const {
        return other.p == p && other.f == f;
    }
};

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

template<>
struct std::hash<SearchNode>
{
    std::size_t operator()(const SearchNode& s) const noexcept
    {
        std::size_t h1 = std::hash<Point>{}(s.p);
        std::size_t h2 = std::hash<int>{}(s.f);
        return h1 ^ (h2 << 2);
    }
};

uint64_t Input::shortest_path() const {
    std::unordered_map<SearchNode, uint64_t> g_score;
    std::unordered_map<SearchNode, uint64_t> f_score;
    std::unordered_map<SearchNode, SearchNode> from;

    auto comparator = [&f_score](const SearchNode &a, const SearchNode &b) -> bool { return f_score[a] > f_score[b];};

    std::priority_queue<SearchNode, std::vector<SearchNode>, decltype(comparator)> open_set(comparator);

    auto h = [this](const SearchNode &n) -> uint64_t {
        return std::abs(n.p.x - end.x) + std::abs(n.p.y - end.y);
    };

    {
        SearchNode startNode(start, Facing::Right);
        g_score[startNode] = 0;
        f_score[startNode] = h(startNode);
        open_set.push(startNode);
    }
    
    while (!open_set.empty()) {
        auto e = open_set.top();
        open_set.pop();
        if (e.p == end) {
            {
                auto p = e;
                while (from.contains(p)) {
                    //std::cout << "*(" << p.p.x << "," << p.p.y << ":" << "^v<>"[p.f] << ")" << std::endl;
                    p = from.find(p)->second;
                }
            }
            return f_score[e];
        }

        auto g = g_score[e];

        //std::cout << "(" << e.p.x << "," << e.p.y << ":" << "^v<>"[e.f] << ") = " << g << std::endl;

        {
            SearchNode straight(e.p + delta(e.f), e.f);
            if (!walls[straight.p.y][straight.p.x]) {
                auto found = g_score.find(straight);
                if (found == g_score.end() || found->second > g+1) {
                    g_score[straight] = g+1;
                    f_score[straight] = g+1 + h(straight);
                    from[straight] = e;
                    open_set.push(straight);
                }
            }
        }

        {
            SearchNode left(e.p, turnLeft(e.f));
            auto found = g_score.find(left);
            if (found == g_score.end() || found->second > g+1000) {
                g_score[left] = g+1000;
                f_score[left] = g+1000 + h(left);
                from[left] = e;
                open_set.push(left);
            }
        }
        {
            SearchNode right(e.p, turnRight(e.f));
            auto found = g_score.find(right);
            if (found == g_score.end() || found->second > g+1000) {
                g_score[right] = g+1000;
                f_score[right] = g+1000 + h(right);
                from[right] = e;
                open_set.push(right);
            }
        }
    }
    return -1;
}

void add_locations(
    std::unordered_set<SearchNode> &on_shortest_path,
    const std::unordered_map<SearchNode, std::unordered_set<SearchNode>> &from,
    const SearchNode &n) {
    if (!on_shortest_path.insert(n).second) {
        return;
    }

    auto found = from.find(n);
    if (found == from.end()) {
        return;
    }

    for (auto e: found->second) {
        add_locations(on_shortest_path, from, e);
    }
}

uint64_t Input::shortest_paths() const {
    std::unordered_map<SearchNode, uint64_t> g_score;
    std::unordered_map<SearchNode, uint64_t> f_score;
    std::unordered_map<SearchNode, std::unordered_set<SearchNode>> from;

    std::unordered_set<SearchNode> on_shortest_path;

    uint64_t shortest_path = std::numeric_limits<uint64_t>::max();

    auto comparator = [&f_score](const SearchNode &a, const SearchNode &b) -> bool { return f_score[a] > f_score[b];};

    std::priority_queue<SearchNode, std::vector<SearchNode>, decltype(comparator)> open_set(comparator);

    auto h = [this](const SearchNode &n) -> uint64_t {
        return std::abs(n.p.x - end.x) + std::abs(n.p.y - end.y);
    };

    {
        SearchNode startNode(start, Facing::Right);
        g_score[startNode] = 0;
        f_score[startNode] = h(startNode);
        open_set.push(startNode);
        on_shortest_path.insert(startNode);
    }
    
    while (!open_set.empty()) {
        auto e = open_set.top();
        open_set.pop();
        if (e.p == end) {
            add_locations(on_shortest_path, from, e);
            shortest_path = f_score[e];
        }

        auto g = g_score[e];

        if (g > shortest_path) {
            continue;
        }

        //std::cout << "(" << e.p.x << "," << e.p.y << ":" << "^v<>"[e.f] << ") = " << g << std::endl;

        {
            SearchNode straight(e.p + delta(e.f), e.f);
            if (!walls[straight.p.y][straight.p.x]) {
                auto found = g_score.find(straight);
                if (found == g_score.end() || found->second > g+1) {
                    g_score[straight] = g+1;
                    f_score[straight] = g+1 + h(straight);
                    from[straight] = std::unordered_set<SearchNode> {e};
                    open_set.push(straight);
                } else if (found->second == g+1) {
                from[straight].insert(e);
            }
            }
        }

        {
            SearchNode left(e.p, turnLeft(e.f));
            auto found = g_score.find(left);
            if (found == g_score.end() || found->second > g+1000) {
                g_score[left] = g+1000;
                f_score[left] = g+1000 + h(left);
                from[left] = std::unordered_set<SearchNode> {e};
                open_set.push(left);
            } else if (found->second == g+1000) {
                from[left].insert(e);
            }
        }
        {
            SearchNode right(e.p, turnRight(e.f));
            auto found = g_score.find(right);
            if (found == g_score.end() || found->second > g+1000) {
                g_score[right] = g+1000;
                f_score[right] = g+1000 + h(right);
                from[right] = std::unordered_set<SearchNode> {e};
                open_set.push(right);
            } else if (found->second == g+1000) {
                from[right].insert(e);
            }
        }
    }

    std::unordered_set<Point> locations;
    for(auto e: on_shortest_path) {
        locations.insert(e.p);
    }
    return locations.size();
}