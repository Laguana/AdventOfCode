#include <vector>
#include <cstdint>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t count_stones(int steps) const;
    uint64_t count_stones_better(int steps) const;

    private:

    std::vector<uint64_t> stones;

    Input(std::vector<uint64_t> stones): stones(stones) {}
};