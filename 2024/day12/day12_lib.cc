#include "day12_lib.h"
#include <queue>

#include <iostream>

Input Input::parse(const unsigned char* start, std::size_t len) {
    std::vector<std::vector<char>> grid;

    auto end = start + len;
    auto p = start;

    std::vector<char> row;

    while(p != end) {
        auto c = *p++;
        if (c == '\n') {
            grid.push_back(row);
            row.clear();
        } else {
            row.push_back(c);
        }
    }

    if (row.size() != 0) {
        grid.push_back(row);
    }

    return Input(grid);
}

bool in_bounds(int x, int y, int width, int height) {
    return x >= 0 && y >= 0 && x < width && y < height;
}

std::tuple<uint64_t, uint64_t, uint64_t> floodfill(const std::vector<std::vector<char>> & grid, int x, int y, int region, std::vector<std::vector<int>> &regions) {
    std::queue<std::tuple<int, int>> fringe;
    fringe.emplace(x,y);

    uint64_t perimeter = 0;
    uint64_t area = 0;
    uint64_t sides = 0;

    auto height = grid.size();
    auto width = grid[0].size();

    char crop = grid[y][x];

    while(!fringe.empty()) {
        std::tie(x,y) = fringe.front();
        fringe.pop();

        if (regions[y][x] != -1) {
            continue;
        }

        regions[y][x] = region;
        ++area;

        perimeter+= 4;


        //std::cout << "  (" << x << "," << y <<")" << std::endl;

        if (y > 0) {
            if (regions[y-1][x] == -1 && grid[y-1][x] == crop) {
                fringe.emplace(x,y-1);
            } else if (regions[y-1][x] == region) {
                perimeter-=2;
            }
        }
        if ((std::size_t)y+1 < height) {
            if (regions[y+1][x] == -1 && grid[y+1][x] == crop) {
                fringe.emplace(x,y+1);
            } else if (regions[y+1][x] == region) {
                perimeter-=2;
            }
        }
        if (x > 0) {
            if (regions[y][x-1] == -1 && grid[y][x-1] == crop) {
                fringe.emplace(x-1,y);
            } else if (regions[y][x-1] == region) {
                perimeter-=2;
            }
        }
        if ((std::size_t)x+1 < width) {
            if (regions[y][x+1] == -1 && grid[y][x+1] == crop) {
                fringe.emplace(x+1, y);
            } else if (regions[y][x+1] == region) {
                perimeter-=2;
            }
        }
    }

    return std::make_tuple(perimeter, area, sides);
}

uint64_t Input::cost_field() const {
    uint64_t result = 0;

    auto width = grid[0].size();

    int region = 0;
    std::vector<std::vector<int>> regions;

    for(std::size_t i = 0; i < grid.size(); ++i) {
        regions.emplace_back(width, -1);
    }

    for(int y = 0; (std::size_t)y < grid.size(); ++y) {
        for(int x = 0; (std::size_t)x < width; ++x) {
            uint64_t perimeter, area, sides;
            std::tie(perimeter, area, sides) = floodfill(grid, x, y, region++, regions);
            //if (perimeter != 0) {
            //    std::cout << "(" << x << "," << y << "): " << grid[y][x] << " -> " << perimeter << " x " << area << std::endl;
            //}
            result += (perimeter * area);
        }
    }


    return result;
}
