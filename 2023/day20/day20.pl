:- load_files(['../aoc_lib.pl']).

input(M) --> rules(wires{}, M).
rules(In, Out) --> "broadcaster -> ", destinations(D), eol, rules(In.put(broadcaster, rule(broadcast,D)), Out).
rules(In, Out) --> kind(K), word_atom(Name), " -> ", destinations(D),
    {Next=In.put(Name, rule(K,D))},eol, (rules(Next,Out), ! | {Out = Next}).
destinations([H|T]) --> word_atom(H), (", ", !, destinations(T) | {T=[]}).
kind(flop) --> "%".
kind(conj) --> "&".

rule_outputs(_-rule(_,D)) :- D.

get_state(M, S) :-
    dict_pairs(M, _, P),
    get_state(P, s{}, S).
get_state([], S, S).
get_state([Name-rule(K,D)|T], In, Out) :-
    get_state(D, Name, In, Mid),
    (K = flop -> 
        (CurState = Mid.get(Name) -> NextState = Mid.put(Name, CurState.put(flop_, off)); NextState = Mid.put(Name, s{flop_: off}) )
        ;NextState = Mid
    ),!,
    get_state(T, NextState, Out).
get_state([],_, S, S).
get_state([H|T], From, In, Out) :-
    (CurState = In.get(H) -> MidState = In.put(H, CurState.put(From, low)); MidState = In.put(H, s{}.put(From,low))),!,
    get_state(T, From, MidState, Out).

tag(From,V, E, From-E-V).

flipflop(on, off, low).
flipflop(off, on, high).

evolve(M, S, SOut, Counts) :-
    evolve(M, S, [button-broadcaster-low], counts(1,0,0,0), SOut, Counts).
evolve(_, S, [], Counts, S, Counts).
evolve(M, SIn, [_-N-V|T], counts(Low,High,RL,RH), SOut, CountsOut) :-
    (\+ rule(_, _) = M.get(N)),
    (N=rx -> (V=low -> TRL is RL+1, TRH=RH; TRL = RL, TRH is RH+1); TRL=RL, TRH=RH),
    evolve(M, SIn, T, counts(Low,High,TRL,TRH), SOut, CountsOut).
evolve(M, SIn, [From-N-V|T], Counts, SOut, CountsOut) :-
    rule(Kind, Dests) = M.get(N),
    evolve(M, SIn, Kind, From-N-V, Dests, T, Counts, SOut, CountsOut).
evolve(M, SIn, broadcast, _-N-V, Dests, T, counts(Low, High,RL,RH), SOut, CountsOut) :-
    length(Dests, LD),
    (V = low -> NHigh = High, NLow is Low + LD; NLow = Low, NHigh is High + LD),
    maplist(tag(N,V), Dests, VD),
    append(T, VD, NT),!,
    evolve(M, SIn, NT, counts(NLow, NHigh, RL,RH), SOut, CountsOut).
evolve(M, SIn, flop, _-_-high, _, T, Counts, SOut, CountsOut) :-
    evolve(M, SIn, T, Counts, SOut, CountsOut).
evolve(M, SIn, flop, _-N-low, Dests, T, counts(Low, High,RL,RH), SOut, CountsOut) :-
    State = SIn.get(N),
    flipflop(State.get(flop_), NewState, Pulse),
    SMid = SIn.put(N, State.put(flop_, NewState)),
    length(Dests, LD),
    (Pulse = low -> NLow is Low + LD, NHigh = High; NLow = Low, NHigh is High + LD),
    maplist(tag(N, Pulse), Dests, VD),
    append(T, VD, NT),!,
    evolve(M, SMid, NT, counts(NLow, NHigh,RL,RH), SOut, CountsOut).
evolve(M, SIn, conj, From-N-V, Dests, T, counts(Low, High,RL,RH), SOut, CountsOut) :-
    State = SIn.get(N),
    Memory = State.put(From, V),
    SMid = SIn.put(N, Memory),
    dict_pairs(Memory, _, P),
    length(Dests, LD),
    (member(_-low, P)
     -> Pulse = high, NLow = Low, NHigh is High + LD
      ; Pulse = low, NLow is Low + LD, NHigh = High),
    maplist(tag(N, Pulse), Dests, DV),
    append(T, DV, NT),!,
    evolve(M, SMid, NT, counts(NLow, NHigh,RL,RH), SOut, CountsOut).

flip(M, S, N, Out) :-
    flip(M, S, N, [S], [counts(0,0)], Out).
flip(_, _, 0, _, [H|_],H) :- !.
flip(M, S, N, Trail, [counts(Low, High)|TCounts], Out) :-
    Counts = [counts(Low, High)|TCounts],
    evolve(M, S, Sn, counts(SLow, SHigh,_,_)),!,
    NLow is Low + SLow, NHigh is High + SHigh,
    (nth1(Period, Trail, Sn) 
        -> Cycles is (N-1) div Period,
           Remainder is (N-1) mod Period,
           nth1(Period, Counts, counts(PLow, PHigh)),
           CycleLow = NLow - PLow,
           CycleHigh = NHigh - PHigh,
           RemIdx is Period-Remainder,
           nth1(RemIdx, Counts, counts(RLow, RHigh)),
           (Remainder = 0 -> RemLow = 0, RemHigh = 0; RemLow is RLow - PLow, RemHigh is RHigh - PHigh),
           OutLow is NLow + Cycles*CycleLow + RemLow,
           OutHigh is NHigh + Cycles*CycleHigh + RemHigh,
           Out = counts(OutLow, OutHigh),!
        ; NN is N-1,!,
          flip(M, Sn, NN, [Sn|Trail], [counts(NLow, NHigh)|Counts], Out)
    ).

part1(I,O) :-
    get_state(I,S),
    flip(I,S,1000, counts(Low,High)),
    O is Low * High.

find_rx(M,S,O) :-
    find_rx(M,S,0,O).
find_rx(M,S,N,O) :-
    NN is N+1,
    evolve(M,S,Sn, counts(_,_,RL,RH)),!,
    %print(NN), print(:), print(c(RL,RH)), nl,
    (RH=0, RL = 1 -> O + NN; find_rx(M,Sn, NN, O)).

part2(I,O) :-
    get_state(I,S),
    find_rx(I,S,O).


:- phrase_from_file(input(I), "day20.example.in"), part1(I, 32000000).
:- phrase_from_file(input(I), "day20.example2.in"), part1(I, 11687500).

:- phrase_from_file(input(I), "day20.in"), part1(I, 684125385).%Out), print(Out).

%:- phrase_from_file(input(I), "day20.example.in"),  part2(I, ).

%:- phrase_from_file(input(I), "day20.in"), part2(I, Out), print(Out).
