:- load_files(['../aoc_lib.pl']).

listing(race(Times, Distances)) --> "Time:", numbers(Times), eol, "Distance:", numbers(Distances).
numbers([H|T]) --> spaces, number(H), (numbers(T) | {T=[]}).

% for a race of length T, the distance travelled is C*(T-C) for a charge time of C
% This is CT-C^2
% (-C^2 + CT - D = 0) has solutions at -T +/- sqrt(T^2-4D)/(-2)
% = (T +/- sqrt(T^2-4D))/2
% x1 = (T-sqrt(Z))/2
% x2 = (T+sqrt(Z))/2
% The number of integral values beween these two points is approximately floor(x2)-ceil(x1)+1
% Unless the sqrt is an integer, in which case the 0s will be integers, and that's when you equal
% distance so you want to discount both of those and it is 2 fewer.
ways_won(Time, Distance, Ways) :-
    sqrt(Time * Time - 4 * Distance, Sqrt),
    Low is ceil((Time - Sqrt)/2),
    High is floor((Time + Sqrt)/2),
    ISqrt is float_integer_part(Sqrt),
    (Sqrt = ISqrt -> Ways is High-Low-1; Ways is High-Low+1).

distance_travelled(Time, Charge, Distance) :-
    Distance is Charge * (Time-Charge).

prod_list([], 1).
prod_list([H|T], Out) :- prod_list(T, TOut), Out is H * TOut.

part1(race(Times, Distance), Out) :-
    maplist(ways_won, Times, Distance, Ways),
    prod_list(Ways, Out).

glue_into_one_number(L, N) :-
    maplist(number_chars, L, LChars),
    append(LChars, Chars),
    number_chars(N, Chars).

part2(race(Times, Distances), Out) :-
    glue_into_one_number(Times, Time),
    glue_into_one_number(Distances, Distance),
    ways_won(Time, Distance, Out).

:- phrase_from_file(listing(L), "day6.example.in"), part1(L, 288).

:- phrase_from_file(listing(L), "day6.in"), part1(L, 1155175).%Out), print(Out).

:- phrase_from_file(listing(L), "day6.example.in"), part2(L, 71503).

:- phrase_from_file(listing(L), "day6.in"), part2(L, 35961505).%Out), print(Out).