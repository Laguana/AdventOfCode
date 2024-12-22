#include <vector>
#include <cstdint>
#include <utility>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    int64_t sum_evolved() const;
    int64_t buy_bananas() const;

    private:

    std::vector<int64_t> secrets;

    Input(std::vector<int64_t> secrets): secrets(secrets) {}
};