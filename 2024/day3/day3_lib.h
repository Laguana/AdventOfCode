
#include <cstdint>
#include <optional>


class Instruction {
    public:
    enum Kind {
        Mul,
        Do,
        Dont
    };

    uint64_t product() const {
        return left*right;
    }

    Kind get_kind() const {
        return kind;
    }

    static std::optional<Instruction> parse(const char** input, const char* end);

    private:

    uint16_t left,right;
    Kind kind;

    Instruction(uint16_t left, uint16_t right): left(left), right(right), kind(Mul) {}
    Instruction(Kind kind): left(0), right(0), kind(kind) {}
};

uint64_t sum_products(const char* start, const char* end);
uint64_t sum_enabled_products(const char* start, const char* end);