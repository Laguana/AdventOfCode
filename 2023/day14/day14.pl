:- load_files(['../aoc_lib.pl']).

input(I) --> lines(I).

calculate_score(Y, Rocks, Score) :-
    LastY is Y-1,
    % with Rocks rocks, you have Y+(Y-1)+(...)+(Y-Rocks-1)
    % Rocks * Y - (1+2+...+Rocks-1)
    % Rocks * Y - (Rocks*Rocks-1)/2?
    Score is Rocks * LastY - Rocks*(Rocks-1) div 2.
score_column(L, Score) :-
    reverse(L, RL),
    score_column(RL, 1, 0, 0, Score).
score_column([], Y, Rocks, Acc, Score) :-
    calculate_score(Y, Rocks, ThisScore),
    Score is Acc + ThisScore.
score_column([H|T], Y, Rocks, Acc, Score) :-
    [H] = "#",!,
    calculate_score(Y, Rocks, ThisScore),
    TY is Y+1,
    TAcc is Acc + ThisScore,!,
    score_column(T, TY, 0, TAcc, Score).
score_column([H|T], Y, Rocks, Acc, Score) :-
    [H] = ".",!,
    TY is Y+1,
    score_column(T, TY, Rocks, Acc, Score).
score_column([H|T], Y, Rocks, Acc, Score) :-
    [H] = "O",!,
    TY is Y+1,
    TRocks is Rocks+1,
    score_column(T, TY, TRocks, Acc, Score).


score(Grid, Score) :-
    transpose(Grid, TG),
    maplist(score_column, TG, Scores),
    sum_list(Scores, Score).

part1(In, Out) :-
    score(In, Out).

pos_atom(X,Y,K) :- atom_concat(X,':', Z), atom_concat(Z, Y, K).
atom_pos(K, p(X,Y)) :- atom_concat(Z, Ya, K),atom_concat(Xa,':', Z),!,atom_number(Ya,Y), atom_number(Xa,X).

grid_as_dicts(In, Grid) :-
    grid_as_dicts(In, 0, grid{rocks:rocks{},stops:stops{}}, Grid).
grid_as_dicts([H|T], Y, In, Out) :-
    grid_as_dicts(H, Y, 0, In, Mid),
    TY is Y+1,
    grid_as_dicts(T, TY, Mid, Out).
grid_as_dicts([], Y, Grid, Grid.put(maxy, Y)).
grid_as_dicts([H|T], Y, X, In, Out) :-
    [H] = "#",!,
    pos_atom(X, Y, K),
    Mid = grid{rocks:In.rocks, stops:In.stops.put(K, '#')},
    TX is X+1,
    grid_as_dicts(T, Y, TX, Mid, Out).
grid_as_dicts([H|T], Y, X, In, Out) :-
    [H] = "O",!,
    pos_atom(X, Y, K),
    Mid = grid{rocks:In.rocks.put(K, 'O'), stops:In.stops},
    TX is X+1,
    grid_as_dicts(T, Y, TX, Mid, Out).
grid_as_dicts([H|T], Y, X, In, Out) :-
    [H] = ".",!,
    TX is X+1,
    grid_as_dicts(T, Y, TX, In, Out).
grid_as_dicts([], _, X, Grid, Grid.put(maxx, X)).

precompute_destinations(Grid, Out) :-
    % For a thing starting at p(X,Y), how far north/south/east/west does it go?
    precompute_destinations(Grid, 0,dests{}, Out).
precompute_destinations(Grid, Grid.maxy, Out, Out).
precompute_destinations(Grid, Y, In, Out) :-
    precompute_destinations(Grid, Y, 0, In, Mid),
    YN is Y+1,!,
    precompute_destinations(Grid, YN, Mid, Out).
precompute_destinations(Grid, _, Grid.maxx, Out, Out).
precompute_destinations(Grid, Y, X, In, Out) :-
    pos_atom(X,Y,K),
    XN is X+1,
    (_ = Grid.stops.get(K) -> precompute_destinations(Grid, Y, XN, In, Out);
    precompute_destination(Grid, Y, X, Deltas),!,
    precompute_destinations(Grid, Y, XN, In.put(K, Deltas), Out)).
precompute_destination(Grid, Y, X, deltas{north:DNorth, south:DSouth, east:DEast, west:DWest}) :-
    precompute_destination(north, Grid, Y, X, DNorth),
    precompute_destination(south, Grid, Y, X, DSouth),
    precompute_destination(east, Grid, Y, X, DEast),
    precompute_destination(west, Grid, Y, X, DWest).
precompute_destination(Dir, Grid, Y, X, DOut) :-
    between(1, Grid.maxx, Delta),
    is_wall(Dir, Grid, Y, X, Delta),!,
    DOut is Delta-1.
is_wall(north, Grid, Y, X, Delta) :-
    YY is Y-Delta,
    (YY < 0;
    pos_atom(X,YY, K),
    _=Grid.stops.get(K)),!.
is_wall(south, Grid, Y, X, Delta) :-
    YY is Y+Delta,
    (YY >= Grid.maxy;
    pos_atom(X,YY, K),
    _=Grid.stops.get(K)),!.
is_wall(west, Grid, Y, X, Delta) :-
    XX is X-Delta,
    (XX < 0;
    pos_atom(XX,Y, K),
    _=Grid.stops.get(K)),!.
is_wall(east, Grid, Y, X, Delta) :-
    XX is X+Delta,
    (XX >=Grid.maxx;
    pos_atom(XX,Y, K),
    _=Grid.stops.get(K)),!.

go_(Dir, Destinations, p(X,Y), p(XX,YY)) :-
    pos_atom(X,Y, K),
    Delta = Destinations.get(K).Dir,
    go_(Dir, Delta, X,Y,XX,YY).
go_(north, Delta, X, Y, X,YY) :- YY is Y-Delta.
go_(south, Delta, X, Y, X,YY) :- YY is Y+Delta.
go_(west, Delta, X, Y, XX,Y) :- XX is X-Delta.
go_(east, Delta, X, Y, XX,Y) :- XX is X+Delta.

unclump(Dir, p(X,Y)-N, Out) :-
    length(Out, N),
    unclump(Dir, Out, X,Y).
unclump(_, [],_,_).
unclump(Dir, [p(X,Y)|T], X, Y) :-
    go_(Dir, -1, X,Y,XX,YY),
    unclump(Dir, T, XX,YY).

go(Dir, Destinations, RIn, ROut) :-
    maplist(go_(Dir, Destinations), RIn, RMid),
    msort(RMid, RSort),
    clumped(RSort, RClumped),
    maplist(unclump(Dir), RClumped, Expanded),
    append(Expanded, ROut).

do_cycle(RIn, Destinations, ROut) :-
    go(north, Destinations, RIn, RNorth),
    go(west, Destinations, RNorth, RWest),
    go(south, Destinations, RWest, RSouth),
    go(east, Destinations, RSouth, REast),
    sort(REast,ROut).

get_cycle(Grid, Destinations, Cycle) :-
    dict_keys(Grid.rocks, RocksK),
    maplist(atom_pos, RocksK, Rocks),
    sort(Rocks, Sorted),
    get_cycle(Sorted, Destinations, [Sorted], Cycle).
get_cycle(RIn, Destinations, Trace, Cycle) :-
    do_cycle(RIn, Destinations, ROut),!,
    (nth1(Period, Trace, ROut)
    -> length(Trace, LTrace), Start is LTrace-Period,length(Repeat,Period), append(Repeat,_,Trace), reverse(Repeat, CList), Cycle = cycle(Start-Period, CList)
    ; get_cycle(ROut, Destinations, [ROut|Trace], Cycle)).

score2_(Maxy, p(_,Y), Score) :-
    Score is Maxy-Y.
score2(Grid, Rocks, Out) :-
    maplist(score2_(Grid.maxy), Rocks, Scores),
    sum_list(Scores,Out).

part2(In, Out) :-
    grid_as_dicts(In, Grid),
    precompute_destinations(Grid, Destinations),
    get_cycle(Grid, Destinations, cycle(Start-Period,Repeat)),
    Phase is (1000000000 - Start) mod Period,
    nth0(Phase, Repeat, State),
    score2(Grid,State, Out).

print_grid(Grid, Rocks) :-
    YM is Grid.maxy-1,
    XM is Grid.maxx-1,
    (between(0,YM, Y),
    nl,
    between(0,XM, X),
    (member(p(X,Y), Rocks) -> write('O');write(' ')), fail).
print_grid(_,_) :- write('---'), nl.

:- phrase_from_file(input(I), "day14.example.in"), part1(I, 136).

:- phrase_from_file(input(I), "day14.in"), part1(I, 109665).%Out), print(Out).

:- phrase_from_file(input(I), "day14.example.in"),  part2(I, 64).

:- phrase_from_file(input(I), "day14.in"), part2(I, 96061).%Out), print(Out).