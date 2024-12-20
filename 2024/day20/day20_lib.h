#include <vector>
#include <cstdint>

#include <unordered_map>

struct Point {
    int x, y;

    bool operator==(const Point& other) const {
        return x == other.x && y == other.y;
    }

    Point operator+(const Point& other) const {
        return {x+other.x, y+other.y};
    }
};

template<>
struct std::hash<Point>
{
    std::size_t operator()(const Point& s) const noexcept
    {
        std::size_t h1 = std::hash<int>{}(s.x);
        std::size_t h2 = std::hash<int>{}(s.y);
        return h1 ^ (h2 << 8);
    }
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    uint64_t count_big_skips(unsigned int threshold) const;
    uint64_t count_bigger_skips(unsigned int threshold) const;

    private:
    Point startPoint, endPoint;
    std::vector<std::vector<bool>> walls;

    std::unordered_map<Point, uint64_t> distances;

    Input(Point startPoint,
          Point endPoint,
          std::vector<std::vector<bool>> walls,
          std::unordered_map<Point, uint64_t> distances):
        startPoint(startPoint), endPoint(endPoint), walls(walls), distances(distances)
        {}
};