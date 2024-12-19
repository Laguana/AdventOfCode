#include <vector>
#include <cstdint>

#include <string>

class Input {
    public:

    static Input parse(const unsigned char *start, std::size_t len);

    int count_possible() const;
    uint64_t count_how_possible() const;

    private:

    std::vector<std::string> available;
    std::vector<std::string> desired;

    Input(std::vector<std::string> available, std::vector<std::string> desired): available(available), desired(desired) {}
};