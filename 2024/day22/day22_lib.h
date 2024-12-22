#include <vector>
#include <cstdint>
#include <utility>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    private:

    Input() {}
};