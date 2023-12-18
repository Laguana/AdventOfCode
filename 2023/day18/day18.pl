:- load_files(['../aoc_lib.pl']).

input([H|T]) --> input_line(H), (eol, input(T),! | {T=[]}).
input_line(instr(D,N,H)) --> direction(D), " ", number(N), " (#", hex(H), ")", !.
direction(up) --> "U".
direction(up) --> "3".
direction(down) --> "D".
direction(down) --> "1".
direction(left) --> "L".
direction(left) --> "2".
direction(right) --> "R".
direction(right) --> "0".
hex(instr(D,N)) --> {length(V, 5)}, V, {append(["0x",V], L), number_codes(N,L)}, direction(D).

pos_key(p(X,Y),K) :- 
    atomic_list_concat([X,:,Y], K).
key_to_pos(K, p(X,Y)) :-
    atom_codes(K, Codes),
    append([XC,":", YC], Codes),
    number_codes(X, XC),
    number_codes(Y, YC).

step(up, p(X,Y), p(X,NY)) :- NY is Y-1.
step(down, p(X,Y), p(X,NY)) :- NY is Y+1.
step(left, p(X,Y), p(NX,Y)) :- NX is X-1.
step(right, p(X,Y), p(NX,Y)) :- NX is X+1.
step(up, D, p(X,Y), p(X,NY)) :- NY is Y-D.
step(down, D, p(X,Y), p(X,NY)) :- NY is Y+D.
step(left, D, p(X,Y), p(NX,Y)) :- NX is X-D.
step(right, D, p(X,Y), p(NX,Y)) :- NX is X+D.



dig_holes(Instructions, Result) :-
    Start = p(0,0),
    pos_key(Start, StartK),
    dig_holes(Instructions, p(0,0), dug{}.put(StartK, []), Result).
dig_holes([], _, Result, Result) :- !.
dig_holes([H|T], P, In, Out) :-
    dig_holes_(H, P, NP, In, Mid),!,
    dig_holes(T, NP, Mid, Out).
dig_holes_(instr(_,0,_), P, P, Out, Out) :- !.
dig_holes_(instr(D,N,H), P, OutP, In, Out) :-
    step(D, P, NP),
    NN is N-1,
    pos_key(NP, K),
    (LastState = In.get(K) -> Mid = In.put(K, [d(D,H)|LastState]);Mid = In.put(K, [d(D,H)])),
    dig_holes_(instr(D,NN,H), NP, OutP, Mid, Out).

bounds(L, Bounds) :-
    bounds(b(0,0,0,0), L, Bounds),!.
bounds(B, [], B).
bounds(b(LX,GX,LY,GY), [p(X,Y)|T], Bounds) :-
    TLX is min(X,LX),
    TGX is max(X,GX),
    TLY is min(Y,LY),
    TGY is max(Y,GY),!,
    bounds(b(TLX, TGX, TLY, TGY), T, Bounds).

col(X,p(X,_)).

neighbor(p(X,Y), p(XX,Y)) :-
    (DX is -1; DX is +1),
    XX is X+DX.
neighbor(p(X,Y), p(X,YY)) :-
    (DY is -1; DY is +1),
    YY is Y+DY.

has_entry(M, P) :-
    pos_key(P, K),
    _ = M.get(K).

fill_map(In, Out) :-
    dict_keys(In, Ks),
    maplist(key_to_pos,Ks, Ps),
    bounds(Ps, b(LX,_,_,_)),!,
    include(col(LX), Ps, Leftmost),
    sort(Leftmost, [_,p(X,Y)|_]),
    XX is X+1,
    fill_map([p(XX,Y)], In, Out).
fill_map([], Out, Out).
fill_map([P|T], In, Out) :-
    pos_key(P,K),
    (_ = In.get(K) -> Mid = In, TT = T
    ; 
        bagof(N, neighbor(P, N), Ns), 
        exclude(has_entry(In), Ns, NNs),
        Mid = In.put(K, filled),
        append(NNs, T, TT)
    ),!,
    fill_map(TT, Mid, Out).


print_map(M) :-
    dict_keys(M, Ks),
    maplist(key_to_pos,Ks, Ps),
    bounds(Ps, b(LX, GX, LY,GY)),
    between(LY,GY,Y),
    nl,
    between(LX,GX,X),
    pos_key(p(X,Y), K),
    (_ = M.get(K) -> write('#'); write('.')),fail.
print_map(_).

/*
part1(I, Out) :-
    dig_holes(I, Map),
    fill_map(Map, FilledMap),
    %print_map(FilledMap),
    dict_keys(FilledMap, Ks),
    length(Ks, Out).
*/

instruction_coords(L, Coords) :-
    instruction_coords(L, p(0,0), [p(0,0)], Coords).
instruction_coords([], _, Out, Out).
instruction_coords([instr(Dir, Delta)|T], P, Acc, Out) :-
    step(Dir, Delta, P, NP),!,
    instruction_coords(T, NP, [NP|Acc], Out).

shoelace_area(L, Area) :-
    reverse(L,RL),
    shoelace_area(RL, 0, Area).
shoelace_area([_], Acc, Area) :- Area is Acc/2.
shoelace_area([p(X,Y), p(X2,Y2)|T], Acc, Area) :-
    NAcc is Acc + X*Y2 - Y*X2,
    shoelace_area([p(X2,Y2)|T], NAcc, Area).

part1_instr(instr(D,N,_), instr(D,N)).
part2_instr(instr(_,_,I), I).

perimeter(Instructions, N) :-
    perimeter(Instructions, 0, N).
perimeter([], N, N).
perimeter([instr(_,D)|T], A, N) :-
    AN is D+A,
    perimeter(T, AN, N).

part1(In, Out) :-
    maplist(part1_instr, In, Instructions),
    instruction_coords(Instructions, Coords),
    perimeter(Instructions, P),
    shoelace_area(Coords, Area),
    Out is Area + P/2 + 1.

part2(In, Out) :-
    maplist(part2_instr, In, Instructions),
    instruction_coords(Instructions, Coords),
    perimeter(Instructions, P),
    shoelace_area(Coords, Area),
    Out is Area + P/2 + 1.
    

:- phrase_from_file(input(I), "day18.example.in"), part1(I, 62).

:- phrase_from_file(input(I), "day18.in"), part1(I, 61661).%Out), print(Out).

:- phrase_from_file(input(I), "day18.example.in"),  part2(I, 952408144115).

:- phrase_from_file(input(I), "day18.in"), part2(I, Out), print(Out).