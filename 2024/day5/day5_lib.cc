#include "day5_lib.h"

#include <iostream>

Input Input::parse(const unsigned char* start, const unsigned char* end) {
    std::vector<std::tuple<int,int>> constraints;
    std::vector<std::vector<int>> updates;

    const unsigned char* p = start;
    char c = 0;

    do {
        int num = 0;
        int left = 0;
        do {
            c = *p++;
            if (c == '|') {
                left = num;
                num = 0;
            } else if (c == '\n') {
                constraints.emplace_back(left, num);
                break;
            } else {
                num *= 10;
                num += c-'0';
            }
        } while (p != end);
    } while (p != end && *p != '\n');
    
    ++p;

    do {
        std::vector<int> &row = updates.emplace_back();
        int num = 0;
        do {
            c = *p++;
            if (c >= '0' && c <= '9') {
                num *= 10;
                num += c-'0';
            } else {
                row.push_back(num);
                num = 0;
            }
        } while(c != '\n');
    } while (p != end);

    return { constraints, updates };
}

int Input::get_ordered_midpoints() const {
    int result = 0;

    for (const auto &row : updates) {
        int prev = 0;
        for (const auto page : row) {
            if (before[page][prev]) {
                goto next_row;
            }
            prev = page;
        }
        // no constraints violated
        result += row[row.size()/2];
        next_row:
    }

    return result;
}

int Input::reorder_and_get_new_midpoint(std::vector<int> row) const {
    bool changed = false;
    do {
        changed = false;
        for (unsigned int i = 0; i < row.size()-1; ++i) {
            for (unsigned int j = i+1; j < row.size(); ++j) {
                if (before[row[j]][row[i]]) {
                    //std::cout << i << "," << j << ": " << row[j] << " must go before " << row[i] << std::endl;
                    std::swap(row[j],row[i]);
                    changed = true;
                    --i;
                    break;
                }
            }
        }
    } while(changed);
    return row[row.size()/2];
}

int Input::get_reordered_midpoints() const {
    int result = 0;

    for (const auto &row : updates) {
        int prev = 0;
        for (const auto page : row) {
            if (before[page][prev]) {
                goto reorder;
            }
            prev = page;
        }
        // no constraints violated
        continue;

        reorder:

        result += reorder_and_get_new_midpoint(row);
    }

    return result;
}
