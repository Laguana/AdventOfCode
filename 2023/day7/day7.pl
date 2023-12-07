:- load_files(['../aoc_lib.pl']).

hands([hand{hand:Hand, bid:Bid, type:Type}|T]) --> hand(Hand), {hand_type(Hand, Type)}, " ", number(Bid), eol, (hands(T), ! | {T=[]}).
hand([A,B,C,D,E]) --> card(A), card(B), card(C), card(D), card(E).
card(1) --> "2".
card(2) --> "3".
card(3) --> "4".
card(4) --> "5".
card(5) --> "6".
card(6) --> "7".
card(7) --> "8".
card(8) --> "9".
card(9) --> "T".
card(10) --> "J".
card(11) --> "Q".
card(12) --> "K".
card(13) --> "A".

multiplicity(_, [], 0).
multiplicity(E, [E|T], V) :- !, multiplicity(E, T, VT), V is VT+1.
multiplicity(E, [_|T], V) :- multiplicity(E, T, V).


% What if we just didn't do any fanciness!
% This works, but is less efficient
hand_type_prologgy(Hand, Type) :-
    msort(Hand, Sorted),
    hand_type_msorted(Sorted, Type).
hand_type_msorted([C,C,C,C,C], 7):- !.
hand_type_msorted(H, 6) :-
    permutation(H, [X,X,X,X,_]), !.
hand_type_msorted(H, 5) :-
    permutation(H, [X,X,X,Y,Y]), !.
hand_type_msorted(H, 4) :-
    permutation(H, [X,X,X,_,_]), !.
hand_type_msorted(H, 3) :-
    permutation(H, [X,X,Y,Y,_]), !.
hand_type_msorted(H, 2) :-
    permutation(H, [X,X,_,_,_]), !.
hand_type_msorted(_, 1).


hand_type(Hand, Type) :- 
    sort(Hand, Sorted), % Note: Also deduplicates
    hand_type_sorted(Sorted, Hand, Type).
hand_type_sorted([_], _, 7). % 5 of a kind
hand_type_sorted([A,_], Hand, Type) :-
    % Could be 4 of a kind or full house
    multiplicity(A, Hand, Repeats),
    ((Repeats = 4; Repeats = 1)
     -> Type = 6 % 4 of a kind
    ; Type = 5). % Full house
hand_type_sorted([A,B,_], Hand, Type) :-
    % Could be 3 of a kind or two pair
    multiplicity(A, Hand, NA),
    multiplicity(B, Hand, NB),
    (NA = 1, NB=1; NA=3; NB=3) 
        -> Type=4 % 3 of a kind
    ; Type=3. % 2 pair
hand_type_sorted([_,__,_,_], _, 2). % one pair
hand_type_sorted([_,_,_,_,_], _, 1). % high card

is_type(T,H) :- H.type = T.

score_type([], R, R, 0).
score_type([H|T], R, ROut, Score) :-
    Value is H.bid * R,
    NR is R+1,
    score_type(T, NR, ROut, NScore),
    Score is NScore + Value.

score_hands(Hands, Score) :- score_hands(Hands, 1, 1, 0, Score).
score_hands([], _, _, X, X).
score_hands(L, Type, Rank, Acc, Score) :-
    partition(is_type(Type), L, TL, TNext),
    msort(TL, Sorted),
    score_type(Sorted, Rank, NextRank, TypeScore),
    TAcc is Acc + TypeScore,
    TType is Type + 1,
    score_hands(TNext, TType, NextRank, TAcc, Score).

score_hands([H|T], Rank, Acc, Out) :-
    TAcc is Acc + H.bid * Rank,
    TRank is Rank+1,
    score_hands(T, TRank, TAcc, Out).

part1(Hands, Out) :-
    score_hands(Hands, Out).

joker_hand([], []).
joker_hand([10|T], [0|JT]) :- !, joker_hand(T, JT).
joker_hand([H|T], [H|JT]) :- joker_hand(T, JT).

joker_hand_type(Hand, JokerType) :-
    multiplicity(0, Hand, NJokers),
    hand_type(Hand, Type),
    joker_hand_type(Type,NJokers, JokerType).
joker_hand_type(7, _, 7).
joker_hand_type(6, _, 7). % we assume at least one joker
joker_hand_type(5, _, 7). % 3/2 becomes 5 regardless
joker_hand_type(4, _, 6). % 3 of a kind becomes a 4
joker_hand_type(3, 2, 6). % 2 pair with one of the pairs being jokers is 4 of a kind
joker_hand_type(3, 1, 5). % otherwise full house
joker_hand_type(2, _, 4). % one pair becomes 3 of a kind
joker_hand_type(1, _, 2). 

jokerize(hand{hand:Hand, bid:Bid, type:_}, Out) :-
    member(10, Hand), !,
    joker_hand(Hand, JokerHand),
    joker_hand_type(JokerHand, JokerType),
    Out = hand{hand:JokerHand, bid: Bid, type: JokerType}.
jokerize(H, H). % no jokers

part2(Hands, Out) :-
    maplist(jokerize, Hands, JokerdHands),
    score_hands(JokerdHands, Out).

:- phrase_from_file(hands(H), "day7.example.in"), part1(H, 6440).

:- phrase_from_file(hands(H), "day7.in"), part1(H, 251806792).%Out), print(Out).

:- phrase_from_file(hands(H), "day7.example.in"), part2(H, 5905).

:- phrase_from_file(hands(H), "day7.in"), part2(H, 252113488).%Out), print(Out).