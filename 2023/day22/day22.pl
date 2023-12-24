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
    sort(Support,SSupport),
    ZBottom is H+1,
    ZTop is H+1+(ZZ-Z),
    add_block(In, Ks, [ZTop-ZBottom]-Id, Mid),
    TId is Id + 1,!,
    drop_blocks(T, TId, Mid, [Id-block(X,Y,Z,XX,YY,ZZ)-SSupport|Supports],Out).

redundant_supports(R,[],R).
redundant_supports(I, [_-_-L|T], R) :-
    ([X] = L 
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


number_supported(L, Out) :-
    number_supported(L, [], 0, Out).
number_supported([], _, Acc, Acc).
number_supported([H|T], Above, Acc, Out) :-
    H = Id-_-Support,
    fall_when_removed(Id, Above, V),
    %(V \= 0 -> print(Id), print(=), print(V),nl;true),
    TAcc is Acc + V,
    number_supported(T, [Id-Support|Above], TAcc, Out).

fall_when_removed(Id, Above, V) :-
    fall_when_removed_([Id], Above, V).
fall_when_removed_(_,[], 0).
fall_when_removed_(L, [Id-Support|T], V) :-
    subset(Support, L),!,
    fall_when_removed_([Id|L], T, V0),
    V is V0 + 1.
fall_when_removed_(L, [_|T], V) :-
    !,
    fall_when_removed_(L, T, V).


part2(I,O) :-
    zsort_blocks(I,S),
    drop_blocks(S,result(_, Supports)),
    number_supported(Supports, O).

:- phrase_from_file(input(I), "day22.example.in"), part1(I, 5).

:- phrase_from_file(input(I), "day22.in"), part1(I, 411).%Out), print(Out).

:- phrase_from_file(input(I), "day22.example.in"),  part2(I, 7).

:- phrase_from_file(input(I), "day22.example2.in"),  part2(I, 1).

:- phrase_from_file(input(I), "day22.in"), part2(I, 47671).%Out), print(Out).