#include <vector>
#include <cstdint>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    int matching_combinations() const;

    private:

    std::vector<std::vector<int>> locks;
    std::vector<std::vector<int>> keys;

    Input(std::vector<std::vector<int>> locks, std::vector<std::vector<int>> keys): locks(locks), keys(keys) {}
};