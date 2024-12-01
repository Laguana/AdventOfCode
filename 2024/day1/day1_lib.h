#include <vector>
#include <tuple>
#include <istream>
#include <cstdint>

std::tuple<std::vector<int>, std::vector<int>> parse_input(std::basic_istream<char> &input);

void sort_list(std::vector<int> &v);

uint64_t compute_distance(std::vector<int> &left, std::vector<int> &right);

uint64_t compute_similarity(std::vector<int> &left, std::vector<int> &right);