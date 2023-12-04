:- load_files(['../aoc_lib.pl']).

card(card(N, Winning, Numbers)) --> "Card ", spaces, digits(D), {number_chars(N, D)}, ": ", numbers(Winning), " | ", numbers(Numbers).
spaces --> [].
spaces --> " ", spaces.
numbers(L) --> " ", numbers(L).
numbers([H|T]) --> digits(D), {number_chars(H, D)}, !, (" ", numbers(T), ! | {T=[]}).


is_winning(Winning, N) :- member(N, Winning).

count_winning(card(_, Winning, Numbers), NWinning) :-
    include(is_winning(Winning), Numbers, WinningNumbers),
    length(WinningNumbers, NWinning).

score(Card, Score) :-
    count_winning(Card, NWinning),
    (NWinning = 0, Score = 0 | Score is 2**(NWinning-1)).

card_from_line(L, C) :- phrase(card(C), L).

part1(L, Out) :-
    maplist(card_from_line, L, Cards),
    maplist(score, Cards, Scores),
    sum_list(Scores, Out).

add_winnings(0, _, L, L).
add_winnings(NWon, NDupes, [H|T], [HOut|TOut]) :-
    NR is NWon-1,
    add_winnings(NR, NDupes, T, TOut),
    HOut is H+NDupes+1.
add_winnings(NWon, NDupes, [], Out) :-
    length(Out, NWon),
    ODupes is NDupes+1,
    maplist(=(ODupes), Out).
    

sum_cards(L, Out) :-
    sum_cards(L, [0], Out).
sum_cards([], _, 0).
sum_cards([H|T], [DH|DT], Out) :-
    add_winnings(H, DH, DT, NDT),
    sum_cards(T, NDT, TOut),
    Out is TOut+DH+1.
sum_cards([H|T], [], Out) :-
    add_winnings(H, 0, [], NDT),
    sum_cards(T, NDT, TOut),
    Out is TOut+1.

part2(L, Out) :-
    maplist(card_from_line, L, Cards),
    maplist(count_winning, Cards, NWinning),
    sum_cards(NWinning, Out).

:- phrase_from_file(lines(L), "day4.example.in"), part1(L, 13).

:- phrase_from_file(lines(L), "day4.in"), part1(L, 26218).%Out), print(Out).

:- phrase_from_file(lines(L), "day4.example.in"), part2(L, 30).

:- phrase_from_file(lines(L), "day4.in"), part2(L, 9997537).%Out), print(Out).