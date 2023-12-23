:- load_files(['../aoc_lib.pl']).

pos_atom(p(X,Y), K) :- atomic_list_concat([X,:,Y], K).
atom_pos(K, p(X,Y)) :-
    atom_codes(K, Codes),
    append([XC,":", YC], Codes),
    number_codes(X, XC),
    number_codes(Y, YC).

input(G) --> grid(g{}, 0, G).
grid(GIn, Y, GOut) --> grid_line(GIn, Y, 0, GMid), {YY is Y+1}, ( grid(GMid, YY, GOut), ! | {GOut = GMid.put(ymax, YY), !}).
grid_line(GIn, Y, X, GOut) --> ".", {XX is X+1}, grid_line(GIn, Y, XX, GOut).
grid_line(GIn, Y, X, GOut) --> "S", {XX is X+1}, grid_line(GIn.put(start, p(X,Y)), Y, XX, GOut).
grid_line(GIn, Y, X, GOut) --> "#", {XX is X+1, pos_atom(p(X,Y), K)}, grid_line(GIn.put(K, wall), Y, XX, GOut).
grid_line(GIn, _, X, GOut) --> {X>0}, eol, {GOut = GIn.put(xmax, X)}.

:- table neighbor/3.
:- table blocked/2.

neighbor(G, p(X,Y), p(XX,Y)) :-
    (DX is -1; DX is +1),
    XX is X+DX,
    \+ blocked(G, p(XX,Y)).
neighbor(G, p(X,Y), p(X,YY)) :-
    (DY is -1; DY is +1),
    YY is Y+DY,
    \+ blocked(G, p(X,YY)).

blocked((G,XM,YM), p(X,Y)) :-
    XX is X mod XM,
    YY is Y mod YM,
    %(XX = X, YY=Y | trace),
    pos_atom(p(XX,YY), K),!,
    _ = G.get(K).

put_all([], S, S) :- !.
put_all([H|T], S0, S1) :-
    pos_atom(H, K),!,
    put_all(T, S0.put(K,x), S1).

one_step(G, S, S1) :-
    %dict_keys(S, R),
    one_step(G, S, [], S1).
one_step(_, [], SI, SO):- sort(SI,SO), !.
one_step(G, [P|T], S0, S2) :-
    bagof(N, neighbor(G, P, N), Ns),
    append(Ns, S0, S1), !, %put_all(Ns, S0, S1),!,
    one_step(G, T, S1, S2).

reachable(_, S, 0, S) :- !.
reachable(G, S, N, Reachable) :-
    NN is N-1,
    one_step(G, S, S1),!,
    (1 is N mod 10 -> 
        length(S1, L),
        print(N), write(": "), print(L), nl; true),
    reachable(G, S1, NN, Reachable).

part1(In, N, Out) :-
    Start = In.get(start),
    reachable((In, In.xmax, In.ymax), [Start], N, Reachable),
    length(Reachable, Out).%dict_size(Reachable, Out).

part2(In, N, Out) :-
    % Did not have the brain for this; after some looking online folks mention that the answer
    % follows a quadratic distribution; if you walk for K grids + half a grid (which happens to match the input)
    % then the reachable space is A*k^2 + B*k + C for some A,B,C.
    % Empirically we can find the first few entries, and work things out
    Period = In.get(xmax),
    Half is Period div 2,
    Start = In.get(start),
    Repeats is N div Period,
    reachable((In, In.xmax, In.ymax),[Start], Half, R1),!,
    reachable((In, In.xmax, In.ymax), R1, Period, R2),!,
    reachable((In, In.xmax, In.ymax), R2, Period, R3),!,
    length(R1, V1),
    length(R2, V2),
    length(R3, V3),
    % V1 = A*0 + B * 0 + C
    % V1 = C
    % V2 = A*1 + B * 1 + C
    % V3 = A*4 + B * 2 + C
    % V3 - 2*V2 = A*2 -C
    % V2 - A - C = B
    C is V1,
    A is (V3-2*V2 + C)/2,
    B is V2 - A - C,
    %print(V1), print(','), print(V2), print(','), print(V3), nl,
    %print(A), print(','), print(B), print(','), print(C), print(','),print(Repeats),nl,
    Out is A * Repeats * Repeats + B * Repeats + C.

:- phrase_from_file(input(I), "day21.example.in"), part1(I, 6, 16).

:- phrase_from_file(input(I), "day21.in"), part1(I, 64, 3687).%Out), print(Out).

:- phrase_from_file(input(I), "day21.example2.in"),  part1(I, 42, 1576).
:- phrase_from_file(input(I), "day21.example2.in"),  part1(I, 59, 3068).
:- phrase_from_file(input(I), "day21.example2.in"),  part1(I, 76, 5052).
:- phrase_from_file(input(I), "day21.example2.in"),  part2(I, 1180148, 1185525742508).

:- phrase_from_file(input(I), "day21.in"), part2(I, 26501365, 610321885082978).%Out), print(Out).
