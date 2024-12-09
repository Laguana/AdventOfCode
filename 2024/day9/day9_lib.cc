#include "day9_lib.h"

#include <iostream>

Input Input::parse(const unsigned char* start, const unsigned char* end) {
    std::vector<int> disk;
    std::vector<std::vector<std::size_t>> gaps {{},{},{},{},{},{},{},{},{}};

    auto p = start;

    bool free = false;
    int blockId = 0;
    while(p != end) {
        auto c = *p++;
        if (c == '\n') {
            break;
        }
        
        int v = c-'0';
        if (free && v > 0) {
            gaps[v-1].push_back(disk.size());
        }
        while (v-- > 0) {
            if (free) {
                disk.push_back(-1);
            } else {
                disk.push_back(blockId);
            }
        }

        if (!free) {
            // TODO: should this only increment if it wasn't 0 blocks?
            ++blockId;
        }
        free = !free;
    }

    return Input(disk,gaps);
}

uint64_t Input::defrag_and_checksum() const {
    uint64_t checksum = 0;

    std::vector<int> disk = this->disk;

    auto big = disk.size()-1;
    std::size_t small = 0;

    while (big > small) {
        if (disk[big] == -1) {
            --big;
        } else if (disk[small] != -1) {
            checksum += small * disk[small];
            ++small;
        } else {
            std::swap(disk[small], disk[big]);
            checksum += small * disk[small];
            ++small;
            --big;
        }
    }
    while (small < disk.size() && disk[small] != -1) {
        checksum += small * disk[small];
        ++small;
    }

    return checksum;
}

uint64_t Input::defrag_blocks_and_checksum() const {
    uint64_t checksum = 0;

    std::vector<int> disk = this->disk;
    std::vector<std::vector<std::size_t>> gaps = this->gaps;

    auto big = disk.size()-1;
    std::size_t small = 0;

    while(big > 0) {
        while (disk[big] == -1) {
            --big;
        }
        // found a file to move, now where to put it;
        // how big is it?
        auto big_end = big-1;
        while (disk[big_end] == disk[big] && big_end > 0) {
            --big_end;
        }
        int filesize = big-big_end;

        // now to find the right gap
        unsigned int first_gap_size = 10;
        std::size_t first_gap_pos = disk.size();
        for(int gapsize = 9; gapsize >= filesize; --gapsize) {
            if(gaps[gapsize-1].size() > 0) {
                if (gaps[gapsize-1][0] < first_gap_pos) {
                    first_gap_size = gapsize;
                    first_gap_pos = gaps[gapsize-1][0];
                }
            }
        }
        if (first_gap_size == 10 || first_gap_pos > big) {
            // nothing fit; leave in place
            big = big_end;
            continue;
        }

        for(int i = 0; i < filesize; ++i) {
            std::swap(disk[first_gap_pos + i], disk[big - i]);
        }
        int remaining_gap = first_gap_size - filesize;
        auto remaining_gap_position = first_gap_pos + filesize;
        gaps[first_gap_size-1].erase(gaps[first_gap_size-1].begin());

        if (remaining_gap > 0) {
            auto &v = gaps[remaining_gap-1];
            auto it = v.begin();
            while (it != v.end() && *it < remaining_gap_position) {
                ++it;
            }
            v.insert(it, remaining_gap_position);
        }

        big = big_end;
    }

    while (small < disk.size()) {
        if (disk[small] != -1) {
            checksum += small * disk[small];
        }
        ++small;
    }

    return checksum;
}
