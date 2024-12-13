#include "day13_lib.h"

#include <tuple>
#include <assert.h>

#include <iostream>

Input Input::parse(const unsigned char* start, std::size_t length) {
    const unsigned char* end = start + length;

    std::vector<Machine> machines;

    auto p = start;
    while(p != end) {
        while(*p++ != '+');

        int64_t ax=0, ay=0, bx=0,by=0,px=0,py=0;
        auto c = *p++;
        while (c != ',') {
            ax *= 10;
            ax += c-'0';
            c = *p++;
        }

        assert(*p++ == ' ');
        assert(*p++ == 'Y');
        assert(*p++ == '+');
        c = *p++;
        while (c != '\n') {
            ay *= 10;
            ay += c-'0';
            c = *p++;
        }
        
        while(*p++ != '+');

        c = *p++;
        while (c != ',') {
            bx *= 10;
            bx += c-'0';
            c = *p++;
        }

        assert(*p++ == ' ');
        assert(*p++ == 'Y');
        assert(*p++ == '+');
        c = *p++;
        while (c != '\n') {
            by *= 10;
            by += c-'0';
            c = *p++;
        }

        while(*p++ != '=');
        c = *p++;
        while(c != ',') {
            px *= 10;
            px += c-'0';
            c = *p++;
        }
        assert(*p++ == ' ');
        assert(*p++ == 'Y');
        assert(*p++ == '=');

        c = *p++;
        while(c != '\n') {
            py *= 10;
            py += c-'0';
            c = *p++;
        }

        machines.emplace_back(ax, ay, bx, by, px, py);
    }

    return Input(machines);
}

std::tuple<int64_t, int64_t, int64_t> extended_euclidean(int64_t a, int64_t b) {
    int64_t rprev = std::max(a,b), rcur = std::min(a,b);
    int64_t sprev = 1, scur = 0;
    int64_t tprev = 0, tcur = 1;

    do {
        uint64_t q = rprev / rcur;
        uint64_t rnext = rprev - q * rcur;
        int64_t snext = sprev - q * scur;
        int64_t tnext = tprev - q * tcur;

        if (rnext == 0) {
            if (a < b) {
                std::swap(scur, tcur);
            }
            return std::make_tuple(rcur, scur, tcur);
        }
        rprev = rcur;
        rcur = rnext;
        sprev = scur;
        scur = snext;
        tprev = tcur;
        tcur = tnext;
    } while (1);
}

int64_t solve_machine(Machine machine, bool part2) {
    // we have a set of equations
    // Na * ax + Nb * bx = px
    // Na * ay + Nb * by = py
    // 0 < Na <= 100
    // 0 < Nb <= 100
    // and we want to minimize 3*Na + Nb

    if (part2) {
        machine.px += 10000000000000;
        machine.py += 10000000000000;
    }

    int64_t gcdx;
    int64_t sx,tx;
    std::tie(gcdx,sx,tx) = extended_euclidean(machine.ax, machine.bx);
    //std::cout << "x:" << gcdx << "," << sx << "," << tx << std::endl;
    if (machine.px % gcdx != 0 ) {
        return 0;
    }

    int64_t gcdy;
    int64_t sy,ty;
    std::tie(gcdy,sy,ty) = extended_euclidean(machine.ay, machine.by);
    //std::cout << "y:" << gcdy << "," << sy << "," << ty << std::endl;
    if (machine.py % gcdy != 0) {
        return 0;
    }

    // If we are here, then there exist integer solutions,
    // but not necessarily positive.

    // from the euclidean algorithm we have
    // sx * ax + tx * bx = gcdx
    // so Na = sx * (px/gcdx), Nb = tx * (px/gcdx)

    // however this may involve Na or Nb being negative
    // other solutions have the form 
    // Na = sx * (px/gcdx) + k * bxr
    // Nb = tx * (px/gcdx) − k * axr

    // Combining both equations together,
    // (sx * (px/gcdx) + k * bxr) * ay + (tx * (px/gcdx) - k * axr) * by = py
    // k * (bxr * ay - axr * by) + (px/gcdx) * (sx * ay + tx * by) = py
    // k = (py - px/gcdx * (sx * ay + tx * by)) / (bxr * ay - axr * by)

    int64_t axr = machine.ax / gcdx;
    int64_t bxr = machine.bx / gcdx;

    int64_t numerator = machine.py - (machine.px / gcdx) * (sx * machine.ay + tx * machine.by);
    int64_t denominator = bxr * machine.ay - axr * machine.by;
    int64_t k = numerator/denominator;

    auto apress = (sx * machine.px/gcdx + k * bxr);
    auto bpress =  (tx * machine.px/gcdx - k * axr);

    if (apress < 0 || bpress < 0 || (!part2 && (apress >= 100 || bpress >= 100))
        || apress * machine.ay + bpress * machine.by != machine.py) {
        return 0;
    }
    //std::cout << apress << "," << bpress << std::endl;

    return 3 * apress + bpress;


}

uint64_t Input::solve_tokens(bool part2) const {
    uint64_t result = 0;

    for(auto &machine : machines) {
        result += solve_machine(machine, part2);
    }

    return result;
}