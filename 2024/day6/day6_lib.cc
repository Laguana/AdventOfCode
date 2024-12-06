#include "day6_lib.h"

#include <unordered_set>
#include <utility>
#include <tuple>

#include <iostream>

Direction clockwise_direction(Direction d) {
    switch (d) {
    case Direction::Up:
        return Direction::Right;
    case Direction::Down:
        return  Direction::Left;
    case Direction::Left:
        return  Direction::Up;
    case Direction::Right:
        return Direction::Down;
    default:
        std::unreachable();
    }
}

Point Point::next(Direction d) const {
    switch (d) {
    case Direction::Up:
        return Point(x,y-1);
    case Direction::Down:
        return Point(x,y+1);
    case Direction::Left:
        return Point(x-1,y);
    case Direction::Right:
        return Point(x+1,y);
    default:
        std::unreachable();
    }
}

Input Input::parse(const unsigned char* start, const unsigned char* end) {
    Point guard(-1,-1);
    std::vector<Point> obstacles;

    int x = 0, y = 0;
    int width = -1;
    Direction dir = Direction::Up;

    const unsigned char* p = start;

    while (p != end) {
        unsigned char c = *p++;
        switch(c) {
        case '\n':
            if (width == -1) {
                width = x;
            }
            x = -1;
            ++y;
            break;
        case '#':
            obstacles.emplace_back(x,y);
            break;
        case '^':
            guard = Point(x,y);
            break;
        default:
            break;
        }
        ++x;
    }

    return Input(obstacles, guard, dir, width, y);
}

int Input::count_steps() const {
    std::unordered_set<Point> steps { guard};
    std::unordered_set<Point> obstacles(this->obstacles.cbegin(), this->obstacles.cend());

    Point p = guard;
    Direction d = dir;

    do {
        Point next = p.next(d);
        if (obstacles.contains(next)) {
            d = clockwise_direction(d);
            continue;
        }
        if (next.x < 0 || next.x >= width || next.y < 0 || next.y >= height) {
            break;
        }
        steps.insert(next);
        p = next;
    } while(1);

    return steps.size();
}

template<>
struct std::hash<std::tuple<Point,Direction>>
{
    std::size_t operator()(const std::tuple<Point,Direction>& s) const noexcept
    {
        Point p;
        Direction d;
        std::tie(p,d) = s;
        std::size_t h1 = std::hash<Point>{}(p);
        std::size_t h2 = std::hash<Direction>{}(d);
        return h1 ^ (h2 << 1);
    }
};

void print_map(const std::unordered_set<std::tuple<Point,Direction>> &steps, Point obstacle, int width, int height) {
    std::unordered_set<Point> s {};
    for(auto k : steps) {
        Point p;
        std::tie(p, std::ignore) = k;
        s.insert(p);
    }
    for (int y = 0; y < height; ++y) {
        for(int x = 0; x < width; ++x) {
            std::cout << (s.contains(Point(x,y)) ? '#' : (obstacle.x == x && obstacle.y == y) ? 'O' :' ');
        }
        std::cout << std::endl;
    }
}

bool Input::will_loop(const std::unordered_set<Point> &obstacles, Point additional_obstacle, Point start, Direction d) const {
    std::unordered_set<std::tuple<Point,Direction>> steps {};
    steps.emplace(start, d);
    Point p = start;
    do {
        Point next = p.next(d);
        auto pair = std::tuple<Point,Direction>(next,d);
        if (steps.contains(pair)) {
            //std::cout << "(" << additional_obstacle.x << "," << additional_obstacle.y << ")" << std::endl;
            //print_map(steps, additional_obstacle, width, height);
            return true;
        }
        if (next == additional_obstacle || obstacles.contains(next)) {
            d = clockwise_direction(d);
        } else {
            p = next;
        }
        steps.emplace(p,d);
    } while(p.x >= 0 && p.x < width && p.y >= 0 && p.y < height);
    return false;
}

int Input::count_opportunities() const {
    std::unordered_set<Point> steps { guard };
    std::unordered_set<Point> obstacles(this->obstacles.cbegin(), this->obstacles.cend());
    std::unordered_set<Point> opportunities { guard };

    Point p = guard;
    Direction d = dir;

    //steps.emplace(p,d);

    do {
        Point next = p.next(d);
        if (obstacles.contains(next)) {
            d = clockwise_direction(d);
            continue;
        } else if (!steps.contains(next)){
            if (will_loop(obstacles, next, p, d)) {
                opportunities.insert(next);
            }
        }
        p = next;
        steps.insert(p);
    } while(p.x >= 0 && p.x < width && p.y >= 0 && p.y < height);

    opportunities.erase(guard);
    return opportunities.size();
}