:- load_files(['../aoc_lib.pl']).

report(report(L, N)) --> map(L), " ", numbers(N).
map([H|T]) --> tile(H), (map(T) | {T=[]}).
tile(dot) --> ".".
tile(hash) --> "#".
tile(unknown) --> "?".
numbers([H|T]) --> number(H), (",", numbers(T) | {T=[]}).

combinations(Row, Combinations) :-
    phrase(report(report(L,N)), Row),
    combinations(L,N, Combinations).

:- table combinations/3.

combinations([], [], 1).
combinations(L, [H], Combinations) :-
    window(L, H, Combinations).
combinations(L, N, _) :- length(L, LL), sum_list(N, SN), length(N, NL), LL < SN+NL-1, !, fail.
combinations([dot|T], N, Combinations) :- !,combinations(T, N, Combinations).
combinations([unknown|T], N, Combinations) :-
    !,
    (combinations([dot|T], N, C1),!;C1=0),
    (combinations([hash|T], N, C2),!;C2=0),
    Combinations is C1+C2.
combinations(T, [HN|TN], Combinations) :-
    length(TPrefix, HN),
    append(TPrefix, TR, T),
    (TR = [HRest|TRest] -> HRest \= hash; TRest = []),
    maplist(\=(dot), TPrefix),
    combinations(TRest, TN, Combinations).

window_(L,S, B,M,A) :-
    length(M, S),
    append([B,M,A], L),
    maplist(\=(hash), B),
    maplist(\=(hash), A),
    maplist(\=(dot), M).
window(L, Size, Count) :-
   aggregate_all(count, window_(L,Size, _,_,_), Count).

combinations2(Row, Combinations) :-
    phrase(report(report(L,N)), Row),
    length(UL, 5),
    maplist(=([unknown|L]), UL),
    length(UN, 5),
    maplist(=(N), UN),
    append(UN, NFlat),
    append(UL, [_|LFlat]),
    combinations(LFlat, NFlat, Combinations).

part1(In, Out):-
    maplist(combinations, In, Combinations),
    sum_list(Combinations, Out).

part2(In, Out):-
    maplist(combinations2, In, Combinations),
    sum_list(Combinations, Out).

:- phrase_from_file(lines(I), "day12.example.in"), part1(I, 21).

:- phrase_from_file(lines(I), "day12.in"), part1(I, 8270).%Out), print(Out).

:- phrase_from_file(lines(I), "day12.example.in"),  part2(I, 525152).

:- phrase_from_file(lines(I), "day12.in"), part2(I, 204640299929836).%Out), print(Out).