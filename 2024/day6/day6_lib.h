#include <vector>
#include <unordered_set>

enum Direction {
    Up,
    Down,
    Left,
    Right
};

Direction clockwise_direction(Direction d);

struct Point {
    int x,y;

    Point next(Direction d) const;

    bool operator==(const Point&) const = default;
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

    int count_steps() const;
    int count_opportunities() const;

    private:
    Point guard;
    Direction dir;
    int width, height;
    std::vector<Point> obstacles;

    Input(std::vector<Point> obstacles, Point guard, Direction dir, int width, int height): 
        guard(guard),
        dir(dir),
        width(width),
        height(height),
        obstacles(obstacles) {}

     bool will_loop(const std::unordered_set<Point> &obstacles, Point additional_obstacle, Point start, Direction d) const;
};