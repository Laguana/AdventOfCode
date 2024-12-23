#include <vector>
#include <cstdint>
#include <utility>

#include <unordered_map>
#include <unordered_set>
#include <string>

class Input {
    public:

    static Input parse(const unsigned char* start, std::size_t len);

    int count_t_cliques() const;

    std::unordered_set<int> get_biggest_connected_component() const;

    std::string get_password() const;

    private:
    std::unordered_map<int, std::unordered_set<int>> connections;

    void BronKerbosh(std::unordered_set<int> & result, std::unordered_set<int> & R, std::unordered_set<int> &P, std::unordered_set<int> & X) const;

    Input(std::unordered_map<int, std::unordered_set<int>> connections): connections(connections) {}
};