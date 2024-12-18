#include <vector>
#include <cstdint>

struct Point {
    int x,y;

    Point operator+(const Point& other) const {
        return {x+other.x, y+other.y};
    }

    bool operator==(const Point& other) const {
        return x == other.x && y == other.y;
    }
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t shortest_path(int size, unsigned int horizon) const;

    Point find_cutoff(int size) const;

    private:

    std::vector<Point> blocks;

    Input(std::vector<Point> blocks): blocks(blocks) {}
};