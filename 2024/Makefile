CFLAGS = -Wall -Wextra -Wpedantic -std=c++23 -g -O2
CXXFLAGS = -Wall -Wextra -Wpedantic -std=c++23 -g -O2
LINK.o = $(LINK.cc)

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

.PHONY: all day1 day2 day3


all: day1 day2 day3