#include <unordered_map>
#include <vector>
#include <unordered_set>

struct Point {
    int x,y;

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

    static Input parse(const unsigned char *start, const unsigned char* end);

    int count_anodes(bool pt2 = false) const;

    
    private:
    int width, height;

    using cviter = std::vector<Point>::const_iterator;
    void add_anodes(bool pt2, std::unordered_set<Point> &set, cviter current, cviter end) const;

    using AntennaMap = std::unordered_map<unsigned char, std::vector<Point>>;

    AntennaMap antennae;

    Input(int width, int height, AntennaMap antennae): width(width), height(height), antennae(antennae) {}

};