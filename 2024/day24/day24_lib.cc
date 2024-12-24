#include "day24_lib.h"

#include <iostream>
#include <utility>
#include <queue>

Input Input::parse(const unsigned char* start, std::size_t len) {
    auto end = start+len;
    std::unordered_map<uint32_t, bool> values;
    std::vector<Operation> operations;
    int max_z = 0;

    auto p = start;
    while(*p != '\n') {
        auto key = key_from_string(std::string_view((const char*)p,3));
        auto value = *(p+5) == '1';
        values[key] = value;
        p += 7;
    }
    p++;

    while(p != end) {
        auto key1 = key_from_string(std::string_view((const char*)p, 3));
        p += 4;
        Op op;
        switch(*p){
        case 'A':
            op = Op::AND;
            p += 4;
            break;
        case 'X':
            op = Op::XOR;
            p += 4;
            break;
        case 'O':
            op = Op::OR;
            p += 3;
            break;
        default:
            std::unreachable();
        }
        auto key2 = key_from_string(std::string_view((const char*)p, 3));
        p += 7;
        auto key3 = key_from_string(std::string_view((const char*)p, 3));
        if (*p == 'z') {
            int z = (p[1]-'0') * 10 + p[2]-'0';
            if (z > max_z) {
                max_z = z;
            }
        }

        p += 4;

        operations.emplace_back(key1, key2, key3, op);
    }

    return Input(values, operations, max_z);
}

uint32_t Input::key_from_string(const std::string_view & s) {
    uint32_t ret = s[0] << 16 | s[1] << 8 | s[2];
    //std::cout << s << "->" << ret << std::endl;
    return ret;
}


uint64_t Input::get_z_number() const {
    uint64_t z_number = 0;

    auto final_values = values;

    // Build a lookup from input to operations;
    std::unordered_map<uint64_t, std::vector<Operation>> keyed_operations;
    for(auto &op: operations) {
        keyed_operations[op.in1].push_back(op);
    }

    std::queue<uint64_t> to_consider;
    for(auto &[v,_]: values) {
        to_consider.push(v);
    }

    while(!to_consider.empty()) {
        auto e = to_consider.front();
        to_consider.pop();

        auto &ops = keyed_operations[e];

        for(auto it = ops.begin(); it != ops.end();) {
            auto & op = *it;
            if (op.in2 == e || final_values.contains(op.in2)) {
                auto v1 = final_values[op.in1];
                auto v2 = final_values[op.in2];
                auto result = op.execute(v1, v2);
                final_values[op.out] = result;
                to_consider.push(op.out);
            } else {
                keyed_operations[op.in2].push_back(op);
            }
            it = ops.erase(it);
        }
    }

    std::cout << keyed_operations.size() << std::endl;


    char key_string[3] = {'z','0','0'};
    for(uint64_t i = 0; i <= max_z; ++i) {
        key_string[1] = '0' + (i/10);
        key_string[2] = '0' + (i%10);
        auto key = key_from_string(std::string_view(key_string, 3));
        auto value = final_values.find(key);
        if (value == final_values.end()) {
            std::cout << "Didn't set bit " << i << "?" << std::endl;
            return 0;
        } else {
            if (value->second) {
                //std::cout << std::string_view(key_string, 3) << " is true?" << std::endl;
                z_number = z_number | ((uint64_t) 1) << i;
            }
        }
    }

    return z_number;
}

std::string Input::get_swapped_wires() const {
    // work forwards? see what wires z00 depends on, then move forwards in the expected way?
    // work backwards and try to automatically disassemble the input?

    // or... just do it myself?

    // dsr is the carry of x00+y00
    // hqh is the carry of x01+y01
    // nmk is x01+y01 ignoring carry of x00+y00

    // in fact, xAA AND yAA -> the immediate carry  iAA
    //          XAA XOR yAA -> the sum ignoring carry
    // so if we find all the carries, and all the sums, we should see
    // xN AND yN -> iN
    // xN XOR yN -> sN [for N = 0 sN = z00]
    // cN-1 AND sN -> dN for carry caused by lower carry
    // iN OR dN -> cN [for N = 0, cN = iN]
    // sN XOR cN-1 -> zN [for N > 0]

    // it's simple enough to walk through doing this, but how do you detect issues?
    // I ended up doing it by hand, and it looks likle if you get a zN where you expected
    // an intermediate result (or vice versa; a non zN where you should have had one) then
    // it's clearly wrong, and you can work out where the zn should be by finding the relevant iN/sN to sub in
    // Also because inputs are never wrong, 
    // you know that a OR b must be computing a cN from iN and dN,
    // a AND b is either computing iN if it is x/y or dN otherwise and its inputs are cN-1 and sN
    // a XOR b is intended to compute sN if it is x/y, or it is computing zN from sN and cN-1
    // whenever there was a discrepancy, this let me (manually) identify the necessary swaps

    std::unordered_map<uint64_t, std::vector<Operation>> keyed_operations;
    for(auto &op: operations) {
        keyed_operations[op.in1].push_back(op);
        keyed_operations[op.in2].push_back(op);
    }

    uint64_t carries[max_z+1] = {0};
    uint64_t immediate_carries[max_z+1] = {0};
    uint64_t sums[max_z+1] = {0};
    uint64_t lower_carry[max_z+1] = {0};

    std::vector<std::pair<uint64_t, uint64_t>> swaps;

    char x_string[3] = {'x','0','0'};
    char y_string[3] = {'y','0','0'};
    char z_string[3] = {'z','0','0'};
    for(uint64_t i = 0; i <= max_z; ++i) {
        x_string[1] = '0' + i/10;
        x_string[2] = '0' + (i%10);
        auto x_key = key_from_string(std::string_view(x_string, 3));
        y_string[1] = '0' + i/10;
        y_string[2] = '0' + (i%10);
        auto y_key = key_from_string(std::string_view(y_string, 3));
        
        auto x_operations = keyed_operations[x_key];
        for(const auto &op : x_operations) {
            if (op.op == Op::AND) {
                if ((op.in1 == x_key && op.in2 == y_key) ||
                    (op.in1 == y_key && op.in2 == x_key)) {
                    sums[i] = op.out;
                    if (i == 0) {
                        carries[i] = op.out;
                    }
                } else {
                    std::cout << "Invalid structure" << std::endl;
                    std::abort();
                }
            } else if (op.op == Op::XOR) {
                if ((op.in1 == x_key && op.in2 == y_key) ||
                    (op.in1 == y_key && op.in2 == x_key)) {
                    immediate_carries[i] = op.out;
                    if (i == 0) {
                        carries[i] = op.out;
                    }
                } else {
                    std::cout << "Invalid structure" << std::endl;
                    std::abort();
                }
            }
        }

        if (i > 0) {

        }

    }

    return "";
}