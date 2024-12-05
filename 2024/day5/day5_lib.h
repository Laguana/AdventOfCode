
#include <vector>
#include <tuple>

class Input {
    public:

    static Input parse(const unsigned char* start, const unsigned char* end);

    int get_ordered_midpoints() const;

    int get_reordered_midpoints() const;

    private:
    Input(std::vector<std::tuple<int,int>> constraints, std::vector<std::vector<int>> updates): constraints(constraints), updates(updates) {
        for (const auto &[left, right] : constraints) {
            before[left][right] = true;
        }
    }

    int reorder_and_get_new_midpoint(std::vector<int> row) const;

    std::vector<std::tuple<int,int>> constraints;
    std::vector<std::vector<int>> updates;

    bool before[100][100] = {0};
};