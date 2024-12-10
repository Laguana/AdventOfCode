#include <vector>

struct Point {
    int x, y;

    bool operator==(const Point &other) const {
        return this->x == other.x && this->y == other.y;
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

    static Input parse(const unsigned char* start, const unsigned char* end);

    int score_trailheads() const;
    int rate_trailheads() const;

    private:
    std::vector<std::vector<int>> map;
    std::vector<Point> origins;
    std::vector<Point> endpoints;

    Input(
        std::vector<std::vector<int>> map,
        std::vector<Point> origins,
        std::vector<Point> endpoints):
        map(map),
        origins(origins),
        endpoints(endpoints) {}
};