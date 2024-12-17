#include "day17_lib.h"
#include "day17.in.h"

#include <iostream>

int main() {
    auto input = Input::parse(day17_day17_input, day17_day17_input_len);

    std::cout << "Part 1: " << input.run_machine() << std::endl;

    // The whole program is a straight line loop
    // The instructions are
    // Bst, 4 
    // Bxl, 1
    // Cdv, 5 
    // Bxl, 5 
    // Bxc, 3
    // Adv, 3
    // Out, 5
    // Jnz, 0

    // subbing in opcode logic
    // Bst regA
    // Bxl 1
    // Cdv RegB
    // Bxl 5
    // Bxc {ignored}
    // Adv 3
    // Out RegB
    // Jnz 0

    // Expanding somewhat
    // B = A%8
    // B = B^1
    // C = A >> B
    // B = B^5
    // B = B ^ C
    // A = A >> 3
    // Out B%8
    // Jnz 0

    // This is roughly 
    // C = A >> (low 3 bits of A, lowest bit switched)
    // B = (low 3 bits of A, highest bit switched) ^ C
    // Shift A by 3 bits
    // print 3 bits of B
    // repeat while A has bits

    // Note that only the low 3 bits of C matter,
    // and they come from discarding between 0 and 7 bits of A
    // so each iteration only considers 10 bits of A
    // We could just.... make a lookup table of that.

    std::vector<Opcode> instructions;
    for (char o : {2,4,1,1,7,5,1,5,4,3,0,3,5,5,3,0}) {
        instructions.push_back(static_cast<Opcode>(o));
    }
    //for(int i = 0; i < 1024; ++i) {
    //    Machine m(i,0,0, static_cast<std::vector<Opcode>>(instructions));
    //    std::cout << i << ": " << m.run_machine() << std::endl;
    //}

    // ending in 3,0 could be
    //   37: 3,0
    //   39: 3,0
    //  296: 0,3,0
    //  297: 4,3,0
    //  298: 3,3,0
    //  299: 5,3,0
    //  300: 1,3,0
    //  301: 3,3,0
    //  302: 0,3,0
    //  303: 7,3,0
    //  312: 0,3,0
    //  313: 4,3,0
    //  314: 1,3,0
    //  315: 1,3,0
    //  316: 1,3,0
    //  317: 2,3,0
    //  318: 0,3,0
    //  319: 7,3,0

    // These are the final bits with no further iterations,
    // and so it must be that the high bits of the number
    // are o45 or o47
    
    // from the other diretion, in order to get 2,4
    //   o16: 2,4
    //  o114: 2,4,4
    //  o116: 2,4,4
    //  o117: 2,4,4
    //  o514: 2,4,1
    // o1114: 2,4,4,4
    // o1117: 2,4,4,4
    // o1514: 2,4,1,4
    // We need a continuation here, so there must be high
    // bits to continue with, so it must be o514

    // after we pop the low bits to get the 2,
    // these make 4,1 with low bits o51
    //  o51: 4,1
    // o151: 4,1,4
    // if we pop the 1 off for the 4, we now need 1,1,7
    // while ending in o5
    // which is none of them...

    // Is this logic even sound?
    // it looks like it must end in o14 at least, o17 has no continuations

    // How about I just try to solve 3 bits at a time, using a search of ~15 bits,
    // and if they all end in the same 3 bits then that's the answer, and if they
    // end in different bits, then expand the search a bit

    // by doing this, i found o04532356064474665 is an answer
    // but... it's too high apparently?

    // ah, o04532306073267275 ends in higher digits but that lets it have
    // smaller digits in higher places. That's the answer.

    /*
    for(uint64_t i = 0; i < 1<<21; ++i) {
        Machine m(i << 30 | 04532306073267275, 0,0, static_cast<std::vector<Opcode>>(instructions));
        auto result = m.run_machine();
        //                      2,4,1,1,7,5,1,5,4,3,0,3,5,5,3,0
        if (result.starts_with("2,4,1,1,7,5,1,5,4,3,0,3")){
            //std::cout << i << std::endl;
            std::cout << std::oct << i  << std::dec<< ": " << result << std::endl;
        }
    }
    */
    
    


    std::cout << "Part 2: " << 04532306073267275 << std::endl;

    return 0;
}