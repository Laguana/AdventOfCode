:- load_files(['../aoc_lib.pl']).

input([H|T]) --> digits(D), {maplist(num_digit, D, H)}, (eol, input(T),! | {T=[]}).

num_digit(C, N) :- N is C-48.

pos_key(p(X,Y,Dir,Run),K) :- 
    atomic_list_concat([X,:,Y,:,Dir,:,Run], K).

allowed_turn(left, up).
allowed_turn(left, down).
allowed_turn(right, up).
allowed_turn(right, down).
allowed_turn(up, left).
allowed_turn(up, right).
allowed_turn(down, left).
allowed_turn(down, right).
allowed_turn(start,_).

neighbor(X,Y,MX,_, DIn, Run, p(NX,Y, DOut, ROut)) :-
    (DX is -1, DOut = left; DX is +1, DOut = right),
    (DIn = DOut -> ROut is Run+1, ROut =< 3; ROut is 1, allowed_turn(DIn, DOut)),
    NX is X+DX, between(1,MX, NX).
neighbor(X,Y,_,MY, DIn, Run, p(X,NY, DOut, ROut)) :-
    (DY is -1, DOut = up; DY is +1, DOut = down),
    (DIn = DOut -> ROut is Run+1, ROut =< 3; ROut is 1, allowed_turn(DIn, DOut)),
    NY is Y+DY, between(1,MY,NY).

step_cost(Costs, p(X,Y,_,_), Cost) :-
    nth1(Y,Costs, Row),
    nth1(X,Row, Cost).

subsumed_by(p(F, X,Y,D,R), p(FF,X,Y,D,R)) :-
    F =< FF, print(p(F, X,Y,D,R)), write('<'), print(p(FF,X,Y,D,R)), nl.

insert_sorted(E,[],[E]).
insert_sorted(E,[H|T], [H|T]) :-
    subsumed_by(H,E),!.
insert_sorted(E,[H|T], [E|TT]) :-
    E @=< H,!,
    exclude(subsumed_by(E), [H|T], TT).
insert_sorted(E,[H|T], [H|TT]) :-
    %E @> H,
    !,
    insert_sorted(E,T, TT).

astar(Costs, p(X,Y), Cost) :-
    pos_key(p(X,Y, start, 0),K),
    length(Costs, TY),
    Costs = [Row|_],
    length(Row,TX),
    DX is abs(TX-X),
    DY is abs(TY-Y),
    H is DX+DY,
    !,
    astar(g(Costs,TX,TY), p(TX,TY), [p(0,X,Y,start,0)], states{}.put(K, state{pred:none, g:0, f:H}), _, Cost).

astar(_,_,[], _,_,_) :- !,fail.
astar(_,p(X,Y), [p(Cost,X,Y,_,_)|_], State, State, Cost) :- !.
astar(g(Costs,MX,MY), p(TX,TY), [p(_,X,Y,Dir,Run)|T], InState, OutState, OutCost) :-
    pos_key(p(X,Y,Dir,Run),K), 
    bagof(N, neighbor(X,Y, MX, MY, Dir, Run,N), Neighbors),
    State = InState.get(K),
    maplist(step_cost(Costs), Neighbors, StepCosts),
    astar_(Neighbors, StepCosts, p(TX, TY), p(X,Y,State.g, K), T, InState, TNext, NextState),!,
    astar(g(Costs,MX,MY), p(TX,TY), TNext, NextState, OutState, OutCost).

astar_([], [], _, _, Open, State, Open, State).
astar_([p(X,Y,Dir,Run)|T], [CH|CT], p(TX, TY), p(CX,CY,CG, CK), OIn, SIn, OOut, SOut) :-
    pos_key(p(X,Y,Dir,Run),K),
    G is CH + CG,
    DX is abs(TX-X),
    DY is abs(TY-Y),
    Delta is DX + DY,
    F is G+Delta,
    (State = SIn.get(K)
    -> (G < State.g 
        -> insert_sorted(p(F,X,Y,Dir,Run), OIn, ONext), SNext = SIn.put(K, state{pred:CK, g:G, f:F })
        ; ONext = OIn, SNext = SIn)
    ; insert_sorted(p(F,X,Y,Dir,Run), OIn, ONext), SNext = SIn.put(K, state{pred:CK, g:G, f:F })
    ),!,
    astar_(T, CT, p(TX, TY), p(CX,CY,CG, CK), ONext, SNext, OOut, SOut).

part1(I,O) :-
    astar(I, p(1,1), O).


:- phrase_from_file(input(I), "day17.example.in"), part1(I, 102).

%:- phrase_from_file(input(I), "day17.in"), part1(I, 635.%Out), print(Out).

%:- phrase_from_file(input(I), "day17.example.in"),  part2(I, ).

%:- phrase_from_file(input(I), "day17.in"), part2(I, Out), print(Out).