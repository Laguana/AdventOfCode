:- load_files(['../aoc_lib.pl']).

input([H|T]) --> block(H), (call(eos), {T=[]} | input(T)).
block(block(X,Y,Z,XX,YY,ZZ)) --> number(X), ",", number(Y), ",", number(Z), "~", number(XX), ",", number(YY), ",", number(ZZ), eol.

ztag(block(X,Y,Z,XX,YY,ZZ), Z-block(X,Y,Z,XX,YY,ZZ)).

zsort_blocks(I,O) :-
    maplist(ztag, I, TI),
    sort(TI,STO),
    maplist(ztag, O, STO).

xy_key(X,Y,K) :- atomic_list_concat([X,:,Y], K).

starting_state_idx(Idx) :-
    bagof(I, between(0,9,I), Is),
    member(X,Is),
    member(Y,Is),
    xy_key(X,Y,Idx).
starting_state(S) :-
    bagof(I, starting_state_idx(I), Idxs),
    starting_state(b{}, Idxs, S),!.
starting_state(I, [], I).
starting_state(I, [H|T], O) :-
    starting_state(I.put(H, [[0-0]-floor]), T, O).

keys(XL,XG,YL,YG, K) :-
    between(XL,XG,X),
    between(YL,YG,Y),
    xy_key(X,Y, K).

add_block(B,[],_,B) :- !.
add_block(B, [H|T], X, Out) :-
    add_block(B.put(H, [X|B.get(H)]), T, X, Out).

max_height(_, [], V, V) :- !.
max_height(In, [H|T], Acc-AccL, Out) :-
    [[V-_]-S|_] = In.get(H),
    (V = Acc -> max_height(In, T, V-[S|AccL], Out)
    ; (V @> Acc -> 
        max_height(In, T, V-[S], Out)
    ;   max_height(In, T, Acc-AccL, Out))).
max_height(In, [H|T], V) :-
    [[Start-_]-SId|_] = In.get(H),
    max_height(In, T, Start-[SId], V).

drop_blocks(I, O) :-
    starting_state(Start),
    drop_blocks(I, 1, Start, [], O).
drop_blocks([], _, Dropped, Supports,result(Dropped,Supports)):- !.
drop_blocks([block(X,Y,Z,XX,YY,ZZ)| T], Id, In, Supports, Out) :-
    % Get the max height H among (X,Y)~(XX,YY), that is what we fall on top of
    % We are now H+1~(H+1+(ZZ-Z)) in (X,Y)~(XX,YY)
    bagof(K, keys(X,XX,Y,YY, K), Ks),
    max_height(In, Ks, H-Support),
    ZBottom is H+1,
    ZTop is H+1+(ZZ-Z),
    add_block(In, Ks, [ZTop-ZBottom]-Id, Mid),
    TId is Id + 1,!,
    drop_blocks(T, TId, Mid, [Id-block(X,Y,Z,XX,YY,ZZ)-Support|Supports],Out).

redundant_supports(R,[],R).
redundant_supports(I, [_-_-L|T], R) :-
    sort(L,SL),
    ([X] = SL 
    -> (select(X,I,IT) 
        -> redundant_supports(IT,T,R)
        ; redundant_supports(I,T,R)
        )
    ; redundant_supports(I,T,R)
    ).

part1(I,O) :-
    zsort_blocks(I,S),
    drop_blocks(S,result(_, Supports)),
    length(S, L),
    bagof(Id, between(1,L,Id), Ids),
    redundant_supports(Ids, Supports, Redundant),
    length(Redundant, O).

:- phrase_from_file(input(I), "day22.example.in"), part1(I, 5).

:- phrase_from_file(input(I), "day22.in"), part1(I, Out), print(Out).

%:- phrase_from_file(input(I), "day22.example.in"),  part2(I, ).

%:- phrase_from_file(input(I), "day22.in"), part2(I, Out), print(Out).