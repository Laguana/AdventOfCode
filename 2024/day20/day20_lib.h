#include <vector>
#include <cstdint>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    private:

    Input() {}
};