:- load_files(['../aoc_lib.pl']).

adjacent(pos(X,Y), pos(NX, Y)) :- NX is X+1.
adjacent(pos(X,Y), pos(NX, Y)) :- NX is X-1.
adjacent(pos(X,Y), pos(X, NY)) :- NY is Y+1.
adjacent(pos(X,Y), pos(X, NY)) :- NY is Y-1.

tile("|", ns).
tile("-", ew).
tile("L", ne).
tile("J", nw).
tile("F", se).
tile("7", sw).
tile("S", start).

connects(Lines, pos(X0,Y0), pos(X1,Y1)) :-
    nth0(Y1, Lines, Row),
    nth0(X1, Row, Char),!,
    tile([Char], Tile),
    DX is X1-X0,
    DY is Y1-Y0,
    atom_chars(Tile, Components),
    connects(Components, DX, DY).
connects([n,s], 0, -1).
connects([n,s], 0, 1).
connects([e,w], 1, 0).
connects([e,w], -1, 0).
connects([n,_], 0, 1).
connects([_,e], -1, 0).
connects([s,_], 0, -1).
connects([_,w], 1, 0).

wrapDist(D, Prev, P, q(P, Prev,D)).

valid_start_continuations(Lines, Start, Next) :-
    bagof(Adj, adjacent(Start,Adj), Adjacent),
    include(connects(Lines, Start), Adjacent, Valid),
    maplist(wrapDist(1, Start), Valid, Next).

seen(D, P) :- pos_key(P, K), D.get(K).
pos_key(pos(X,Y), K) :- atom_concat(X, ':', K0), atom_concat(K0, Y, K).

next(Tile, pos(X0,Y0), pos(X1,Y1), Next) :-
    DX is X1-X0,
    DY is Y1-Y0,
    next(Tile, d(DX, DY, X1,Y1), Next).
next(ns, d(_, DY, X1, Y1), pos(X1, Yn)) :- Yn is Y1+DY.
next(ew, d(DX, _, X1, Y1), pos(Xn, Y1)) :- Xn is X1+DX.
%          DY=1
%           v
%  DX=1 --> X <-- DX=-1
%           ^
%          DY=-1
next(ne, d(DX, DY, X1, Y1), pos(Xn, Yn)) :- Xn is X1+DY, Yn is Y1+DX. % yes dx and dy are right here
next(nw, d(DX, DY, X1, Y1), pos(Xn, Yn)) :- Xn is X1-DY, Yn is Y1-DX.
next(se, d(DX, DY, X1, Y1), pos(Xn, Yn)) :- Xn is X1-DY, Yn is Y1-DX.
next(sw, d(DX, DY, X1, Y1), pos(Xn, Yn)) :- Xn is X1+DY, Yn is Y1+DX.

connected_distances(Lines, D, Start, MaxD) :-
    valid_start_continuations(Lines, Start, Next),
    pos_key(Start, KStart),
    connected_distances(Lines, Next, dist{}.put(KStart, 0), D, MaxD).
connected_distances(_, [q(Pos, _, _)|_], D,D, MaxDist) :-
    pos_key(Pos, K),
    MaxDist = D.get(K).
connected_distances(Lines, [q(Pos, Prev, Dist)|T], DIn, DOut, MaxDist) :-
    Pos = pos(X, Y),
    nth0(Y, Lines, Row),
    nth0(X, Row, C),!,
    tile([C], Tile),
    next(Tile, Prev, Pos, Next),
    NDist is Dist+1,
    append(T, [q(Next, Pos, NDist)], TNext),
    pos_key(Pos, K),
    connected_distances(Lines, TNext, DIn.put(K, Dist), DOut, MaxDist).

part1(Lines, Out) :-
    [S] = "S",
    !,
    nth0(StartY, Lines, StartRow),
    nth0(StartX, StartRow, S),
    !,
    connected_distances(Lines, _, pos(StartX,StartY), Out).

% State is out, n, s, in
enclosed_step(ew, State, State). % :- print('-').
enclosed_step(ns, out, in). % :- print('i').
enclosed_step(ns, in, out). % :- print('i').
% Not legal to be n or s

enclosed_step(ne, out, n). % :- print('\\').
enclosed_step(ne, in, s). % :- print('\\').
enclosed_step(se, out, s). % :- print('/').
enclosed_step(se, in, n). % :- print('/').
% Not legal to be n or s

enclosed_step(nw, n, out). % :- print('/').
enclosed_step(nw, s, in). % :- print('/').
enclosed_step(sw, n, in). % :- print('\\').
enclosed_step(sw, s, out). % :- print('\\').
% Not legal to be in or out

is_one_step(MainLoop, Pos) :-
    pos_key(Pos, K),
    MainLoop.get(K) = 1.

pos_sub(pos(X,Y), pos(X1,Y1), delta(DX, DY)) :-
    DX is X1-X,
    DY is Y1-Y.

delta_to_tile([delta(-1, 0), delta(0, -1)], nw).
delta_to_tile([delta(-1, 0), delta(0, 1)], sw).
delta_to_tile([delta(-1, 0), delta(1, 0)], ew).
delta_to_tile([delta(0, -1), delta(0, 1)], ns).
delta_to_tile([delta(0, -1), delta(1, 0)], se).
delta_to_tile([delta(1, 0), delta(0, -1)], ne).



effective_tile(start, MainLoop, X, Y, Tile) :- !,
    bagof(Adj, adjacent(pos(X,Y),Adj), Adjacent),
    include(is_one_step(MainLoop), Adjacent, OneStep),
    maplist(pos_sub(pos(X,Y)), OneStep, Deltas),
    sort(Deltas, SDeltas),
    delta_to_tile(SDeltas, Tile).
effective_tile(Tile, _, _, _, Tile).

count_enclosed_line(Lines, MainLoop, Line, Y, Count) :-
    %print(Y), print(':'),
    count_enclosed_line(Lines, MainLoop, Y, Line, out, 0, 0, Count),
    !.
count_enclosed_line(_, _, _, [], _, _, Count, Count).
count_enclosed_line(Lines, MainLoop, Y, [H|T], State, X, Acc, Count) :-
    pos_key(pos(X,Y), K),
    _ is MainLoop.get(K), !,
    Xn is X+1,
    tile([H], RawTile),
    effective_tile(RawTile,MainLoop, X, Y, Tile),
    enclosed_step(Tile, State, NState),
    count_enclosed_line(Lines, MainLoop, Y, T, NState, Xn, Acc, Count).
count_enclosed_line(Lines, MainLoop, Y, [_|T], State, X, Acc, Count) :-
    Xn is X+1,
    (State = out -> NAcc = Acc; State = in, NAcc is Acc+1),
    count_enclosed_line(Lines, MainLoop, Y, T, State, Xn, NAcc, Count).

count_enclosed(Lines, MainLoop, Out) :- 
    count_enclosed(Lines, MainLoop, Lines, 0, 0, Out).
count_enclosed(_, _, [], _, Out, Out).
count_enclosed(Lines, MainLoop, [H|T], Y, Acc, Out) :-
    count_enclosed_line(Lines, MainLoop, H, Y, Count),
    NAcc is Acc + Count,
    NY is Y+1,
    !,
    count_enclosed(Lines, MainLoop, T, NY, NAcc, Out).


part2(Lines, Out) :-
    [S] = "S",
    !,
    nth0(StartY, Lines, StartRow),
    nth0(StartX, StartRow, S),
    !,
    connected_distances(Lines, D, pos(StartX,StartY), _),
    !,
    count_enclosed(Lines, D, Out).

:- phrase_from_file(lines(L), "day10.in"), part1(L, 6773).%Out), print(Out).

:- phrase_from_file(lines(L), "day10.in"), part2(L, 493).%Out), print(Out).