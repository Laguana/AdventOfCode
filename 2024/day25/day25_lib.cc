#include "day25_lib.h"

#include <iostream>

Input Input::parse(const unsigned char* start, std::size_t len) {
    std::vector<std::vector<int>> locks;
    std::vector<std::vector<int>> keys;

    auto end = start+len;

    auto p = start;
    while(p != end) {
        // all inputs are 5x7 grids        
        if (*p == '#') {
            // lock
            std::vector<int> lock;

            for (int x = 0; x < 5; ++x) {
                for(int y = 1; y < 7; ++y) {
                    if (p[x + (6*y)] == '.') {
                        lock.push_back(y-1);
                        break;
                    }
                }
            }

            locks.push_back(lock);
        } else {
            // key
            std::vector<int> key;

            for (int x = 0; x < 5; ++x) {
                for(int y = 1; y < 7; ++y) {
                    if (p[x + (6*y)] == '#') {
                        key.push_back(6-y);
                        break;
                    }
                }
            }

            keys.push_back(key);
        }

        p += (6 * 7);
        if (p == end) {
            break;
        } else {
            p++;
        }
    }

    return Input(locks, keys);
}

int Input::matching_combinations() const {
    int result = 0;

    for(auto &lock: locks) {
        for(auto &key: keys) {
            bool matched = true;
            for(int i = 0; i < 5; ++i) {
                if (lock[i] + key[i] > 5) {
                    matched = false;
                    break;
                }
            }
            if (matched) {
                ++result;
            }
        }
    }

    return result;
}