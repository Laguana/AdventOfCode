:- load_files(['../aoc_lib.pl']).

input(map{directions:D, nodes:N}) --> directions(D, 0), eol, eol, nodes(N).
directions([instr(H, I)|T],I) --> direction(H), ({IT is I+1}, directions(T, IT) | {T=[]}).
direction(left) --> "L".
direction(right) --> "R".
nodes(D) --> word_atom(From), " = (", word_atom(Left), ", ", word_atom(Right), ")", eol,
    (nodes(DT), ! | {DT=nodes{}}), {D = DT.put(From, [Left, Right])}.


follow(map{directions:D, nodes:N}, From, To, Steps) :-
    append(D, CyclicDirections, CyclicDirections),
    follow(CyclicDirections, N, From, To, 0, Steps).
follow(_, _, From, From, Steps, Steps).
follow([H|T], N, From, To, SoFar, Steps) :-
    NextSoFar is SoFar+1,
    [Left, Right] = N.From,
    (H = instr(left ,_)
        -> follow(T, N, Left, To, NextSoFar, Steps)
    ; follow(T, N, Right, To, NextSoFar, Steps)).

validStart(From) :- [A] = "A", atom_codes(From, [_,_,A]).
validEnd(From) :- [Z] = "Z", atom_codes(From, [_,_,Z]).

single_step(left, [To,_], To).
single_step(right, [_,To], To).

cycle_key(Location, Phase, Key) :- 
    atom_concat(Location, Phase, Key).

cycle(Map, Location, Phase, Step, cycle(Prefix, Period)) :-
    cycle_key(Location, Phase, Key),
    Prefix = Map.get(Key),
    Period is Step-Prefix.

find_cycle(Instructions, N, Location, Cycle) :- find_cycle(Instructions, N, 0, Location, trace{}, Cycle).
find_cycle([instr(_,Phase)|_],_,Step, Location, Map, Cycle) :- cycle(Map, Location, Phase, Step, Cycle), !.
find_cycle([instr(Dir,Phase)|T], N, Step, Location, Map, Cycle) :-
    NStep is Step + 1,
    single_step(Dir, N.Location, To),
    cycle_key(Location, Phase, Key),
    !,
    find_cycle(T, N, NStep, To, Map.put(Key, Step), Cycle).

combine_cycles(IL, [], IL).
combine_cycles(IL, [cycle(_, H)|T], Steps) :-
    combine_cycles(IL,T,TSteps),
    CT is H div IL,
    Steps is TSteps * CT.

part1(In, Out) :-
    atom_codes(Start, "AAA"),
    atom_codes(End, "ZZZ"),
    follow(In, Start, End, Out).

part2(map{directions:D, nodes:N}, Steps) :-
    append(D, CyclicDirections, CyclicDirections),
    dict_keys(N, Keys),
    include(validStart, Keys, Starts),
    maplist(find_cycle(CyclicDirections,N), Starts, Cycles),
    length(D, InstructionLength),
    combine_cycles(InstructionLength, Cycles, Steps).

:- phrase_from_file(input(I), "day8.example.in"), part1(I, 2).
:- phrase_from_file(input(I), "day8.example2.in"), part1(I, 6).

:- phrase_from_file(input(I), "day8.in"), part1(I, 17287).%Out), print(Out).

:- phrase_from_file(input(I), "day8.example3.in"),  part2(I, 6).

:- phrase_from_file(input(I), "day8.in"), part2(I, 18625484023687).%Out), print(Out).