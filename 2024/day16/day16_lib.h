#include <vector>
#include <cstdint>
#include <tuple>

struct Point {
    int x,y;

    bool operator==(const Point & other) const {
        return other.x == x && other.y == y;
    }

    Point operator+(const Point & other) const {
        return Point {x+other.x, y+other.y};
    }
};

class Input {
    public:
    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t shortest_path() const;
    uint64_t shortest_paths() const;

    private:
    std::vector<std::vector<bool>> walls;
    Point start;
    Point end;

    Input(std::vector<std::vector<bool>> walls, Point start, Point end): walls(walls), start(start), end(end) {}
};