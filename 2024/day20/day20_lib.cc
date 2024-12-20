#include "day20_lib.h"

#include <iostream>

#include <utility>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start + len;

    Point startPoint{-1,-1}, endPoint {-1,-1};
    std::vector<std::vector<bool>> walls;

    std::vector<bool> row;

    auto p = start;
    while(p != end) {
        while (*p != '\n') {
            switch(*p++) {
            case '#':
                row.push_back(true);
                break;
            case '.':
                row.push_back(false);
                break;
            case 'S':
                startPoint.x = row.size();
                startPoint.y = walls.size();
                row.push_back(false);
                break;
            case 'E':
                endPoint.x = row.size();
                endPoint.y = walls.size();
                row.push_back(false);
                break;
            default:
                std::unreachable();
            }
        }
        walls.push_back(row);
        row.clear();
        p++;
    }

    std::unordered_map<Point, uint64_t> distances;
    distances[endPoint] = 0;

    const std::vector<Point> deltas = {{-1,0},{1,0},{0,-1},{0,1}};

    
    uint64_t distance = 0;
    Point pt = endPoint;
    Point pp = {-1,-1};
    do {
        Point next;
        for(auto &d:deltas) {
            next = pt+d;
            if (next != pp && !walls[next.y][next.x]) {
                //std::cout << next.x << "," << next.y << " != " << pp.x << "," << pp.y << std::endl;
                break;
            }
        }
        distances[next] = ++distance; 
        pp = pt;
        pt = next;
    } while(pt != startPoint);
    
    
    return Input(startPoint, endPoint, walls, distances);
}

void print_map(const std::vector<std::vector<bool>>& walls) {
    for(auto &row: walls) {
        for(auto e: row) {
            std::cout << (e?'#':'.');
        }
        std::cout << std::endl;
    }
}

uint64_t Input::count_big_skips(unsigned int threshold) const {
    uint64_t ret = 0;

    const std::vector<Point> two_step = {
        {-2,0}, {-1,-1}, {-1,1}, {2,0}, {1,-1}, {1,1},
        {0,-2}, {0,2}};

    for(auto &p: distances) {
        if (p.second < threshold) {
            // can't skip big if we aren't big
            continue;
        }
        for (auto &d: two_step) {
            // what if we stepped from p to p+d?
            Point dest = p.first + d;
            if (!distances.contains(dest)) {
                // not on the track
                continue;
            }
            auto d2 = distances.at(dest);
            if (p.second < d2) {
                // went backwards
                continue;
            }
            // we gained distance, but spent 2 steps
            auto gain = p.second-d2-2;
            if (gain >= threshold) {
                ++ret;
            }
        }
        
    }

    return ret;
}

uint64_t Input::count_bigger_skips(unsigned int threshold) const {
    uint64_t ret = 0;

    for(auto &p: distances) {
        if (p.second < threshold) {
            // can't skip big if we aren't big
            continue;
        }
        //std::cout << p.first.x << "," << p.first.y << " is " << p.second << std::endl;
        for(int x = -20; x <= 20; ++x) {
            //std::cout << "  " << x << std::endl;
            for(int y = -20; y <= 20; ++y) {
                if (std::abs(x) + std::abs(y) >20) {
                    continue;
                }
                //std::cout << "    " << y << std::endl;
                Point d ={x,y};
                // what if we stepped from p to p+d?
                Point dest = p.first + d;
                if (!distances.contains(dest)) {
                    // not on the track
                    continue;
                }
                auto d2 = distances.at(dest);
                if (p.second < d2) {
                    // went backwards
                    //std::cout << " " << dest.x << "," << dest.y << " backwards" <<  std::endl;
                    continue;
                }
                // we gained distance, but spent |x|+|y| steps
                auto gain = p.second-d2-std::abs(x)-std::abs(y);
                if (gain >= threshold) {
                    //std::cout << "Gained " << gain << std::endl;
                    ++ret;
                } else {
                    //std::cout << " " << dest.x << "," << dest.y << " only gained " << gain << std::endl;
                }
            }
        }
        
    }

    return ret;
}