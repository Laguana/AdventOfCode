CFLAGS = -Wall -Wextra -Wpedantic -std=c++23 -g -O2
CXXFLAGS = -Wall -Wextra -Wpedantic -std=c++23 -g -O2 -fno-exceptions
LINK.o = $(LINK.cc)
CXX=g++-14

%.in.h: %.input
	xxd -i $< > $@

.DEFAULT_GOAL := all

day1/day1.o: day1/day1.cc day1/day1.in.h

day1/day1: day1/day1.o day1/day1_lib.o

day1/day1_test.o: day1/day1_test.cc day1/day1.in.h

day1/day1_test: day1/day1_test.o day1/day1_lib.o

day1: day1/day1 day1/day1_test

day2/day2_test: day2/day2_test.o day2/day2_lib.o

day2/day2.o: day2/day2.cc day2/day2.in.h

day2/day2_test.o: day2/day2_test.cc day2/day2.in.h

day2/day2: day2/day2.o day2/day2_lib.o

day2: day2/day2 day2/day2_test

day3/day3: day3/day3.o day3/day3_lib.o

day3/day3_test: day3/day3_test.o day3/day3_lib.o

day3/day3.o: day3/day3.cc day3/day3.in.h

day3/day3_test.o: day3/day3_test.cc day3/day3.in.h

day3: day3/day3 day3/day3_test

day4/day4.o: day4/day4.cc day4/day4.in.h

day4/day4: day4/day4.o day4/day4_lib.o

day4/day4_test.o: day4/day4_test.cc day4/day4.in.h

day4/day4_test: day4/day4_test.o day4/day4_lib.o

day4: day4/day4 day4/day4_test

day5/day5.o: day5/day5.cc day5/day5.in.h

day5/day5: day5/day5.o day5/day5_lib.o

day5/day5_test.o: day5/day5_test.cc day5/day5.in.h

day5/day5_test: day5/day5_test.o day5/day5_lib.o

day5: day5/day5 day5/day5_test

day6/day6.o: day6/day6.cc day6/day6.in.h

day6/day6: day6/day6.o day6/day6_lib.o

day6/day6_test.o: day6/day6_test.cc day6/day6.in.h

day6/day6_test: day6/day6_test.o day6/day6_lib.o

day6: day6/day6 day6/day6_test

day7/day7.o: day7/day7.cc day7/day7.in.h

day7/day7: day7/day7.o day7/day7_lib.o

day7/day7_test.o: day7/day7_test.cc day7/day7.in.h

day7/day7_test: day7/day7_test.o day7/day7_lib.o

day7: day7/day7 day7/day7_test

day8/day8.o: day8/day8.cc day8/day8.in.h

day8/day8: day8/day8.o day8/day8_lib.o

day8/day8_test.o: day8/day8_test.cc day8/day8.in.h

day8/day8_test: day8/day8_test.o day8/day8_lib.o

day8: day8/day8 day8/day8_test

day9/day9.o: day9/day9.cc day9/day9.in.h

day9/day9: day9/day9.o day9/day9_lib.o

day9/day9_test.o: day9/day9_test.cc day9/day9.in.h

day9/day9_test: day9/day9_test.o day9/day9_lib.o

day9: day9/day9 day9/day9_test

day10/day10.o: day10/day10.cc day10/day10.in.h

day10/day10: day10/day10.o day10/day10_lib.o

day10/day10_test.o: day10/day10_test.cc day10/day10.in.h

day10/day10_test: day10/day10_test.o day10/day10_lib.o

day10: day10/day10 day10/day10_test

day11/day11.o: day11/day11.cc day11/day11.in.h

day11/day11: day11/day11.o day11/day11_lib.o

day11/day11_test.o: day11/day11_test.cc day11/day11.in.h

day11/day11_test: day11/day11_test.o day11/day11_lib.o

day11: day11/day11 day11/day11_test

day12/day12.o: day12/day12.cc day12/day12.in.h

day12/day12: day12/day12.o day12/day12_lib.o

day12/day12_test.o: day12/day12_test.cc day12/day12.in.h

day12/day12_test: day12/day12_test.o day12/day12_lib.o

day12: day12/day12 day12/day12_test

day13/day13.o: day13/day13.cc day13/day13.in.h

day13/day13: day13/day13.o day13/day13_lib.o

day13/day13_test.o: day13/day13_test.cc day13/day13.in.h

day13/day13_test: day13/day13_test.o day13/day13_lib.o

day13: day13/day13 day13/day13_test

day14/day14.o: day14/day14.cc day14/day14.in.h

day14/day14: day14/day14.o day14/day14_lib.o

day14/day14_test.o: day14/day14_test.cc day14/day14.in.h

day14/day14_test: day14/day14_test.o day14/day14_lib.o

day14: day14/day14 day14/day14_test

day15/day15.o: day15/day15.cc day15/day15.in.h

day15/day15: day15/day15.o day15/day15_lib.o

day15/day15_test.o: day15/day15_test.cc day15/day15.in.h

day15/day15_test: day15/day15_test.o day15/day15_lib.o

day15: day15/day15 day15/day15_test

day16/day16.o: day16/day16.cc day16/day16.in.h

day16/day16: day16/day16.o day16/day16_lib.o

day16/day16_test.o: day16/day16_test.cc day16/day16.in.h

day16/day16_test: day16/day16_test.o day16/day16_lib.o

day16: day16/day16 day16/day16_test

day17/day17.o: day17/day17.cc day17/day17.in.h

day17/day17: day17/day17.o day17/day17_lib.o

day17/day17_test.o: day17/day17_test.cc day17/day17.in.h

day17/day17_test: day17/day17_test.o day17/day17_lib.o

day17: day17/day17 day17/day17_test

day18/day18.o: day18/day18.cc day18/day18.in.h

day18/day18: day18/day18.o day18/day18_lib.o

day18/day18_test.o: day18/day18_test.cc day18/day18.in.h

day18/day18_test: day18/day18_test.o day18/day18_lib.o

day18: day18/day18 day18/day18_test

day19/day19.o: day19/day19.cc day19/day19.in.h

day19/day19: day19/day19.o day19/day19_lib.o

day19/day19_test.o: day19/day19_test.cc day19/day19.in.h

day19/day19_test: day19/day19_test.o day19/day19_lib.o

day19: day19/day19 day19/day19_test

day20/day20.o: day20/day20.cc day20/day20.in.h day20/day20_lib.h

day20/day20: day20/day20.o day20/day20_lib.o

day20/day20_test.o: day20/day20_test.cc day20/day20.in.h day20/day20_lib.h

day20/day20_test: day20/day20_test.o day20/day20_lib.o

day20: day20/day20 day20/day20_test

day21/day21.o: day21/day21.cc day21/day21.in.h day21/day21_lib.h

day21/day21: day21/day21.o day21/day21_lib.o

day21/day21_test.o: day21/day21_test.cc day21/day21.in.h day21/day21_lib.h

day21/day21_test: day21/day21_test.o day21/day21_lib.o

day21: day21/day21 day21/day21_test

day22/day22.o: day22/day22.cc day22/day22.in.h day22/day22_lib.h

day22/day22: day22/day22.o day22/day22_lib.o

day22/day22_test.o: day22/day22_test.cc day22/day22.in.h day22/day22_lib.h

day22/day22_test: day22/day22_test.o day22/day22_lib.o

day22: day22/day22 day22/day22_test

.PHONY: all day1 day2 day3 day4 day5 day6 day7 day8 day9 day10 day11 day12 day13 day14 day15 day16 day17 day18 day19 day20 day21 day22

all: day1 day2 day3 day4 day5 day6 day7 day8 day9 day10 day11 day12 day13 day14 day15 day16 day17 day18 day19 day20 day21 day22