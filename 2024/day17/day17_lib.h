#include <vector>
#include <cstdint>

#include <string>

enum Opcode: char {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
};

struct Machine {
    
    int64_t regA, regB, regC;
    unsigned int ip;

    const std::vector<Opcode> instructions;

    std::string run_machine();

    Machine(int64_t regA, int64_t regB, int64_t regC, const std::vector<Opcode> &instructions):
        regA(regA), regB(regB), regC(regC), ip(0), instructions(instructions) {}

    private:

    int64_t combo(int address) const;
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    std::string run_machine() const;

    uint64_t find_quine() const;

    private:
    Machine machine;

    Input(Machine machine): machine(machine) {}
};