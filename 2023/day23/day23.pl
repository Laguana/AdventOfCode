:- load_files(['../aoc_lib.pl']).

input(I) --> lines(I).

xy_key(p(X,Y),K) :- atomic_list_concat([X,:,Y], K).

neighbor(p(X,Y), p(XX,Y)) :-
    (DX is -1; DX is +1),
    XX is X+DX, XX>=0.
neighbor(p(X,Y), p(X,YY)) :-
    (DY is -1; DY is +1),
    YY is Y+DY, YY>=0.

slope_neighbor(p(X,Y), "^", p(X,YY), _) :-
    YY is Y-1.
slope_neighbor(p(X,Y), ">", p(XX,Y), _) :-
    XX is X+1.
slope_neighbor(p(X,Y), "v", p(X,YY), _) :-
    YY is Y+1.
slope_neighbor(p(X,Y), "<", p(XX,Y), _) :-
    XX is X-1.
slope_neighbor(p(X,Y), "^", p(X,YY), p2) :-
    YY is Y+1.
slope_neighbor(p(X,Y), ">", p(XX,Y), p2) :-
    XX is X-1.
slope_neighbor(p(X,Y), "v", p(X,YY), p2) :-
    YY is Y-1.
slope_neighbor(p(X,Y), "<", p(XX,Y), p2) :-
    XX is X+1.


valid_slope(_, _, C, _) :- [C] = ".",!.
valid_slope(p(X,Y), p(XX,YY), C, PX) :-
    DX is X-XX,
    DY is Y-YY,
    valid_slope(d(DX,DY), [C], PX).
valid_slope(d(-1,0), ">", _).
valid_slope(d(1,0), "<", _).
valid_slope(d(0,-1), "v", _).
valid_slope(d(0,1), "^", _).
valid_slope(d(-1,0), "<", p2).
valid_slope(d(1,0), ">", p2).
valid_slope(d(0,-1), "^", p2).
valid_slope(d(0,1), "v", p2).


paths_from_grid(I, O, PX) :-
    paths_from_grid(I, [p(1,0)], p{}, O, PX).
paths_from_grid(_,[], O, O, _) :- !.
paths_from_grid(I,[P|T], SoFar, O, PX) :- 
    xy_key(P, K),
    _ = SoFar.get(K),
    !,
    paths_from_grid(I, T, SoFar, O, PX).
paths_from_grid(I, [P|T], SoFar, O, PX) :-
    get_paths(I,P, Paths, PX),
    xy_key(P, K),
    get_ends(I,Paths, Ends),
    append(Ends, T, TT),
    paths_from_grid(I, TT, SoFar.put(K, Paths), O, PX).

end(_-P, P).

get_ends(I,Paths, Ends) :-
    length(I, L),
    LastRow is L-1,
    maplist(end, Paths, E),
    exclude(=(p(_,LastRow)), E, E2),
    sort(E2, Ends).

get_paths(I, P, Ends, PX) :-
    bagof(N, neighbor(P, N), Ns),
    get_paths(I, P, Ns, [], Ends, PX), !.
get_paths(_, _, [], E, E, _) :- !.
get_paths(I, P, [p(X,Y)|T], Acc, Ends, PX) :-
    nth0(Y, I, Row),
    nth0(X, Row, C),!,
    (valid_slope(P, p(X,Y), C, PX) ->
        follow_path(I, P, p(X,Y), 1, End, PX),!,
        get_paths(I,P, T, [End|Acc], Ends, PX)
    ; get_paths(I,P, T, Acc, Ends, PX)).

follow_path(_,_,p(X,0), L, L-p(X,0),p2).
follow_path(I,Prev, P, L, End, PX) :-
    neighbor(P,N),
    N \= Prev,
    N = p(X,Y),
    nth0(Y, I, Row),
    nth0(X, Row, C),
    valid_slope(P, N, C, PX),!,
    ([C] = "." 
        -> NL is L+1, follow_path(I,P, N, NL, End, PX)
    ; slope_neighbor(N, [C], NN, PX), NN \= P, NL is L+2, End=NL-NN).
follow_path(I,_, p(X,Y), L, L-p(X,Y), _) :-
    YL is Y+1,
    length(I,YL),!.

%:- table longest_path/4.

longest_path(Paths,From, Seen, Longest) :-
    xy_key(From, K),
    Steps = Paths.get(K),!,
    sort([From|Seen], TSeen), longest_path(Paths, Steps, TSeen, -1-[], LD-LP),
    (LP = [] 
    -> Longest= -1-[]
    ; Longest = LD-[From|LP]),!.
longest_path(Paths,From, _, Longest) :-
    xy_key(From, K),
    (\+ _ = Paths.get(K)),
    Longest = 0-[From].
longest_path(_, [], _, Longest, Longest) :- !.
longest_path(Paths, [D-H|T], Seen, SD-SP, Longest) :- 
    member(H, Seen) 
    -> longest_path(Paths, T, Seen, SD-SP, Longest)
    ; longest_path(Paths, H, Seen, HLong-HP),!,

        TSoFar is max(SD, D+HLong),
        (TSoFar = SD -> TP = SP; TP=HP),
        longest_path(Paths, T, Seen, TSoFar-TP, Longest).

part1(I, O) :-
    paths_from_grid(I, P, p1),
    longest_path(P, p(1,0), [], O-_).

part2(I, O) :-
    paths_from_grid(I, P, p2),
    longest_path(P, p(1,0), [], O-_).



:- phrase_from_file(input(I), "day23.example.in"), part1(I, 94).

:- phrase_from_file(input(I), "day23.in"), part1(I, 2186).%Out), print(Out).

:- phrase_from_file(input(I), "day23.example.in"),  part2(I, 154).

%:- phrase_from_file(input(I), "day23.in"), part2(I, Out), print(Out).