:- load_files(['../aoc_lib.pl']).

input(map{directions:D, nodes:N}) --> directions(D, 0), eol, eol, nodes(N).
directions([instr(H, I)|T],I) --> direction(H), ({IT is I+1}, directions(T, IT) | {T=[]}).
direction(left) --> "L".
direction(right) --> "R".
nodes([node(From, Left, Right)|T]) --> word(From), " = (", word(Left), ", ", word(Right), ")", eol, (nodes(T) | {T=[]}).

follow(map{directions:D, nodes:N}, From, To, Steps) :-
    append(D, CyclicDirections, CyclicDirections),
    follow(CyclicDirections, N, From, To, 0, Steps).
follow(_, _, From, From, Steps, Steps).
follow([H|T], N, From, To, SoFar, Steps) :-
    NextSoFar is SoFar+1,
    member(node(From, Left, Right), N),
    (H = instr(left ,_)
        -> follow(T, N, Left, To, NextSoFar, Steps)
    ; follow(T, N, Right, To, NextSoFar, Steps)).

validStart(node(From, _, _), From) :- [A] = "A", From = [_,_,A].
validEnd(From) :- [Z] = "Z", From = [_,_,Z].

single_step(instr(left,_), N, From, To) :- member(node(From, To, _), N), !.
single_step(instr(right,_), N, From, To) :- member(node(From, _, To), N), !.

cycle_valid_ends(Prefix, state(Location, Step, _), Phase) :-
    validEnd(Location), Phase is Step-Prefix, Phase >= 0.

adjust_ends(Prefix, End, CEnd) :-
    CEnd is End-Prefix, CEnd >= 0.

cycle([state(Location, Step, instr(_,Phase))|T], Ends, cycle(Prefix, Period, CEnds)) :-
    member(state(Location, Prefix, instr(_,Phase)), T),
    Period is Step-Prefix, !,
    convlist(adjust_ends(Prefix), Ends, CEnds).
    %convlist(cycle_valid_ends(Prefix), T,Ends).

find_cycle(Instructions, N, Location, Cycle) :- find_cycle(Instructions, N, 0, Location, [], [], Cycle).
find_cycle(_,_,_,_, Ends, Trace, Cycle) :- cycle(Trace, Ends, Cycle), !.
find_cycle([H|T], N, Step, Location, Ends, Trace, Cycle) :-
    NStep is Step + 1,
    (validEnd(Location) -> TEnds = [Step|Ends]; TEnds = Ends),
    single_step(H, N, Location, To),!,
    find_cycle(T, N, NStep, To, TEnds, [state(Location, Step, H)|Trace], Cycle).

constraints(Prefix, Period, End, [constraint(Period, Rem)]) :-
    % step Prefix + x*Period + End is a valid end
    % which means if Step%Period = (End+Prefix)%Period then it is valid
    Rem is (Prefix + End) mod Period.

prepend(L, E, [E|L]).

combine_options(Prev, New, Out) :- combine_options(Prev, New, [], Out).
combine_options([], _, Acc, Acc).
combine_options([H|T], New, Acc, Out) :-
    % H is a conjunction of constraints, New is a disjunction; we want to prepend each New to H
    maplist(prepend(H), New, NewH),
    append(NewH, Acc, TAcc),
    combine_options(T, New, TAcc, Out).

extended_euclid(A, B, S, T) :- A>B -> extended_euclid(A,B,1,0,0,1,S,T);extended_euclid(B,A,1,0,0,1,T,S).
extended_euclid(_, 0, S, _, T, _, S, T).
extended_euclid(R0, R1, S0, S1, T0, T1, SK, TK) :-
    Q is R0 div R1,
    R2 is R0 - Q*R1,
    S2 is S0 - Q*S1,
    T2 is T0 - Q*T1,
    extended_euclid(R1, R2, S1, S2, T1, T2, SK, TK).



smallest_solution(BigPrefix, Constraints, Solution) :-
    smallest_solution(BigPrefix, Constraints, constraint(1, 0), Solution).
smallest_solution(BigPrefix, [constraint(CMod, CRem)|T], constraint(Mod, Rem), Solution) :-
    % All mods are prime
    % if N % p = r
    %    N % q = s
    % then N % pq = aps + bqr where ap + bq = 1
    smallest_solution(BigPRefix, T, constraint(TMod, TRem), Solution).
smallest_solution(BigPrefix, [], constraint(Mod, Rem), Solution) :-
    fail.

cycle_prefix(cycle(Prefix,_,_), Prefix).

combine_cycles(Cycles, Steps) :-
    maplist(cycle_prefix, Cycles, Prefixes),
    max_list(Prefixes, BigPrefix),
    combine_cycles(Cycles, BigPrefix, Steps).
combine_cycles([cycle(Prefix, Period, Ends)|T], BigPrefix, Steps) :-
    maplist(constraints(Prefix, Period) Ends, Options),
    combine_cycles(T, BigPrefix, Options, Steps).
combine_cycles([cycle(Prefix, Period, Ends)|T], BigPrefix, Options, Steps) :-
    % Each Option is a different potential solution
    maplist(constraints(Prefix, Period), Ends, HeadOptions),
    combine_options(Options, HeadOptions, NewOptions),
    combine_cycles(T, BigPrefix, NewOptions, Steps).
combine_cycles([], BigPrefix, Options, Steps) :-
    trace,
    convlist(smallest_solution(BigPrefix), Options, Solutions),
    min_list(Solutions, Steps),
    Steps > BigPrefix.

part2(map{directions:D, nodes:N}, Steps) :-
    append(D, CyclicDirections, CyclicDirections),
    convlist(validStart, N, Starts),
    maplist(find_cycle(CyclicDirections,N), Starts, Cycles),
    combine_cycles(Cycles, Steps).
    

/*
followAll(map{directions:D, nodes:N}, Steps) :-
    append(D, CyclicDirections, CyclicDirections),
    convlist(validStart, N, From),
    followAll(CyclicDirections, N, From, 0, Steps).
followAll(_, _, End, Steps, Steps) :- maplist(validEnd, End), !.
followAll([H|T], N, From, SoFar, Steps) :-
    NextSoFar is SoFar+1,
    maplist(single_step(H,N), From, To),
    %maplist(atom_codes, FromAtoms, From),
    %maplist(atom_codes, ToAtoms, To),
    %print(H), print(FromAtoms), print(' '), print(ToAtoms), nl,
    followAll(T, N, To, NextSoFar, Steps).
*/

part1(In, Out) :-
    follow(In, "AAA", "ZZZ", Out).

%part2(In, Out) :-
%    followAll(In, Out).

:- phrase_from_file(input(I), "day8.example.in"), part1(I, 2).
:- phrase_from_file(input(I), "day8.example2.in"), part1(I, 6).

:- phrase_from_file(input(I), "day8.in"), part1(I, 17287).%Out), print(Out).

%:- phrase_from_file(input(I), "day8.example3.in"),  part2(I, 6).

%:- phrase_from_file(input(I), "day8.in"), part2(I, Out), print(Out).