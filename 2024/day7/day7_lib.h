#include <vector>
#include <cstdint>

class Input {
    public:

    static Input parse(const unsigned char* start, const unsigned char* end);

    uint64_t sum_workable(bool allow_concat = false) const;

    private:

    Input(std::vector<std::vector<uint64_t>> rows): rows(rows) {}

    std::vector<std::vector<uint64_t>> rows;
};