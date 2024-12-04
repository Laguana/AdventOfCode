
class Input {
    public:

    char get(int x, int y) const;

    int count_xmas() const;
    int count_x_mas() const;

    static Input parse(const char* start, const char* end);

    private:
    const char *start, *end;
    int width, height;

    Input(const char* start, const char* end, int width, int height):
        start(start),
        end(end),
        width(width),
        height(height) {}
};