CFLAGS = -Wall -std=c++20 -g
CXXFLAGS = -Wall -std=c++20 -g
LINK.o = $(LINK.cc)

%.in.h: %.input
	xxd -i $< > $@

day1/day1.o: day1/day1.cc day1/day1.in.h

day1/day1: day1/day1.o day1/day1_lib.o

day1/day1_test.o: day1/day1_test.cc day1/day1.in.h

day1/day1_test: day1/day1_test.o day1/day1_lib.o

.PHONY: all

all: day1/day1 day1/day1_test