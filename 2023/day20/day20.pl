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

evolve(M, S, SOut, Counts, WIn, WOut) :-
    evolve(M, S, [button-broadcaster-low], counts(1,0), SOut, Counts, WIn, WOut).
evolve(M, S, SOut, Counts) :-
    evolve(M, S, SOut, Counts, w{}, _).
evolve(_, S, [], Counts, S, Counts, Watch,Watch).
evolve(M, SIn, [_-N-_|T], Counts, SOut, CountsOut, WIn, WOut) :-
    (\+ rule(_, _) = M.get(N)),
    evolve(M, SIn, T, Counts, SOut, CountsOut, WIn, WOut).
evolve(M, SIn, [From-N-V|T], Counts, SOut, CountsOut, WIn, WOut) :-
    rule(Kind, Dests) = M.get(N),
    evolve(M, SIn, Kind, From-N-V, Dests, T, Counts, SOut, CountsOut, WIn, WOut).
evolve(M, SIn, broadcast, _-N-V, Dests, T, counts(Low, High), SOut, CountsOut, WIn, WOut) :-
    length(Dests, LD),
    (V = low -> NHigh = High, NLow is Low + LD; NLow = Low, NHigh is High + LD),
    maplist(tag(N,V), Dests, VD),
    append(T, VD, NT),!,
    evolve(M, SIn, NT, counts(NLow, NHigh), SOut, CountsOut, WIn, WOut).
evolve(M, SIn, flop, _-_-high, _, T, Counts, SOut, CountsOut, WIn, WOut) :-
    evolve(M, SIn, T, Counts, SOut, CountsOut, WIn, WOut).
evolve(M, SIn, flop, _-N-low, Dests, T, counts(Low, High), SOut, CountsOut, WIn, WOut) :-
    State = SIn.get(N),
    flipflop(State.get(flop_), NewState, Pulse),
    SMid = SIn.put(N, State.put(flop_, NewState)),
    length(Dests, LD),
    (Pulse = low -> NLow is Low + LD, NHigh = High; NLow = Low, NHigh is High + LD),
    maplist(tag(N, Pulse), Dests, VD),
    append(T, VD, NT),!,
    evolve(M, SMid, NT, counts(NLow, NHigh), SOut, CountsOut, WIn, WOut).
evolve(M, SIn, conj, From-N-V, Dests, T, counts(Low, High), SOut, CountsOut, WIn, WOut) :-
    State = SIn.get(N),
    Memory = State.put(From, V),
    SMid = SIn.put(N, Memory),
    dict_pairs(Memory, _, P),
    length(Dests, LD),
    (member(_-low, P)
     -> Pulse = high, NLow = Low, NHigh is High + LD
      ; Pulse = low, NLow is Low + LD, NHigh = High),
    maplist(tag(N, Pulse), Dests, DV),
    (V = high, _ = WIn.get(From) -> WNext = WIn.put(From, 1); WNext = WIn),
    append(T, DV, NT),!,
    evolve(M, SMid, NT, counts(NLow, NHigh), SOut, CountsOut, WNext, WOut).

flip(M, S, N, Out) :-
    flip(M, S, N, [S], [counts(0,0)], Out).
flip(_, _, 0, _, [H|_],H) :- !.
flip(M, S, N, Trail, [counts(Low, High)|TCounts], Out) :-
    Counts = [counts(Low, High)|TCounts],
    evolve(M, S, Sn, counts(SLow, SHigh)),!,
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

starter_dict([],D,D).
starter_dict([H|T],In, Out) :-
    starter_dict(T, In.put(H,0), Out).

update(N, DIn, Touched, DOut) :-
    %print(Touched), nl,
    dict_pairs(Touched, _, KS),
    update_(N,KS, DIn, DOut).
update_(_, [], D, D).
update_(N, [_-0|T], DIn, DOut) :-
    !,
    update_(N,T,DIn, DOut).
update_(N, [H-1|T], DIn, DOut) :-
    V = DIn.get(H),
    (V = 0 -> DMid = DIn.put(H, N); DMid = DIn),
    update_(N,T,DMid, DOut).

gcd(X, 0, X):- !.
gcd(0, X, X):- !.
gcd(X, Y, D):- X =< Y, !, Z is Y - X, gcd(X, Z, D).
gcd(X, Y, D):- gcd(Y, X, D).

final(D, O) :-
    dict_pairs(D, _, P),
    final(P,1,O).
final([], O,O).
final([_-H|T], A,O) :-
    (H \= 0),
    gcd(H,A,G),
    NA is (H * A div G),
    final(T,NA,O).


find_rx(M,S,O) :-
    RXI = S.get(rx),
    dict_keys(RXI, [RXPred]),
    RXPredI = S.get(RXPred),
    dict_keys(RXPredI, RXPredInputs),
    starter_dict(RXPredInputs, d{}, Z),
    find_rx(M, RXPred ,S,0,Z,O).
find_rx(M,RXPred, S,N,Z,O) :-
    NN is N+1,
    dicts_to_same_keys([d{}, Z], dict_fill(0), [SZ|_]),
    evolve(M,S,SN,_, SZ, SZO),!,
    update(NN, Z,SZO,NZ),
    %print(NN), print(:), print(NZ),nl,
    (final(NZ, O),!;find_rx(M,RXPred,SN,NN,NZ,O)).

part2(I,O) :-
    get_state(I,S),
    find_rx(I,S,O).

:- phrase_from_file(input(I), "day20.example.in"), part1(I, 32000000).
:- phrase_from_file(input(I), "day20.example2.in"), part1(I, 11687500).

:- phrase_from_file(input(I), "day20.in"), part1(I, 684125385).%Out), print(Out).

%:- phrase_from_file(input(I), "day20.example.in"),  part2(I, ).

:- phrase_from_file(input(I), "day20.in"), part2(I, 225872806380073).%Out), print(Out).
