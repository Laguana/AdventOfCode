#include "day2_lib.h"

#include <cmath>

#include <iostream>

day2_input parse_input(std::basic_istream<char> &input) {
    day2_input result;

    do {
        std::vector<int> row;
        while(input) {
            if (input.peek() == '\r') {
                input.get();
            }
            if (input.peek() == '\n') {
                input.get();
                break;
            }
            if (input.peek() == -1) {
                break;
            }
            int entry;
            input >> entry;
            row.push_back(entry);
        } 

        result.push_back(row);
    } while(input && input.peek() != -1);

    return result;
}

int signum(int x) {
    return (0 < x) - (x < 0);
}

bool safe_delta(int delta) {
    int dist = abs(delta);
    return dist >= 1 && dist <= 3;
}

bool is_safe(const std::vector<int> &row) {
    auto iter = row.cbegin();

    int first = *iter++;
    int prev = *iter++;
    int dx = prev-first;
    if (!safe_delta(dx)) {
        return false;
    }
    int direction = signum(dx);

    //std::cout << dx << " " << direction << std::endl;

    for(; iter != row.cend(); ++iter) {
        int cur = *iter;
        dx = cur-prev;

        //std::cout << dx << std::endl;
        if (!safe_delta(dx) || signum(dx) != direction) {
            return false;
        }
        prev = cur;
    }

    return true;
}

int count_safe(const std::vector<std::vector<int>> &input) {
    int result = 0;
    for (auto it = input.cbegin(); it != input.cend(); ++it){ 
        if (is_safe(*it)){
            result++;
        }
    }
    return result;
}

bool is_damped_safe(const std::vector<int> &row) {
    auto iter = row.cbegin();

    bool used_damper = false;

    int first = *iter++;
    int pprev = *iter++;

    // nothing has only 3 elements, so I'm not going to bother with the validation logic here.

    int prev = *iter++;

    int dx = pprev-first;

    if (!safe_delta(dx)) {
        if (!safe_delta(prev-first)) {
            dx = prev-pprev;
        } else if (!safe_delta(prev-pprev)) {
            dx = prev-first;
        } else {
            // could remove either, check the directions match
            if (signum(prev-first) == signum(prev-pprev)) {
                // doesn't really matter
                dx = prev-first;
            } else {
                // need to check the next entry to make sure the direction is right
                if (iter == row.cend()) {
                    // if there were only 3 elements, whatever it's fine
                    return true;
                } else {
                    dx = (*iter - prev);
                }
            }
        }
        if (!safe_delta(dx)) {
            return false;
        }
        used_damper = true;
        //std::cout << "removed one of first 2" << std::endl;
    } else if (!safe_delta(prev-pprev)) {
        // first to second is ok, second to third is not
        if (iter == row.cend()) {
            return true;
        }
        int cur = *iter++;
        if (!safe_delta(prev-first)) {
            // first to third is not, so we must remove the third
            if (!safe_delta(cur-pprev)) {
                return false;
            }
            prev = cur;
            used_damper = true;
            //std::cout << "removed third element" << std::endl;
        } else {
            // first to third is ok, could remove either
            if (signum(cur-first) != signum(dx)) {
                if (signum(cur-prev) != signum(prev-cur)) {
                    return false;
                }
                // remove second element
                dx = prev-first;
                pprev = prev;
                prev = cur;
                used_damper = true;
            } else {
                // first to fourth is the same direction as first to third,
                // so it doesn't really matter which we remove
                prev = cur;
                used_damper = true;
            }
        }
    }

    int direction = signum(dx);

    //std::cout << "direction is " << dx << " " << direction << std::endl;

    // ok, made it this far, we've looked at the first 3 elements and picked a direction

    for(;iter != row.cend(); ++iter) {
        auto cur = *iter;
        dx = cur-prev;
        if (!safe_delta(dx) || signum(dx) != direction) {
            if (used_damper) {
                return false;
            }
            used_damper = true;

            if (!safe_delta(cur-pprev) || signum(cur-pprev) != direction) {
                // too far to go from n to n-2, skip this one
                // deliberately not updating prev/pprev
                //std::cout << "skipping " << cur << std::endl;
                continue;
            }
            // Could remove either the current one, or the previous one.
            // look ahead one
            if (iter+1 == row.cend()) {
                // actually this is the end, we're fine
                return true;
            }
            int next = *(iter+1);
            if (safe_delta(next-cur) && signum(next-cur) == direction) {
                // we can remove prev fine
                // deliberately do not update pprev
                //std::cout << "skipping " << prev << std::endl;
                prev = cur;
                continue;
            }
        }

        pprev = prev;
        prev = cur;
    }
    return true;
}

int count_damped_safe(const std::vector<std::vector<int>> &input) {
    int result = 0;
    for (auto it = input.cbegin(); it != input.cend(); ++it){ 
        if (is_damped_safe(*it)){
            result++;
            //std::cout << "safe" << std::endl;
        } else {
            //std::cout << "unsafe" << std::endl;
        }
    }
    return result;
}