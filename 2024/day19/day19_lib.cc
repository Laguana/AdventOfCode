#include "day19_lib.h"

#include <iostream>
#include <string_view>
#include <unordered_map>

#include <algorithm>

#include <regex>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start+len;
    std::vector<std::string> available;
    std::vector<std::string> desired;

    auto p = start;
    while (*p != '\n') {
        auto pend = p;
        for(; *pend != ',' && *pend != '\n'; ++pend);
        available.emplace_back(p,pend);
        p = pend;
        if (*p == ',') {
            p += 2;
        }
    }
    p+=2;
    while(p != end) {
        auto pend = p;
        for(; *pend != '\n'; ++pend);
        desired.emplace_back(p, pend);
        p = pend + 1;
    }

    return Input(available, desired);
}



bool is_possible(const std::string_view s, const std::vector<std::string> &available) {
    if (s.length() == 0) {
        return true;
    }
    for(auto &candidate: available) {
        if (s.starts_with(candidate)) {
            //std::cout << s << " starts with " << candidate << std::endl;
            if (is_possible(s.substr(candidate.size()), available)) {
                return true;
            }
        }
    }
    return false;
}




std::vector<std::string> reduce_options(const std::vector<std::string> &available) {
    std::vector<std::string> reduced;

    std::vector<std::string> sorted(available);
    std::sort(sorted.begin(), sorted.end(), [](const std::string &a, const std::string &b) { return a.size() < b.size();});

    for(auto &s: sorted) {
        if (!is_possible(s, reduced)) {
            //std::cout << "reduced " << s << std::endl;
            reduced.push_back(s);
        }
    }

    return reduced;
}

int Input::count_possible() const {
    auto reduced = reduce_options(available);
    
    
    int possible = 0;
    for(auto &potential : desired) {
        //std::cout << potential;
        if (is_possible(potential, reduced)) {
            ++possible;
            //std::cout << " yes";
        }
        // std::cout << std::endl;
    }
    return possible;
    
    /*
    auto it = available.cbegin();
    std::string options = *it++;
    while(it != available.cend()) {
        options += "|"+*it++;
    }
    std::regex regex("^("+options+")*$");

    int possible = 0;
    std::cout << "Beginning" << std::endl;
    for(auto potential:desired) {
        std::cout << "Considering " << potential << std::endl;
        std::sregex_iterator it(potential.begin(), potential.cend(), regex), it_end;
        if (it != it_end) {
            possible++;
            std::cout << " Yes" << std::endl;
        }
    }

    return possible;
    */
}

uint64_t how_possible(const std::string_view s, const std::vector<std::string> &available, std::unordered_map<std::string_view, uint64_t> &memo) {
    if (s.length() == 0) {
        return true;
    }
    auto found = memo.find(s);
    if (found != memo.end()) {
        return found->second;
    } else {}
    //std::cout << s << std::endl;

    uint64_t count = 0;
    for(auto &candidate: available) {
        if (s.starts_with(candidate)) {
            //std::cout << " " << candidate << std::endl;
            count += how_possible(s.substr(candidate.size()), available, memo);
        }
    }
    //std::cout << "memoizing " << s << " -> " << count << std::endl;
    memo[s] = count;
    return count;
}

uint64_t Input::count_how_possible() const {
    uint64_t result = 0;
    std::unordered_map<std::string_view, uint64_t> memo;
    for(auto &potential : desired) {
        result += how_possible(potential, available, memo);
    }
    return result;
}