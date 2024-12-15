#include <vector>
#include <cstdint>

enum Instruction {
    Up,
    Down,
    Left,
    Right
};

enum Entity {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t run_instructions_and_score() const;
    uint64_t run_instructions_on_double_wide() const;

    private:

    int botx, boty;
    std::vector<std::vector<Entity>> map;
    std::vector<Instruction> instructions;

    Input(int botx, int boty,
        std::vector<std::vector<Entity>> map,
        std::vector<Instruction> instructions):
        botx(botx),
        boty(boty),
        map(map),
        instructions(instructions) {}
};