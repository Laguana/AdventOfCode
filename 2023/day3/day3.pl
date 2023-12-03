:- load_files(['../aoc_lib.pl']).

grid([H|T]) --> grid_line(H, 0), (call(eos), !, {T=[]} | grid(T)).
grid_line(L, X) --> ".",!, {X1 is X+1}, grid_line(L, X1).
grid_line([], _) --> ("\r\n" | "\n" | call(eos)), !.
grid_line([sym(X, S)|T], X) --> symbol(S), {X1 is X+1}, grid_line(T, X1).
grid_line([num(XStart, N, XEnd)|T], XStart) 
    --> digits(D), {number_chars(N, D), length(D, Len), XEnd is XStart + Len},!, grid_line(T, XEnd).
symbol(S) --> [S], {char_type(S, punct)}.

has_adjacent_symbol(G, Y, num(XS, _, XE)) :-
    nth0(Y, G, GY), member(sym(SX, _), GY), (SX is XS-1 | SX is XE), !.
has_adjacent_symbol(G, Y, num(XS, _, XE)) :-
    (YI is Y-1 | YI is Y+1), nth0(YI, G, GY), member(sym(SX, _), GY), (SX >=XS-1, SX =< XE), !.

part_numbers(G, Numbers) :-
    part_numbers(G, Numbers, G, 0).
part_numbers(_, [], [], _).
part_numbers(G, Numbers, [H|T], Y) :-
    include(has_adjacent_symbol(G, Y), H, NHead),
    YT is Y+1, part_numbers(G, NTail, T, YT),
    append(NHead, NTail, Numbers).

number_from_part_number(num(_, N, _), N).

part1(G, Out) :-
    part_numbers(G, Numbers),
    maplist(number_from_part_number, Numbers, JustNumbers),
    sum_list(JustNumbers, Out).

adjacent_to_coord(X, num(XS, _, XE)) :- (X >=XS-1, X =< XE).

gear_ratio(G, Y, S) :- gear_ratio(G, Y, S, _).
gear_ratio(G, Y, sym(SX, S), Ratio) :-
    [S] = "*",
    YPrev is Y-1, YNext is Y+1,
    (nth0(YPrev, G, RPrev) | RPrev = []),
    nth0(Y, G, RCur),
    (nth0(YNext, G, RNext) | RNext = []),
    append([RPrev, RCur, RNext], Window),
    include(adjacent_to_coord(SX), Window, Candidates),
    Candidates = [num(_, N1, _), num(_, N2, _)],
    Ratio is N1 * N2.

gear_ratios(G, Ratios) :- gear_ratios(G, Ratios, G, 0).
gear_ratios(_, [], [], _).
gear_ratios(G, Ratios, [H|T], Y) :-
    include(gear_ratio(G, Y), H, Gears),
    maplist(gear_ratio(G, Y), Gears, RHead),
    YT is Y+1, gear_ratios(G, RTail, T, YT),
    append(RHead, RTail, Ratios).

part2(G, Out) :-
    gear_ratios(G, Ratios),
    sum_list(Ratios, Out).

:- phrase_from_file(grid(G), "day3.example.in"), part1(G, 4361).

:- phrase_from_file(grid(G), "day3.in"), part1(G, 533784).%Out), print(Out).

:- phrase_from_file(grid(G), "day3.example.in"), part2(G, 467835).

:- phrase_from_file(grid(G), "day3.in"), part2(G, 78826761).%Out), print(Out).