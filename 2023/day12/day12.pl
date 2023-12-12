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
combinations([], [], 1).
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
    combinations2(LFlat,NFlat, 0, 0, Combinations).
% How many ways are there to take a substring L and have it conform to constraints N,
% if it must start with Before hashes and end with After hashes.
combinations2([],[], 0, 0, 1):- !.
combinations2([],[], _, _, _):- !, fail.
combinations2(L, [], 0, 0, 1) :-
    !, maplist(\=(hash), L).
combinations2(L, [H], 0, 0, Combinations) :-
    !,
    window(L, H, Combinations), Combinations > 0.
combinations2(L, N, 0, 0, _) :-
    length(L, LL),
    sum_list(N, SN),
    length(N, NL),
    LL < SN+NL-1,
    !, fail.
combinations2(L, N, 0, 0, _) :-
    include(=(hash), L, Hashes),
    length(Hashes, LH),
    sum_list(N, SN),
    LH > SN,
    !,
    fail.
combinations2(L, N, 0, 0, Combinations) :-
    include(=(unknown), L, LUnknown),
    length(LUnknown, LU),
    LU =<4,
    !,
    combinations(L, N, Combinations).
combinations2(L, N, 0, 0, Combinations) :-
    %print(L), print(' '), print(N), nl,
    !,
    %Cut it in the middle, and send different cuts of N down each side
    aggregate_all(sum(C), combinations2_(L, N, C), Combinations),
    print(aggregate), print(':'), print(Combinations), nl.

combinations2(L, N, Before, After, Combinations) :-
    integer(Before),
    integer(After),
    length(LBefore, Before),
    length(LAfter, After),
    maplist(\=(dot), LBefore),
    maplist(\=(dot), LAfter),
    append([LBefore, LMid, LAfter], L),
    !, combinations2(LMid, N, 0, 0, Combinations).



combinations2_(L, N, Combinations) :-
    N \= [],
    length(L, LL),
    LLeft is LL div 2,
    length(Left, LLeft),
    append(Left, Right, L),
    append(NLeft, NRight, N),

    print('--'), print(Left), print(NLeft), print('--'), print(Right), print(NRight), nl,
    combinations2(Left, NLeft, 0, 0, CL),
    CL>0,
    print('---'), print(CL),nl,
    combinations2(Right, NRight, 0, 0, CR),
    CR>0,
    print(found), print(CL), print('*'), print(CR),nl,
    Combinations is CL*CR.
combinations2_(L, N, Combinations) :-
    N \= [],
    length(L, LL),
    LLeft is LL div 2,
    length(Left, LLeft),
    append(Left, Right, L),
    append(NLeft, [H|NRight], N),

    Hm1 is H-1,
    between(1,Hm1, Before),
    After is H-Before,

    %print('--'), print(Left), print(NLeft), print('-'), print(After),print('-'), print(Right), print(NRight), nl,
    combinations2(Left, NLeft, 0, After, CL),
    CL>0,
    combinations2(Right, NRight, Before, 0, CR),
    CR>0,
    %print(found2), print(CL), print('*'), print(CR),nl,
    Combinations is CL*CR.


part1(In, Out):-
    maplist(combinations, In, Combinations),
    sum_list(Combinations, Out).

part2(In, Out):-
    maplist(combinations2, In, Combinations),
    sum_list(Combinations, Out).

:- phrase_from_file(lines(I), "day12.example.in"), part1(I, 21).

:- phrase_from_file(lines(I), "day12.in"), part1(I, 8270).%Out), print(Out).

%:- phrase_from_file(lines(I), "day12.example.in"),  part2(I, 525152).

%:- phrase_from_file(lines(I), "day12.in"), part2(I, Out), print(Out).