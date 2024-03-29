:- load_files(['../aoc_lib.pl']).

input(Grid) --> grid(grid{}, Grid,0).
grid(In, Out, Y) --> call(eos), {Out = In.put(ymax,Y)}.
grid(In, Out, Y) --> grid_line(In, O1, 0, Y), {Y1 is Y+1}, grid(O1, Out, Y1).
grid_line(In, Out, X, Y) --> ".",!, {NX is X+1}, grid_line(In, Out, NX, Y).
grid_line(In, Out, X, Y) --> 
    tile(T), !,
    {T=start -> NGrid = In.put(start, pos(X,Y));NGrid = In},
    {NX is X+1, pos_key(pos(X,Y), K)}, grid_line(NGrid.put(K, T), Out, NX, Y).
grid_line(In, In.put(xmax, X), X, _) --> eol.
tile(ns) --> "|".
tile(ew) --> "-".
tile(start) --> "S".
tile(ne) --> "L".
tile(nw) --> "J".
tile(se) --> "F".
tile(sw) --> "7".

adjacent(pos(X,Y), pos(NX, Y)) :- NX is X+1.
adjacent(pos(X,Y), pos(NX, Y)) :- NX is X-1.
adjacent(pos(X,Y), pos(X, NY)) :- NY is Y+1.
adjacent(pos(X,Y), pos(X, NY)) :- NY is Y-1.

connects(G, pos(X0,Y0), pos(X1,Y1)) :-
    pos_key(pos(X1,Y1), K),
    Tile = G.get(K),
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

wrapDist(D, Prev, P, q(P, Prev,D)).

valid_start_continuations(G, Start, Next) :-
    bagof(Adj, adjacent(Start,Adj), Adjacent),
    include(connects(G, Start), Adjacent, Valid),
    maplist(wrapDist(1, Start), Valid, Next).

seen(D, P) :- pos_key(P, K), D.get(K).
pos_key(pos(X,Y), K) :- atom_concat(X, ':', K0), atom_concat(K0, Y, K).

connected_distances(G, D, MaxD) :-
    Start = G.start,
    pos_key(Start, KStart),
    valid_start_continuations(G, Start, Next),
    connected_distances(G, Next, dist{}.put(KStart, 0), D, MaxD).
connected_distances(_, [q(Pos, _, _)|_], D,D, MaxDist) :-
    pos_key(Pos, K),
    MaxDist = D.get(K).
connected_distances(G, [q(Pos, Prev, Dist)|T], DIn, DOut, MaxDist) :-
    pos_key(Pos, PK),
    next(G.get(PK), Prev, Pos, Next),
    NDist is Dist+1,
    append(T, [q(Next, Pos, NDist)], TNext),
    pos_key(Pos, K),
    connected_distances(G, TNext, DIn.put(K, Dist), DOut, MaxDist).
    
part1(In, Out) :-
    connected_distances(In, _, Out).

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

count_enclosed_line(G, MainLoop, Y, Count) :-
    %print(Y), print(':'),
    count_enclosed_line(G, MainLoop, Y, G.xmax, out, 0, 0, Count),
    !.
count_enclosed_line(_, _, _, XMax, _, X, Count, Count) :-
    X >= XMax, !.
count_enclosed_line(G, MainLoop, Y, XMax, State, X, Acc, Count) :-
    pos_key(pos(X,Y), K),
    _ is MainLoop.get(K), !,
    Xn is X+1,
    effective_tile(G.get(K),MainLoop, X, Y, Tile),
    enclosed_step(Tile, State, NState),
    count_enclosed_line(G, MainLoop, Y, XMax, NState, Xn, Acc, Count).
count_enclosed_line(G, MainLoop, Y, XMax, State, X, Acc, Count) :-
    Xn is X+1,
    (State = out -> NAcc = Acc; State = in, NAcc is Acc+1),
    count_enclosed_line(G, MainLoop, Y, XMax, State, Xn, NAcc, Count).

count_enclosed(G, MainLoop, Out) :- 
    setof(Y, between(0,G.ymax, Y), Ys),!,
    %print(start), nl,
    maplist(count_enclosed_line(G,MainLoop), Ys, Counts),
    !,
    sumlist(Counts, Out).

part2(In, Out) :-
    connected_distances(In, D, _),!,
    count_enclosed(In, D, Out).


:- phrase_from_file(input(I), "day10.example.in"), part1(I, 8).

:- phrase_from_file(input(I), "day10.in"), part1(I, 6773).%Out), print(Out).

:- phrase_from_file(input(I), "day10.example2.in"),  part2(I, 10).

:- phrase_from_file(input(I), "day10.in"), part2(I, 493).%Out), print(Out).