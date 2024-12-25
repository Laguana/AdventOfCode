#include <vector>
#include <cstdint>

#include <unordered_map>
#include <string>
#include <utility>

enum Op {
    AND,
    OR,
    XOR
};

struct Operation {
    uint32_t in1, in2;
    uint32_t out;
    Op op;

    bool execute(bool left, bool right) {
        switch(op) {
        case Op::AND:
            return left && right;
        case Op::OR:
            return left || right;
        case Op::XOR:
            return left != right;
        default:
            std::unreachable();
        }
    }

    Operation(uint32_t in1, uint32_t in2, uint32_t out, Op op):
        in1(in1), in2(in2), out(out), op(op) {}
    
    Operation() {};
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t get_z_number() const;

    std::string get_swapped_wires() const;

    private:
    static uint32_t key_from_string(const std::string_view & s);

    std::unordered_map<uint32_t, bool> values;
    std::vector<Operation> operations;
    unsigned int max_z;

    Input(std::unordered_map<uint32_t, bool> values, std::vector<Operation> operations, int max_z):
        values(values), operations(operations), max_z(max_z) {}
};