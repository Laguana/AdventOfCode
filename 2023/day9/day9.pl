:- load_files(['../aoc_lib.pl']).

input([H|T]) --> numbers(H), eol, (input(T) | {T=[]}).
numbers([H|T]) --> integer(H), (" ", numbers(T) | {T=[]}).

diffs([], []).
diffs([_], []).
diffs([A,B|T], [DA|DT]) :-
    DA is A-B, % because we reversed the list
    diffs([B|T], DT).

next_element(L, 0) :-
    maplist(=(0), L).
next_element([H|T], E) :-
    diffs([H|T], D),
    next_element(D, DE),
    E is DE + H.

part1(I, Out) :-
    maplist(reverse,I,IR),
    maplist(next_element, IR, Nexts),
    sum_list(Nexts, Out).

part2(I, Out) :-
    maplist(next_element, I, Nexts),
    sum_list(Nexts, Out).

:- phrase_from_file(input(I), "day9.example.in"), part1(I, 114).

:- phrase_from_file(input(I), "day9.in"), part1(I, 1637452029).%Out), print(Out).

%:- phrase_from_file(input(I), "day9.example.in"),  part2(I, X).

:- phrase_from_file(input(I), "day9.in"), part2(I, 908).%Out), print(Out).