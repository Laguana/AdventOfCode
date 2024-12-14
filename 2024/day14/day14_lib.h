#include <cstdint>
#include <vector>

struct Bot {
    int64_t x, y;
    int64_t vx, vy;

    Bot(int64_t x, int64_t y, int64_t vx, int64_t vy): x(x), y(y), vx(vx), vy(vy) {}
};

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t length);

    int mult_quadrants(int steps, int width, int height) const;

    void execute_until_stop(int width, int height);
    void print_at(int steps, int width, int height) const;

    private:

    std::vector<Bot> bots;

    Input(std::vector<Bot> bots): bots(bots) {}
};