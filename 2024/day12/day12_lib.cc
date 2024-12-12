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

int get_region(int x, int y, const std::vector<std::vector<int>> &regions) {
    if (x < 0 || y < 0 || (std::size_t)y >= regions.size() || (std::size_t)x >= regions[y].size()) {
        return -1;
    }
    return regions[y][x];
}

int left_edge_change(int x, int y, const std::vector<std::vector<int>> &regions) {
    // we just added (x,y)
    // What is the change in left-edges?
    
    // We count something as a left edge if it is the topmost tile of a left edge
    // meaning that
    // - it has a free spot to the left of it
    // - either it has nothing above it, or the thing above it has something to its left

    int delta = 0;

    auto this_region = regions[y][x];
    if (get_region(x-1,y, regions) != this_region) {
        // This is a candidate for a left edge
        if (get_region(x,y-1, regions) != this_region
            || get_region(x-1,y-1, regions) == this_region) {
            // This is a topmost left edge
            ++delta;        
        }
    }

    // Adding this tile may have un-left-edge'd something else

    // it could have made the tile to the right no longer be the top-left edge
    if (get_region(x+1,y, regions) == this_region
        && (get_region(x+1,y-1, regions) != this_region
            || get_region(x,y-1, regions) == this_region)) {
        --delta;
    }
    // it could have made the tile below no longer be the top left edge
    if (get_region(x,y+1, regions) == this_region
        && get_region(x-1,y+1, regions) != this_region
        && get_region(x-1,y, regions) != this_region) {
        --delta;
    }

    // but wait; this may not be a left edge, but could it have created a new one?

    // .##
    // .?A
    // ..B

    // the ? is not the topmost left edge, but A was
    // and now A is not, but B is

    // it could have made the tile down-right of it be a new left edge
    if (get_region(x+1,y+1, regions) == this_region
        && get_region(x,y+1, regions) != this_region
        && get_region(x+1,y, regions) == this_region) {
        ++delta;
    }

    //std::cout << "   <" << delta << std::endl;

    return delta;
}

int right_edge_change(int x, int y, const std::vector<std::vector<int>> &regions) {
    // we just added (x,y)
    // What is the change in right-edges?
    
    // We count something as a right edge if it is the topmost tile of a right edge
    // meaning that
    // - it has a free spot to the right of it
    // - either it has nothing above it, or the thing above it has something to its right

    int delta = 0;

    auto this_region = regions[y][x];
    if (get_region(x+1,y, regions) != this_region) {
        // This is a candidate for a right edge
        if (get_region(x,y-1, regions) != this_region
            || get_region(x+1,y-1, regions) == this_region) {
            // This is a topmost right edge
            ++delta;        
        }
    }

    // Adding this tile may have un-right-edge'd something else

    // it could have made the tile to the right no longer be the top-right edge
    if (get_region(x-1,y, regions) == this_region
        && (get_region(x-1,y-1, regions) != this_region
            || get_region(x,y-1, regions) == this_region)) {
        --delta;
    }
    // it could have made the tile below no longer be the top right edge
    if (get_region(x,y+1, regions) == this_region
        && get_region(x+1,y+1, regions) != this_region
        && get_region(x+1,y, regions) != this_region) {
        --delta;
    }

    // #..
    // #?.
    // #..

    // it could have made the tile down-left of it be a new right edge
    if (get_region(x-1,y+1, regions) == this_region
        && get_region(x,y+1, regions) != this_region
        && get_region(x-1,y, regions) == this_region) {
        ++delta;
    }

    //std::cout << "   >" << delta << std::endl;

    return delta;
}

int top_edge_change(int x, int y, const std::vector<std::vector<int>> &regions) {
    // we just added (x,y)
    // What is the change in top-edges?
    
    // We count something as a top edge if it is the leftmost tile of a top edge
    // meaning that
    // - it has a free spot above it
    // - either it has nothing left of it, or the thing left of it has something above it

    int delta = 0;

    auto this_region = regions[y][x];
    if (get_region(x,y-1, regions) != this_region) {
        // This is a candidate for a top edge
        if (get_region(x-1,y, regions) != this_region
            || get_region(x-1,y-1, regions) == this_region) {
            // This is a leftmost top edge
            ++delta;        
        }
    }

    // Adding this tile may have un-top-edge'd something else

    // it could have made the tile below no longer be the top-left edge
    if (get_region(x,y+1, regions) == this_region
        && (get_region(x-1,y+1, regions) != this_region
            || get_region(x-1,y, regions) == this_region)) {
        --delta;
    }
    // it could have made the tile to the right no longer be the top left edge
    if (get_region(x+1,y, regions) == this_region
        && get_region(x+1,y-1, regions) != this_region
        && get_region(x,y-1, regions) != this_region) {
        --delta;
    }

    // ...
    // .?.
    // ###

    // it could have made the tile down-right of it be a new top edge
    if (get_region(x+1,y+1, regions) == this_region
        && get_region(x+1,y, regions) != this_region
        && get_region(x,y+1, regions) == this_region) {
        ++delta;
    }

    //std::cout << "   ^" << delta << std::endl;

    return delta;
}

int bottom_edge_change(int x, int y, const std::vector<std::vector<int>> &regions) {
    // we just added (x,y)
    // What is the change in bottom-edges?
    
    // We count something as a bottom edge if it is the leftmost tile of a bottom edge
    // meaning that
    // - it has a free spot below it
    // - either it has nothing left of it, or the thing left of it has something below it

    int delta = 0;

    auto this_region = regions[y][x];
    if (get_region(x,y+1, regions) != this_region) {
        // This is a candidate for a bottom edge
        if (get_region(x-1,y, regions) != this_region
            || get_region(x-1,y+1, regions) == this_region) {
            // This is a leftmost bottom edge
            ++delta;        
        }
    }

    // Adding this tile may have un-bottom-edge'd something else

    // it could have made the tile above no longer be the bottom-left edge
    if (get_region(x,y-1, regions) == this_region
        && (get_region(x-1,y-1, regions) != this_region
            || get_region(x-1,y, regions) == this_region)) {
        --delta;
    }
    // it could have made the tile to the right no longer be the bottom left edge
    if (get_region(x+1,y, regions) == this_region
        && get_region(x+1,y+1, regions) != this_region
        && get_region(x,y+1, regions) != this_region) {
        --delta;
    }

    // ###
    // .?.
    // ...

    // it could have made the tile up-right of it be a new bottom edge
    if (get_region(x+1,y-1, regions) == this_region
        && get_region(x+1,y, regions) != this_region
        && get_region(x,y-1, regions) == this_region) {
        ++delta;
    }

    //std::cout << "   v" << delta << std::endl;

    return delta;
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

        // What's the deal with sides?
        // you start with 4; adding in a line is no change
        // creating a corner adds 2 sides
        //  #       #
        //  #  -->  ##
        // filing in a corner removes 2 sides
        //  #        ##
        //  ##  -->  ##
        // and you can fill in multiple corners at oncein theory
        //  # #     ###
        //  ### --> ###
        // or even
        //  # # --> ###  removing 4 sides at once
        // although with a queue based floodfill that may not be possible
        // how can we detect a corner?
        // ABC
        // D?F
        // GHI
        // if we place a piece at ?, when has a corner been made?
        // outside of the start, one of B D F H must exist, and we are
        // extending its sides.
        // if more than one of BDFH exist, we are combining sides
        // e.g. B+D combines Bv with dv, and d> with B>, for -2 sides
        // B+H combines H< with B<, H> with B> and removes Bv and H^
        // that is -4 edges

        // .#.        ##.        ###
        // .?. 3 -> 3 .?. 2 -> 4 .?. 1 -> 5
        // ...        ...        ...

        // .#.        X#.        X#X
        // .?. 6 -> 2 #?. 5 -> 1 #?# 4 -> 0
        // .#.        X#.        X#X

        // X#.        .#.        X##
        // #?. 4 -> 2 .?. 3 -> 3 #?. 4 -> 2
        // ...        ...        X#.

        // ##.        ##.
        // .?. 2 -> 4 .?. 5 -> 3
        // ...        .#.

        // alternately, maybe try to count things differently
        // every side has a topleft (and either continues down or to the right)
        // when we place a piece, see if it is the top (of an vertical edge)
        // or left (of a horizontal edge), and if it usurped that from another piece?

        // this piece is the top of a vertical if there is no piece to that side
        // and there is no piece above it or there is a piece above that side

        // AB
        // .CF
        // DE
        
        // the piece at C is the top of the left-vertical edge if B is empty,
        // or A is full.

        // Additionally, if E is present, then if D is empty E thought it was
        // the top
        // but we also need to account for if F thought it was the top of a
        // horizontal but adding C removed that horizontal entirely?

        //std::cout << "  (" << x << "," << y <<")" << std::endl;
        sides += bottom_edge_change(x,y,regions);
        sides += top_edge_change(x,y,regions);
        sides += left_edge_change(x,y,regions);
        sides += right_edge_change(x,y,regions);
        

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

uint64_t Input::cost_field(bool discounted) const {
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
            //    std::cout << "(" << x << "," << y << "): " << grid[y][x] << " -> " << perimeter << " x " << area  << " , " << sides << std::endl;
            //}
            if (discounted) {
                result += (sides * area);
            } else {
                result += (perimeter * area);
            }
        }
    }


    return result;
}
