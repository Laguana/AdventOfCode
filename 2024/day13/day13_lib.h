#include <cstdint>
#include <vector>

struct Machine {
    int64_t ax, ay;
    int64_t bx, by;
    int64_t px, py;

    Machine(
        int64_t ax,
        int64_t ay,
        int64_t bx,
        int64_t by,
        int64_t px,
        int64_t py):
        ax(ax), ay(ay), bx(bx), by(by), px(px), py(py) {}
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t length);

    uint64_t solve_tokens(bool part2 = false) const;

    private:

    std::vector<Machine> machines;

    Input(std::vector<Machine> machines): machines(machines) {}
};