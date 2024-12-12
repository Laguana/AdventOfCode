#include <vector>
#include <cstdint>
#include <tuple>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t cost_field(bool discounted = false) const;

    private:

    std::vector<std::vector<char>> grid;

    Input(std::vector<std::vector<char>> grid): grid(grid) {}
};
