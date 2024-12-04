#include "day4_lib.h"

Input Input::parse(const char* start, const char* end) {
    const char* p = start;
    while (*p != '\n') ++p;
    int width = p-start;
    int height = (end-start)/(width+1); // +1 for the newlines

    return Input(start, end, width, height);
}

char Input::get(int x, int y) const {
    if (x < 0 || x >= width) {
        return 0;
    }
    if (y < 0 || y >= height) {
        return 0;
    }

    return start[y * (width+1) + x];
}


int Input::count_xmas() const {
    int result = 0;
    for (int x = 0; x < width; ++x) {
        for (int y = 0; y < height; ++y) {
            if (get(x,y) == 'X') {
                // could be the start of something
                for (int dx = -1; dx <= 1; ++dx) {
                    for (int dy = -1; dy <= 1; ++dy) {
                        if (dx==0 && dy==0) {
                            continue;
                        }
                        if (get(x+dx,y+dy) == 'M'
                            && get(x+2*dx,y+2*dy) == 'A'
                            && get(x+3*dx,y+3*dy) == 'S') {
                            ++result;
                        }
                    }
                }
            }
        }
    }
    return result;
}

int Input::count_x_mas() const {
    int result = 0;
    for (int x = 1; x < width-1; ++x) {
        for (int y = 1; y < height-1; ++y) {
            if (get(x,y) == 'A') {
                // could be the middle of something
                for(int dx = -1; dx <= 1; dx+=2){
                    for (int dy = -1; dy <= 1; dy+=2) {
                        char c = get(x+dx, y+dy);
                        if (c != 'M' && c != 'S') {
                            goto next;
                        }
                    }
                }
                if (get(x-1,y-1) != get(x+1,y+1) &&
                    get(x+1,y-1) != get(x-1,y+1)) {
                    ++result;
                }
            }
next:
        }
    }
    return result;
}