#include "day15_lib.h"

#include <utility>
#include <iostream>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start + len;
    int botx = 0;
    int boty = 0;

    std::vector<std::vector<Entity>> map;
    std::vector<Instruction> instructions;

    auto p = start;
    while(*p != '\n') {
        // parse the map;
        std::vector<Entity> row;
        unsigned char c;
        while((c = *p++) != '\n') {
            switch(c) {
            case '#':
                row.emplace_back(Entity::Wall);
                break;
            case '.':
                row.emplace_back(Entity::Empty);
                break;
            case '@':
                botx = row.size();
                boty = map.size();
                row.emplace_back(Entity::Robot);
                break;
            case 'O':
                row.emplace_back(Entity::Box);
                break;
            default:
                std::unreachable();
            }
        }
        map.push_back(row);
    }
    // empty line, time to parse the instructions
    while(p != end) {
        switch(*p++) {
        case '^':
            instructions.push_back(Instruction::Up);
            break;
        case '>':
            instructions.push_back(Instruction::Right);
            break;
        case '<':
            instructions.push_back(Instruction::Left);
            break;
        case 'v':
            instructions.push_back(Instruction::Down);
            break;
        case '\n':
            break;
        default:
            std::unreachable();
        }
    }

    return Input(botx,boty, map, instructions);
}

uint64_t Input::run_instructions_and_score() const {
    std::vector<std::vector<Entity>> map = this->map;

    int botx = this->botx, boty = this->boty;

    for(auto instruction: instructions) {
        int dx, dy;
        switch(instruction) {
        case Instruction::Up:
            dx = 0;
            dy = -1;
            break;
        case Instruction::Right:
            dx = 1;
            dy = 0;
            break;
        case Instruction::Left:
            dx = -1;
            dy = 0;
            break;
        case Instruction:: Down:
            dx = 0;
            dy = 1;
            break;
        default:
            std::unreachable();
        }
        switch (map[boty+dy][botx+dx]) {
        case Entity::Empty:
            map[boty][botx] = Entity::Empty;
            botx = botx+dx;
            boty = boty+dy;
            map[boty][botx] = Entity::Robot;
            break;
        case Entity::Wall:
            break;
        case Entity::Robot:
        case Entity::BoxLeft:
        case Entity::BoxRight:
            std::unreachable();
        case Entity::Box:
            // see what is at the other end;
            int x = botx + 2*dx;
            int y = boty + 2*dy;
            while (map[y][x] == Entity::Box) {
                x += dx;
                y += dy;
            }
            if (map[y][x] == Entity::Empty) {
                map[y][x] = Entity::Box;
                map[boty][botx] = Entity::Empty;
                map[boty+dy][botx+dx] = Entity::Robot;
                botx += dx;
                boty += dy;
            }
            break;
        }
    }
    // now scoring

    uint64_t score = 0;
    for(std::size_t y = 1; y < map.size(); ++y) {
        for(std::size_t x = 1; x < map[y].size(); ++x) {
            if (map[y][x] == Entity::Box) {
                score += 100 * y + x;
            }
        }
    }

    return score;
}

bool can_push_vertical(int dy, int x, int y, const std::vector<std::vector<Entity>> &map) {
    // We assume that x/y is half a box, and we want to see the consequences of moving
    int x2 = x;
    if (map[y][x] == Entity::BoxLeft) {
        x2++;
    } else {
        x2--;
    }
    auto e1 = map[y+dy][x];
    auto e2 = map[y+dy][x2];
    if (e1 == Entity::Wall || e2 == Entity::Wall) {
        return false;
    }
    bool b1 = true;
    bool b2 = true;
    if (e1 != Entity::Empty) {
        b1 = can_push_vertical(dy, x, y+dy, map);
    }
    if (e2 != Entity::Empty) {
        b2 = can_push_vertical(dy, x2, y+dy, map);
    }
    return b1 && b2;
}

void do_push_vertical(int dy, int x, int y, std::vector<std::vector<Entity>> &map) {
    int x2 = x;
    if (map[y][x] == Entity::BoxLeft) {
        x2++;
    } else {
        x2--;
    }
    auto e1 = map[y+dy][x];
    if (e1 != Entity::Empty) {
        do_push_vertical(dy, x, y+dy, map);
    }
    auto e2 = map[y+dy][x2];
    if (e2 != Entity::Empty) {
        do_push_vertical(dy, x2, y+dy, map);
    }
    map[y+dy][x] = map[y][x];
    map[y+dy][x2] = map[y][x2];
    map[y][x] = Entity::Empty;
    map[y][x2] = Entity::Empty;
}

void print_map(const std::vector<std::vector<Entity>> &map) {
    for (auto &row: map) {
        for (auto e: row) {
            switch(e) {
            case Entity::Empty:
                std::cout << " ";
                break;
            case Entity::Wall:
                std::cout << "#";
                break;
            case Entity::Robot:
                std::cout << "@";
                break;
            case Entity::BoxLeft:
                std::cout << "[";
                break;
            case Entity::BoxRight:
                std::cout << "]";
                break;
            default:
                std::unreachable();
            }
        }
        std::cout << std::endl;
    }
}

uint64_t Input::run_instructions_on_double_wide() const {
    std::vector<std::vector<Entity>> map;
    for (auto &original_row: this->map) {
        std::vector<Entity> row;
        for(auto e: original_row) {
            switch(e) {
            case Entity::Empty:
            case Entity::Wall:
                row.push_back(e);
                row.push_back(e);
                break;
            case Entity::Robot:
                row.push_back(Entity::Robot);
                row.push_back(Entity::Empty);
                break;
            case Entity::Box:
                row.push_back(Entity::BoxLeft);
                row.push_back(Entity::BoxRight);
                break;
            default:
                std::unreachable();
            }
        }
        map.push_back(row);
    }
    int botx = this->botx*2;
    int boty = this->boty;

    for(auto instruction: instructions) {
        //std::cout << "^v<>"[instruction] << "(" << botx << "," << boty << ")" << std::endl;
        //print_map(map);
        int dx, dy;
        switch(instruction) {
        case Instruction::Up:
            dx = 0;
            dy = -1;
            break;
        case Instruction::Right:
            dx = 1;
            dy = 0;
            break;
        case Instruction::Left:
            dx = -1;
            dy = 0;
            break;
        case Instruction:: Down:
            dx = 0;
            dy = 1;
            break;
        default:
            std::unreachable();
        }
        switch (map[boty+dy][botx+dx]) {
        case Entity::Empty:
            map[boty][botx] = Entity::Empty;
            botx = botx+dx;
            boty = boty+dy;
            map[boty][botx] = Entity::Robot;
            break;
        case Entity::Wall:
            break;
        case Entity::Robot:
            std::unreachable();
        case Entity::Box:
            std::unreachable();
        case Entity::BoxLeft:
        case Entity::BoxRight:
            if (dy == 0) {
                // see what is at the other end;
                int x = botx + 3*dx;
                int y = boty;
                while (map[y][x] == Entity::BoxLeft 
                    || map[y][x] == Entity::BoxRight) {
                    x += dx;
                }
                if (map[y][x] == Entity::Empty) {
                    // We can do the move; walk back moving everything over
                    while(x != botx) {
                        map[y][x] = map[y][x-dx];
                        x-=dx;
                    }
                    map[y][botx] = Entity::Empty;
                    botx += dx;
                }
            } else {
                // up and down can push a tree of boxes
                if (can_push_vertical(dy, botx, boty+dy, map)) {
                    do_push_vertical(dy, botx, boty+dy, map);
                    map[boty][botx] = Entity::Empty;
                    map[boty+dy][botx] = Entity::Robot;
                    boty += dy;
                }
            }
            break;
        }
    }

    // now scoring

    uint64_t score = 0;
    for(std::size_t y = 1; y < map.size(); ++y) {
        for(std::size_t x = 1; x < map[y].size(); ++x) {
            if (map[y][x] == Entity::BoxLeft) {
                score += 100 * y + x;
            }
        }
    }

    return score;

}