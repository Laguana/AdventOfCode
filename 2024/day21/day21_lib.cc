#include "day21_lib.h"

#include <iostream>
#include <utility>
#include <algorithm>
#include <unordered_map>

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

std::unordered_map<int,uint64_t> dirpad_memo;

uint64_t dirpad_shortest(int button, int depth) {
    if (depth == 0) {
        // you can push any button yourself by just pushing the button
        std::cout << "(" << "X^A<v>"[button] <<  ")";
        return 1;
    }

    int key = button <<4 | depth;

    if (dirpad_memo.contains(key)) {
        std::cout << depth << "X^A<v>"[button];
        return dirpad_memo[key];
    }

    // to push button  0 1 2
    //                 3 4 5
    // assuming you start at the A / 2, you need to do the following:
    
    int answer = -1;

    switch(button) {
    case 2:
        // to push button A when you're pointing at A, just push it
        answer = dirpad_shortest(2, depth-1);
        dirpad_memo[key] = answer;
        return answer;
    case 1:
        // to push 1, go left and hit A
        answer = dirpad_shortest(3, depth-1) + dirpad_shortest(2, depth-1);
        dirpad_memo[key] = answer;
        return answer;
    case 5:
        // to push 5, go down and hit A
        answer = dirpad_shortest(4, depth-1) + dirpad_shortest(2, depth-1);
        dirpad_memo[key] = answer;
        return answer;
    case 4:
        // to push 4, go down then left and hit A
        answer = dirpad_shortest(4, depth-1) + dirpad_shortest(3, depth-1) + dirpad_shortest(2, depth-1);
        dirpad_memo[key] = answer;
        return answer;
    case 3:
        // to push 3, go down then left twice and hit A
        // note that the second left is just a single extra press
        answer = dirpad_shortest(4, depth-1) + dirpad_shortest(3, depth-1)+ 1 + dirpad_shortest(2, depth-1);
        dirpad_memo[key] = answer;
        return answer;
    default:
        std::unreachable();
    }

}

/*
uint64_t shortest_input(const std::vector<unsigned char> &code) {
    // On each keypad, we are doing a circuit from A to a list of targets and back to A
    // to be maximally fast we want as much repetition in directions as possible

    uint64_t result = 0;

    // numpad positions
    const std::vector<int> x_pos = {1,0,1,2,0,1,2,0,1,2,2};
    const std::vector<int> y_pos = {3,2,2,2,1,1,1,0,0,0,3};

    // The shortest input on the numpad will be
    // up A left B to first number, A,
    // up/down C left/right D to next number, A
    // up/down E left/right F to next number, A
    // right G down E to A

    unsigned char current_key = 10;

    for(auto key: code) {
        auto dx = x_pos[key]-x_pos[current_key];
        auto dy = y_pos[key]-y_pos[current_key];

        // first robot wants to go dy up/down and dx left/right, then A
        // but not necessarily in that order; we need to avoid the blank at 0,3

        if (dx > 0 ) {
            // go right first
            result += dirpad_shortest(4,2); 
            result += dirpad_shortest(2,2);
            result += dx-1; // if we step more than once, just keep hitting a

            // go up or down
            if (dy < 0) {
                result += dirpad_shortest(4,2);
                result += dirpad_shortest(3,2);
                result += dirpad_shortest(2,2);
                result += std::abs(dy)-1;
                result += dirpad_shortest(5,2);
                result += dirpad_shortest(2,2);
            } else if (dy > 0) {
                result += dirpad_shortest(3,2);
                result += dirpad_shortest(2,2);
                result += std::abs(dy)-1;
                result += dirpad_shortest(5,2);
            }
        } else {
            // go up/down first, and then maybe left if dx < 0
            if (dy > 0) {
                result += dirpad_shortest(3,2);
                result += dirpad_shortest(2,2);
                result += std::abs(dy)-1;

                if (dx < 0) {
                    result += dirpad_shortest(4,2);
                    result += dirpad_shortest(3,2);
                    result += dirpad_shortest(2,2);
                    result += std::abs(dx-1);
                    result += dirpad_shortest(5,2);
                    result += 1;
                    result += dirpad_shortest(1,2);
                } else {
                    result += dirpad_shortest(5,2);
                }
            } else if (dy < 0) {
                result += dirpad_shortest(3,2);
                result += dirpad_shortest(4,2);
                result += dirpad_shortest(2,2);
                result += std::abs(dy)-1;
                if (dx < 0) {
                    result += dirpad_shortest(3,2);
                    result += dirpad_shortest(2,2);
                    result += std::abs(dx-1);
                    result += dirpad_shortest(5,2);
                    result += 1;
                    result += dirpad_shortest(1,2);
                } else {
                    result += dirpad_shortest(5,2);
                    result += dirpad_shortest(1,2);
                }
            }
        }

        // hit A
        result += dirpad_shortest(2,1);
    }
    std::cout << std::endl;

    
    return result;
}
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

uint64_t shortest_input(const std::vector<unsigned char> &code) {

    const std::vector<int> x_pos = {1,0,1,2,0,1,2,0,1,2,2};
    const std::vector<int> y_pos = {3,2,2,2,1,1,1,0,0,0,3};

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
    std::cout << std::endl;
    //*/
    auto stage3 = expand(stage2);
    
    /** /
    for(const auto d: stage3) {
        std::cout << dirpad_lookup[d];
    }
    std::cout << std::endl;
    // */

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