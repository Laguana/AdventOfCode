#include "day21_lib.h"

#include <iostream>
#include <utility>
#include <algorithm>
#include <unordered_map>
#include <limits>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start+len;
    std::vector<std::vector<unsigned char>> codes;

    auto p = start;
    while(p != end) {
        std::vector<unsigned char> code;
        while(*p != '\n') {
            auto c = *p++;
            if (c == 'A') {
                code.push_back(10);
            } else {
                code.push_back(c - '0');
            }
        }
        codes.push_back(code);
        p++;
    }

    return Input(codes);
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/

enum Dirpad: unsigned char {
    Up,
    Down,
    A,
    Left,
    Right,
};
const char* dirpad_lookup = "^vA<>";

std::vector<Dirpad> expand(const std::vector<Dirpad> & input) {
    std::vector<Dirpad> result;

    const std::vector<int> x_pos {1,1,2,0,2};
    const std::vector<int> y_pos {0,1,0,1,1};

    Dirpad current = Dirpad::A;

    for(auto d: input) {
        int dx = x_pos[d]-x_pos[current];
        int dy = y_pos[d]-y_pos[current];

        // prefer right, down, up, left;
        while(dx > 0) {
            result.push_back(Dirpad::Right);
            --dx;
        }
        while(dy > 0) {
            result.push_back(Dirpad::Down);
            --dy;
        }
        
        while(dx < 0) {
            result.push_back(Dirpad::Left);
            ++dx;
        }
        while(dy < 0) {
            result.push_back(Dirpad::Up);
            ++dy;
        }

        result.push_back(Dirpad::A);
        current = d;
    }

    return result;
}

template<class T,  class U>
struct std::hash<std::pair<T, U>>
{
    std::size_t operator()(const std::pair<T,U>& s) const noexcept
    {
        std::size_t h1 = std::hash<T>{}(s.first);
        std::size_t h2 = std::hash<U>{}(s.second);
        return h1 ^ (h2 << 8);
    }
};

std::unordered_map<std::pair<uint64_t, int>,uint64_t> dirpad_memo = {};

uint64_t count_expanded(const std::vector<Dirpad> & input, int depth) {
    if (depth == 0) {
        return input.size();
    }
    uint64_t key1 = 0;
    for(auto d : input) {
        key1 <<=3;
        key1 |= d+1;
    }
    std::pair<uint64_t, int> key(key1, depth);
    /*
    std::cout << "++" << depth << ":";
    for(auto i:input) {
        std::cout << dirpad_lookup[i];
    }
    */
    if(dirpad_memo.contains(key)) {
        /*std::cout << std::oct << key.first << "~" << dirpad_memo.find(key)->first.first << std::dec;
        if (dirpad_memo.find(key)->first != key) {
            std::cout << "????" << std::endl;
        }
        std::cout << "=" << dirpad_memo[key] << std::endl;
        */
        return dirpad_memo[key];
    } else {
        //std::cout << "= ??" << std::endl;
    }

    static const std::vector<int> x_pos {1,1,2,0,2};
    static const std::vector<int> y_pos {0,1,0,1,1};

    Dirpad current = Dirpad::A;

    uint64_t result = 0;

    for(auto d: input) {
        int dx = x_pos[d]-x_pos[current];
        int dy = y_pos[d]-y_pos[current];

        std::vector<Dirpad> sequence;

        // prefer right, down, up, left;
        while(dx > 0) {
            sequence.push_back(Dirpad::Right);
            --dx;
        }
        while(dy > 0) {
            sequence.push_back(Dirpad::Down);
            --dy;
        }
        
        while(dx < 0) {
            sequence.push_back(Dirpad::Left);
            ++dx;
        }
        while(dy < 0) {
            sequence.push_back(Dirpad::Up);
            ++dy;
        }

        std::sort(sequence.begin(), sequence.end(), std::less<Dirpad>());

        uint64_t best = std::numeric_limits<uint64_t>::max();
        //int iter = 0;
        do {
            //std::cout << depth << ":" << iter++ << std::endl;
            if ((x_pos[d] == 0 || x_pos[current] == 0) && (y_pos[d] == 0 || y_pos[current] == 0)) {
                // can't go to 0,0
                bool valid = true;
                int x = x_pos[current];
                int y = y_pos[current];
                for(const auto d: sequence) {
                    switch(d) {
                    case Dirpad::Left:
                        x--;
                        break;
                    case Dirpad::Right:
                        x++;
                        break;
                    case Dirpad::Up:
                        y--;
                        break;
                    case Dirpad::Down:
                        y++;
                        break;
                    default:
                        break;
                    }
                    if (x == 0 && y == 0) {
                        valid = false;
                        break;
                    }
                }
                if (!valid) {
                    continue;
                }
            }
            sequence.push_back(Dirpad::A);
            best = std::min(best, count_expanded(sequence,depth-1));
            sequence.pop_back();
        } while(std::next_permutation(sequence.begin(), sequence.end()));

        result += best;
        current = d;
    }    

    //std::cout << "==" << depth << "=" << result << std::endl;
    dirpad_memo[key] = result;
    return result;
}

uint64_t shortest_input(const std::vector<unsigned char> &code) {

    static const std::vector<int> x_pos = {1,0,1,2,0,1,2,0,1,2,2};
    static const std::vector<int> y_pos = {3,2,2,2,1,1,1,0,0,0,3};

    unsigned char current = 10;

    std::vector<Dirpad> stage1;

    for(auto key: code) {
        int dx = x_pos[key]-x_pos[current];
        int dy = y_pos[key]-y_pos[current];

        // prefer right first, then up/down, then left
        // actually not always best??
        // <<^^ encode shorter than ^^<<

        std::vector<Dirpad> candidate;
        
        while(dx > 0) {
            candidate.push_back(Dirpad::Right);
            --dx;
        }
        while(dy > 0) {
            candidate.push_back(Dirpad::Down);
            --dy;
        }
        while(dy < 0) {
            candidate.push_back(Dirpad::Up);
            ++dy;
        }
        while(dx < 0) {
            candidate.push_back(Dirpad::Left);
            ++dx;
        }

        if ((x_pos[key] == 0 || x_pos[current] == 0) && (y_pos[key] == 3 || y_pos[current] == 3)) {
            candidate.push_back(Dirpad::A);
        } else {
            std::sort(candidate.begin(), candidate.end(), std::greater());
            candidate.push_back(Dirpad::A);
            auto size1 = expand(expand(candidate)).size();
            candidate.pop_back();
            std::sort(candidate.begin(), candidate.end(), std::less());
            candidate.push_back(Dirpad::A);
            auto size2 = expand(expand(candidate)).size();

            if (size1 < size2) {
                candidate.pop_back();
                std::sort(candidate.begin(), candidate.end(), std::greater());
                candidate.push_back(Dirpad::A);
            }
        }

        //std::cout << expand(expand(candidate)).size() << " =?= " << count_expanded(candidate,2) << std::endl;

        stage1.insert(stage1.end(), candidate.begin(), candidate.end());

        current = key;
    }

    /** /
    for(const auto d: stage1) {
        std::cout << dirpad_lookup[d];
    }
    std::cout << std::endl;
    //*/
    
    auto stage2 = expand(stage1);
    /** /
    for(const auto d: stage2) {
        std::cout << dirpad_lookup[d];
    }
    std::cout << ": " << stage2.size() << " ?= " << count_expanded(stage1, 1) << std::endl;
    //*/
    auto stage3 = expand(stage2);
    
    /** /
    for(const auto d: stage3) {
        std::cout << dirpad_lookup[d];
    }
    std::cout << ": " << stage3.size() << " ?= " << count_expanded(stage2, 1) << " ?= " << count_expanded(stage1, 2) << std::endl;
    //*/

    return stage3.size();
}

uint64_t Input::score_codes() const {
    uint64_t result = 0;

    for(auto &code:codes) {
        auto dist = shortest_input(code);
        //std::cout << dist << std::endl;
        result += dist * (code[0] * 100 + code[1] * 10 + code[2]);
    }

    return result;
}

uint64_t count_shortest_input(const std::vector<unsigned char> &code, int depth) {

    static const std::vector<int> x_pos = {1,0,1,2,0,1,2,0,1,2,2};
    static const std::vector<int> y_pos = {3,2,2,2,1,1,1,0,0,0,3};

    unsigned char current = 10;

    uint64_t result = 0;;

    for(auto key: code) {
        int dx = x_pos[key]-x_pos[current];
        int dy = y_pos[key]-y_pos[current];

        // prefer right first, then up/down, then left
        // actually not always best??
        // <<^^ encode shorter than ^^<<

        std::vector<Dirpad> candidate;
        
        while(dx > 0) {
            candidate.push_back(Dirpad::Right);
            --dx;
        }
        while(dy > 0) {
            candidate.push_back(Dirpad::Down);
            --dy;
        }
        while(dy < 0) {
            candidate.push_back(Dirpad::Up);
            ++dy;
        }
        while(dx < 0) {
            candidate.push_back(Dirpad::Left);
            ++dx;
        }

        uint64_t current_best = std::numeric_limits<uint64_t>::max();

        std::sort(candidate.begin(), candidate.end(), std::less<Dirpad>());
        do {
            if ((x_pos[key] == 0 || x_pos[current] == 0) && (y_pos[key] == 3 || y_pos[current] == 3)) {
                bool valid = true;
                int x = x_pos[current];
                int y = y_pos[current];
                for(const auto d: candidate) {
                    switch(d) {
                    case Dirpad::Left:
                        x--;
                        break;
                    case Dirpad::Right:
                        x++;
                        break;
                    case Dirpad::Up:
                        y--;
                        break;
                    case Dirpad::Down:
                        y++;
                        break;
                    default:
                        break;
                    }
                    if (x == 0 && y == 3) {
                        valid = false;
                        break;
                    }
                }
                if (!valid) {
                    continue;
                }
            }
            candidate.push_back(Dirpad::A);

            /** /
            for(auto c: candidate) {
                std::cout << dirpad_lookup[c];
            }
            std::cout << std::endl;
            //*/
            current_best = std::min(current_best, count_expanded(candidate,depth));
            candidate.pop_back();
        } while(std::next_permutation(candidate.begin(), candidate.end()));
        //std::cout << "->" << current_best << std::endl;

        result += current_best;

        current = key;
    }
    return result;
}

uint64_t Input::score_codes2(int depth) const {
    uint64_t result = 0;

    for(auto &code:codes) {
        auto dist = count_shortest_input(code, depth);
        //std::cout << dist << std::endl;
        result += dist * (code[0] * 100 + code[1] * 10 + code[2]);
    }

    return result;
}