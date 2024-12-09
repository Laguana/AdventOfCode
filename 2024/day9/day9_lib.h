
#include <vector>
#include <cstdint>

class Input {
    public:

    static Input parse(const unsigned char* start, const unsigned char* end);

    uint64_t defrag_and_checksum() const;

    uint64_t defrag_blocks_and_checksum() const ;

    private:

    std::vector<int> disk;
    std::vector<std::vector<std::size_t>> gaps;

    Input(std::vector<int> disk, std::vector<std::vector<std::size_t>> gaps): disk(disk), gaps(gaps) {}

};