#include <istream>
#include <vector>

using day2_input = std::vector<std::vector<int>>;

day2_input parse_input(std::basic_istream<char> &input);

bool is_safe(const std::vector<int> &row);

int count_safe(const day2_input &input);

bool is_damped_safe(const std::vector<int> &row);

int count_damped_safe(const day2_input &input);