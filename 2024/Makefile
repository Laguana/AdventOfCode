CFLAGS = -Wall -Wextra -Wpedantic -std=c++23 -g -O2
CXXFLAGS = -Wall -Wextra -Wpedantic -std=c++23 -g -O2 -fno-exceptions
LINK.o = $(LINK.cc)
CXX=g++-14

%.in.h: %.input
	xxd -i $< > $@

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

.PHONY: all day1 day2 day3 day4 day4


all: day1 day2 day3 day4 day5