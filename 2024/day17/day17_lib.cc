#include "day17_lib.h"

#include <iostream>
#include <sstream>
#include <utility>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start+len;

    int64_t regA=-1, regB=-1, regC=-1;
    std::vector<Opcode> instructions;

    auto p = start;
    while(*p++ != ':');
    p++;
    regA = 0;
    unsigned char c;
    while((c = *p++) != '\n') {
        regA *= 10;
        regA += c-'0';
    }
    while(*p++ != ':');
    p++;
    regB = 0;
    while((c = *p++) != '\n') {
        regB*= 10;
        regB += c-'0';
    }
    while(*p++ != ':');
    p++;
    regC = 0;
    while((c = *p++) != '\n') {
        regC *= 10;
        regC += c-'0';
    }
    while(*p++ != ':');
    p++;
    while(p != end && (c = *p++) != '\n') {
        if (c == ',') {
            continue;
        }
        instructions.push_back(static_cast<Opcode>(c-'0'));
    }

    Machine machine(regA, regB, regC, instructions);

    return Input(machine);
}

std::string Input::run_machine() const {
    Machine copy(machine);
    return copy.run_machine();
}

int64_t Machine::combo(int address) const {
    char v = instructions[address];
    if (v <= 3) {
        return v;
    } else if (v == 4) {
        return regA;
    } else if (v == 5) {
        return regB;
    } else if (v == 6) {
        return regC;
    }
    std::unreachable();
}

std::string Machine::run_machine() {
    std::ostringstream output("");
    bool has_output = false;
    while(ip < instructions.size()-1) {
        switch(instructions[ip]) {
        case Opcode::Adv:
            // A = A/(2**combo)
            {
                int64_t operand = combo(ip+1);
                regA = regA / (1<<operand);
            }
            ip += 2;
            break;
        case Opcode::Bxl:
            // B = B ^ literal
            {
                int64_t literal = instructions[ip+1];
                regB = regB ^ literal;
            }
            ip += 2;
            break;
        case Opcode::Bst:
            // B = (combo) % 8
            {
                int64_t operand = combo(ip+1);
                regB = operand % 8;
            }
            ip += 2;
            break;
        case Opcode::Jnz:
            // if A != 0, IP = literal
            if (regA != 0) {
                ip = instructions[ip+1];
            } else {
                ip += 2;
            }
            break;
        case Opcode::Bxc:
            // B = B ^ C
            regB = regB ^ regC;
            ip += 2;
            break;
        case Opcode::Out:
            // output combo%8
            {
                int64_t operand = combo(ip+1);
                if (has_output) {
                    output << ',';
                }
                output << operand % 8;
                has_output = true;
            }
            ip += 2;
            break;
        case Opcode::Bdv:
            // B = A/(2**combo)
            {
                int64_t operand = combo(ip+1);
                regB = regA / (1<<operand);
            }
            ip += 2;
            break;
        case Opcode::Cdv:
            // C = A/(2**combo)
            {
                int64_t operand = combo(ip+1);
                regC = regA / (1<<operand);
            }
            ip += 2;
            break;
        default:
            std::unreachable();
        }
    }
    return output.str();
}