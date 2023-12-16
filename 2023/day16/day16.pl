:- load_files(['../aoc_lib.pl']).

input(I) --> lines(I).

pos_atom(X,Y, K) :- atom_concat(X,:, M), atom_concat(M,Y,K).

energize(In, Energized) :-
    length(In,MaxY),
    In = [H|_],
    length(H, MaxX),

    energize(In, MaxX, MaxY, [p(1,1,right)],energized{}, Energized).
energize(_, _, _, [], Energized, Energized).
energize(Grid, MaxX, MaxY, [p(X,Y,_)|T], EIn, EOut) :-
    (\+ (between(1, MaxX, X), between(1, MaxY,Y))),!,
    energize(Grid, MaxX, MaxY, T, EIn, EOut).
energize(Grid, MaxX, MaxY, [p(X,Y,Dir)|T], EIn, EOut) :-
    pos_atom(X,Y,K),
    nth1(Y, Grid, Row),
    nth1(X, Row, C),
    (OldDir = EIn.get(K)
     -> (
        redundant(Dir, OldDir, C) 
        -> energize(Grid, MaxX, MaxY, T, EIn, EOut)
        ; energize_(Grid, MaxX,MaxY, C, p(X,Y,Dir), T, EIn.put(K, both), EOut))
    ; energize_(Grid, MaxX,MaxY, C, p(X,Y,Dir), T, EIn.put(K, Dir), EOut)).
energize_(Grid, MaxX, MaxY, C, p(X,Y,Dir), T, EIn, EOut) :-
    single_energize(C, X,Y,Dir, New),
    append(New, T, Next),
    energize(Grid, MaxX, MaxY, Next, EIn, EOut).

redundant(_,both,_).
redundant(Dir, Dir,_).
redundant(up, down, C) :- \+ member(C, "/\\").
redundant(down,up, C) :- \+ member(C, "/\\").
redundant(left, right, C) :- \+ member(C, "/\\").
redundant(right, left, C) :- \+ member(C, "/\\").

step(X,Y,right,XX,Y) :- XX is X+1.
step(X,Y,left,XX,Y) :- XX is X-1.
step(X,Y,up,X,YY) :- YY is Y-1.
step(X,Y,down,X,YY) :- YY is Y+1.

interact(C, Dir, Dir) :- [C] = ".".

interact(C, up   , up   ) :- [C] = "|".
interact(C, down , down ) :- [C] = "|".
interact(C, left , left ) :- [C] = "-".
interact(C, right, right) :- [C] = "-".

interact(C, right, up   ) :- [C] = "/".
interact(C, down , left ) :- [C] = "/".
interact(C, left , down ) :- [C] = "/".
interact(C, up   , right) :- [C] = "/".
interact(C, right, down ) :- [C] = "\\".
interact(C, down , right) :- [C] = "\\".
interact(C, left , up   ) :- [C] = "\\".
interact(C, up   , left ) :- [C] = "\\".

interact(C, right, up  , down ) :- [C] = "|".
interact(C, left , up  , down ) :- [C] = "|".
interact(C, up   , left, right) :- [C] = "-".
interact(C, down , left, right) :- [C] = "-".

single_energize(C, X, Y, Dir, [p(XX,YY, NewDir)]) :-
    interact(C,Dir, NewDir),!,
    step(X,Y,NewDir, XX,YY).
single_energize(C, X, Y, Dir, [p(XX,YY, D1), p(X2,Y2, D2)]) :-
    interact(C, Dir, D1, D2),!,
    step(X,Y,D1, XX, YY),
    step(X,Y,D2, X2, Y2).

part1(I, O) :-
    energize(I,E),
    dict_keys(E,K),
    length(K,O).

:- phrase_from_file(input(I), "day16.example.in"), part1(I, 46).

:- phrase_from_file(input(I), "day16.in"), part1(I, 7608).%Out), print(Out).

%:- phrase_from_file(input(I), "day16.example.in"),  part2(I, ).

%:- phrase_from_file(input(I), "day16.in"), part2(I, Out), print(Out).