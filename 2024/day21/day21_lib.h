#include <vector>
#include <cstdint>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t score_codes() const;

    private:

    std::vector<std::vector<unsigned char>> codes;



    Input(std::vector<std::vector<unsigned char>> codes): codes(codes) {}
};