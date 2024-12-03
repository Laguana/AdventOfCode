#include "day3_lib.h"

// who _wouldn't_ want to hand author a parsing state machine with gotos in c++??
std::optional<Instruction> Instruction::parse(const char** input, const char* end) {
    uint16_t left, right;
    uint16_t temp = 0;
    char state = 0;
    const char *p = *input;
    char c = 0;
    std::optional<Instruction> result = std::nullopt;
    while(p != end) {
        c = *p++;
        switch(state) {
        case 0:
            if (c == 'd') {
                state = 11;
            } else if (c != 'm') {
                goto done_parsing;
            }
            break;
        case 1:
            if (c != 'u') {
                goto done_parsing;
            }
            break;
        case 2:
            if (c != 'l') {
                goto done_parsing;
            }
            break;
        case 3:
            if (c != '(') {
                goto done_parsing;
            }
            break;
        case 4:
            if (c < '0' || c > '9') {
                goto done_parsing;
            }
            temp = c-'0';
            break;
        case 5:
            if (c == ',') {
                state = 6;
                --p;
            } else if (c < '0' || c > '9') {
                goto done_parsing;
            } else {
                temp *= 10;
                temp += c-'0';
            }
            break;
        case 6:
            if (c == ',') {
                --p;
            } else if (c < '0' || c > '9') {
                goto done_parsing;
            } else {
                temp *= 10;
                temp += c-'0';
            }
            break;
        case 7:
            if (c != ',') {
                goto done_parsing;
            }
            left = temp;
            temp = 0;
            break;
        case 8:
            if (c < '0' || c > '9') {
                goto done_parsing;
            }
            temp = c-'0';
            break;
        case 9:
            if (c == ')') {
                state = 10;
                --p;
            } else if (c < '0' || c > '9') {
                goto done_parsing;
            } else {
                temp *= 10;
                temp += c-'0';
            }
            break;
        case 10:
            if (c == ')') {
                --p;
            } else if (c < '0' || c > '9') {
                goto done_parsing;
            } else {
                temp *= 10;
                temp += c-'0';
            }
            break;
        case 11:
            if (c != ')') {
                goto done_parsing;
            }
            right = temp;
            result = Instruction(left, right);
            goto done_parsing;

        case 12:
            if (c != 'o') {
                goto done_parsing;
            }
            break;
        case 13:
            if (c == 'n') {
                state = 14;
            } else if (c != '(') {
                goto done_parsing;
            }
            break;
        case 14:
            if (c != ')') {
                goto done_parsing;
            }
            result = Instruction(Instruction::Kind::Do);
            goto done_parsing;
        case 15:
            if (c != '\'') {
                goto done_parsing;
            }
            break;
        case 16:
            if (c != 't') {
                goto done_parsing;
            }
            break;
        case 17:
            if (c != '(') {
                goto done_parsing;
            }
            break;
        case 18:
            if (c != ')') {
                goto done_parsing;
            }
            result = Instruction(Instruction::Kind::Dont);
            goto done_parsing;
        }
        ++state;
    }
done_parsing:
    if (!result.has_value() && 
            ((c == 'm' && (end-p) > 6)
            || (c == 'd' && (end-p) > 3))) {
        // This might actually be the start of another; unconsume it
        --p;
    }
    *input = p;
    return result;
    
}

uint64_t sum_products(const char* start, const char* end) {
    uint64_t result = 0;

    while (start != end) {
        auto parsed = Instruction::parse(&start, end);
        if (parsed.has_value()) {
            result += parsed.value().product();
        }
    }

    return result;
}

uint64_t sum_enabled_products(const char* start, const char* end) {
    uint64_t result = 0;
    bool enabled = true;

    while (start != end) {
        auto parsed = Instruction::parse(&start, end);
        if (parsed.has_value()) {
            switch(parsed.value().get_kind()) {
            case Instruction::Kind::Do:
                enabled = true;
                break;
            case Instruction::Kind::Dont:
                enabled = false;
                break;
            case Instruction::Kind::Mul:
                if (enabled) {
                    result += parsed.value().product();
                }
            }
        }
    }

    return result;
}