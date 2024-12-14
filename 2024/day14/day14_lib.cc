#include "day14_lib.h"

#include <iostream>
#include <assert.h>

Input Input::parse(const unsigned char* start, std::size_t length) {
    auto end = start + length;
    auto p = start;

    std::vector<Bot> bots;

    while(p != end) {
        while (*p++ != '=');

        int64_t x=0,y=0,vx=0,vy=0;
        bool negative = false;

        unsigned char c;
        while ((c = *p++) != ',') {
            if (c == '-') {
                negative = true;
                continue;
            }
            x *= 10;
            x += c-'0';
        }
        if (negative) {
            x *= -1;
            negative = false;
        }
        while((c = *p++) != ' ') {
            if (c == '-') {
                negative = true;
                continue;
            }
            y *= 10;
            y += c-'0';
        };
        if (negative) {
            y *= -1;
            negative = false;
        }
        while(*p++ != '=');
        while ((c = *p++) != ',') {
            if (c == '-') {
                negative = true;
                continue;
            }
            vx *= 10;
            vx += c-'0';
        }
        if (negative) {
            vx *= -1;
            negative = false;
        }
        while((c = *p++) != '\n') {
            if (c == '-') {
                negative = true;
                continue;
            }
            vy *= 10;
            vy += c-'0';
        };
        if (negative) {
            vy *= -1;
            negative = false;
        }

        bots.emplace_back(x,y,vx, vy);
    }

    return Input(bots);
}

int Input::mult_quadrants(int steps, int width, int height) const {
    int q1=0, q2=0, q3=0, q4=0;

    auto xmid = width/2;
    auto ymid = height/2;

    for(auto bot : bots) {
        auto fx = ((bot.x + steps * bot.vx) % width + width) % width;
        auto fy = ((bot.y + steps * bot.vy) % height + height) % height;
        
        if (fx < xmid) {
            if (fy < ymid) {
                ++q1;
            } else if (fy > ymid) {
                ++q3;
            }
        } else if (fx > xmid) {
             if (fy < ymid) {
                ++q2;
             } else if (fy > ymid) {
                ++q4;
             }
        }
    }

    return q1 * q2 * q3 * q4;
}

void Input::execute_until_stop(int width, int height) {
    int step = 0;
    char c = 0;

    std::vector<std::vector<int>> map;
    for(int y = 0; y < height; ++y) {
        map.emplace_back(width);
    }

    for (auto bot: bots) {
        map[bot.y][bot.x]++;
    }

    do {
        std::cout << step << std::endl;

        for (auto &row : map) {
            for (auto e : row) {
                if (e == 0) {
                    std::cout << " ";
                } else if (e < 10) {
                    std::cout << e;
                } else {
                    std::cout << "*";
                }
            }
            std::cout << std::endl;
        }

        for(auto &bot: bots) {
            auto fx = ((bot.x + bot.vx) + width) % width;
            auto fy = ((bot.y + bot.vy) + height) % height;
            if (map[bot.y][bot.x] == 0) {
                std::cout << bot.x <<"," << bot.y << " " << bot.vx << "," << bot.vy << std::endl;
            }
            map[bot.y][bot.x]--;
            assert(map[bot.y][bot.x] >= 0);
            map[fy][fx]++;
            bot.x = fx;
            bot.y = fy;
        }
        ++step;

        if (step %100 == 0)
        {
            std::cin >> c;
        }
    } while (c != 'q');
}

void Input::print_at(int steps, int width, int height) const {
    std::vector<std::vector<int>> map;
    for(int y = 0; y < height; ++y) {
        map.emplace_back(width);
    }

    for (auto bot: bots) {
        auto fx = ((bot.x + steps * bot.vx)% width + width)% width;
        auto fy = ((bot.y + steps * bot.vy) % height + height) % height;
        map[fy][fx]++;
    }

    for (auto &row : map) {
        for (auto e : row) {
            if (e == 0) {
                std::cout << " ";
            } else if (e < 10) {
                std::cout << e;
            } else {
                std::cout << "*";
            }
        }
        std::cout << std::endl;
    }
}