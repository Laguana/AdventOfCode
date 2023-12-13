:- load_files(['../aoc_lib.pl']).

input([H|T])       --> grid(H), !, (call(eos),!, {T=[]} | input(T), !).
grid([])           --> eol, !.
grid([H|T])        --> grid_line(H), {H =[_|_]}, grid(T).
grid_line([])      --> eol, !.
grid_line([C|Cs])  --> [C], line(Cs).

vertical_reflection(L, Column, Part) :-
    L = [H|_],
    length(H, XMax),
    Xm1 is XMax-1,
    (Part = p2 -> Expected = 1; Expected=0),
    !,
    between(1, Xm1, Column),
    maplist(mirror(Column), L, Mismatches),
    sum_list(Mismatches, Expected),
    !.
mirror(LLeft, List , Mismatches) :-
    length(L, LLeft),
    append(L,R, List),
    reverse(L, LR),
    mismatches(LR, R, Mismatches).
    %(append(LR, _, R);append(R,_,LR)),!.

mismatches(_, [], 0).
mismatches([], _, 0).
mismatches([H|T], [RH|RT], M) :-
    mismatches(T,RT, MT),
    (H=RH -> Add=0;Add=1),
    M is MT + Add.

horizontal_reflection(L, Row, Part) :-
    transpose(L, TL),
    vertical_reflection(TL, Row, Part).

score(Part, Grid, Score) :-
    vertical_reflection(Grid, Column, Part),
    !,
    Score = Column.
score(Part, Grid, Score) :-
    horizontal_reflection(Grid, Row, Part),
    !,
    Score is 100*Row.

% Debugging!
score(_, Grid,_) :-
    print_grid(Grid),
    fail.

print_grid(G) :- print(G).

part1(In, Out) :-
    maplist(score(p1), In, Scores),
    sum_list(Scores, Out).

part2(In, Out) :-
    maplist(score(p2), In, Scores),
    sum_list(Scores, Out).

:- phrase_from_file(input(I), "day13.example.in"), part1(I, 405).

:- phrase_from_file(input(I), "day13.in"), part1(I, 40006).%Out), print(Out).

:- phrase_from_file(input(I), "day13.example.in"),  part2(I, 400).

:- phrase_from_file(input(I), "day13.in"), part2(I, 28627).%Out), print(Out).