CFLAGS = -Wall -Wextra -Wpedantic -std=c++20 -g -O2
CXXFLAGS = -Wall -Wextra -Wpedantic -std=c++20 -g -O2
LINK.o = $(LINK.cc)

%.in.h: %.input
	xxd -i $< > $@

day1/day1.o: day1/day1.cc day1/day1.in.h

day1/day1: day1/day1.o day1/day1_lib.o

day1/day1_test.o: day1/day1_test.cc day1/day1.in.h

day1/day1_test: day1/day1_test.o day1/day1_lib.o

day2/day2_test: day2/day2_test.o day2/day2_lib.o

day2/day2.o: day2/day2.cc day2/day2.in.h

day2/day2_test.o: day2/day2_test.cc day2/day2.in.h

day2/day2: day2/day2.o day2/day2_lib.o

.PHONY: all

all: day1/day1 day1/day1_test day2/day2 day2/day2_test