:- load_files(['../aoc_lib.pl']).

input([H|T]) --> digits(D), {maplist(num_digit, D, H)}, (eol, input(T),! | {T=[]}).

num_digit(C, N) :- N is C-48.

pos_key(p(X,Y,Dir),K) :- 
    atomic_list_concat([X,:,Y,:,Dir], K).

allowed_turn(left, up).
allowed_turn(left, down).
allowed_turn(right, up).
allowed_turn(right, down).
allowed_turn(up, left).
allowed_turn(up, right).
allowed_turn(down, left).
allowed_turn(down, right).
allowed_turn(start,_).

neighbor(X,Y,g(Costs, MX, _, PX), DIn, p(NX,Y, DOut, StepCost)) :-
    % neighbors reflect the full extent of a run in a direction
    (DX is -1, DOut = left; DX is +1, DOut = right),
    allowed_turn(DIn, DOut),
    (PX = p1 -> between(1,3, RX); between(4,10,RX)),
    NX is X + (DX*RX),
    between(1,MX,NX),
    step_cost(Costs, p(X,Y), DX, 0, RX, 0, StepCost).
neighbor(X,Y,g(Costs, _, MY, PX), DIn, p(X,NY, DOut, StepCost)) :-
    % neighbors reflect the full extent of a run in a direction
    (DY is -1, DOut = up; DY is +1, DOut = down),
    allowed_turn(DIn, DOut),
    (PX = p1 -> between(1,3, RY); between(4,10,RY)),
    NY is Y + (DY*RY),
    between(1,MY,NY),
    step_cost(Costs, p(X,Y), 0, DY, RY, 0, StepCost).

step_cost(_Costs, _Point, _DX, _DY, 0, Cost, Cost) :- !.
step_cost(Costs, p(X,Y), DX, DY, R, Acc, Cost) :-
    NX is X + DX,
    NY is Y + DY,
    NRX is R-1,!,
    nth1(NY, Costs, Row),
    nth1(NX, Row, SCost),
    NAcc is Acc + SCost,!,
    step_cost(Costs, p(NX, NY), DX, DY, NRX, NAcc, Cost).

/*
step_cost(Costs, p(X,Y,_,_), Cost) :-
    nth1(Y,Costs, Row),
    nth1(X,Row, Cost).
*/

insert_sorted(E,[],[E]).
insert_sorted(E,[H|T], [E,H|T]) :-
    E @=< H,!.
insert_sorted(E,[H|T], [H|TT]) :-
    %E @> H,
    !,
    insert_sorted(E,T, TT).

astar(Costs, p(X,Y), Cost, PX) :-
    pos_key(p(X,Y, start),K),
    length(Costs, TY),
    Costs = [Row|_],
    length(Row,TX),
    DX is abs(TX-X),
    DY is abs(TY-Y),
    H is DX+DY,
    !,
    singleton_heap(Heap, 0, p(X,Y,start)),
    %Heap=[p(0,X,Y,start)],
    astar(g(Costs,TX,TY, PX), p(TX,TY), Heap, states{}.put(K, state{pred:none, g:0, f:H}), _, Cost).

astar(_,_,Heap, _,_,_) :- empty_heap(Heap), !,fail.
astar(_,p(X,Y), Heap, State, State, Cost) :- 
    get_from_heap(Heap, Cost, p(X,Y,_),_),
    %Heap=[p(Cost,X,Y,_)|_],
    !.
astar(g(Costs,MX,MY, PX), p(TX,TY), Heap, InState, OutState, OutCost) :-
    get_from_heap(Heap, _, p(X,Y,Dir), THeap),
    %Heap = [p(_,X,Y,Dir)|THeap],
    pos_key(p(X,Y,Dir),K), 
    (bagof(N, neighbor(X,Y, g(Costs, MX, MY, PX), Dir, N), Neighbors); Neighbors = []),!,
    State = InState.get(K),
    astar_(Neighbors, p(TX, TY), p(X,Y,State.g, K), THeap, InState, TNext, NextState),!,
    astar(g(Costs,MX,MY, PX), p(TX,TY), TNext, NextState, OutState, OutCost).

astar_([], _, _, Open, State, Open, State).
astar_([p(X,Y,Dir,Cost)|T], p(TX, TY), p(CX,CY,CG, CK), OIn, SIn, OOut, SOut) :-
    pos_key(p(X,Y,Dir),K),
    G is Cost + CG,
    DX is abs(TX-X),
    DY is abs(TY-Y),
    Delta is DX + DY,
    F is G+Delta,
    (State = SIn.get(K)
    -> (G < State.g 
        -> /*insert_sorted(p(F,X,Y,Dir), OIn, ONext)*/ add_to_heap(OIn, F, p(X,Y,Dir), ONext), SNext = SIn.put(K, state{pred:CK, g:G, f:F })
        ; ONext = OIn, SNext = SIn)
    ; /*insert_sorted(p(F,X,Y,Dir), OIn, ONext)*/ add_to_heap(OIn, F, p(X,Y,Dir), ONext), SNext = SIn.put(K, state{pred:CK, g:G, f:F })
    ),!,
    astar_(T, p(TX, TY), p(CX,CY,CG, CK), ONext, SNext, OOut, SOut).

part1(I,O) :-
    astar(I, p(1,1), O,p1).
part2(I,O) :-
    astar(I, p(1,1), O, p2).


:- phrase_from_file(input(I), "day17.example.in"), part1(I, 102).

:- phrase_from_file(input(I), "day17.in"), call_time(part1(I, 635), Time), print(p1), print(Time), nl.%Out), print(Out).

:- phrase_from_file(input(I), "day17.example.in"),  part2(I, 94).

:- phrase_from_file(input(I), "day17.in"), call_time(part2(I, 734), Time), print(p2), print(Time), nl.%Out), print(Out).