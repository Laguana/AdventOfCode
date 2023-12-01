
:- use_module(library(pio)).
:- set_prolog_flag(double_quotes, codes).

lines([])     --> call(eos), !.
lines([L|Ls]) --> line(L), lines(Ls).

line([])      --> ("\n" | call(eos)), !.
line([C|Cs])  --> [C], line(Cs).

eos([],[]).

number_word(one, 1).
number_word(two, 2).
number_word(three, 3).
number_word(four, 4).
number_word(five, 5).
number_word(six, 6).
number_word(seven, 7).
number_word(eight, 8).
number_word(nine, 9).

numbers_in_line([],[]).
numbers_in_line([H|T], [HNew| Rest]) :- 
    char_type(H, digit), number_chars(HNew, [H]), !, numbers_in_line(T, Rest).
numbers_in_line([H|T], Rest) :-
    (\+ char_type(H, digit)), numbers_in_line(T, Rest).

numbers_in_line2([],[]).
numbers_in_line2([H|T], [HNew| Rest]) :- 
    char_type(H, digit), number_chars(HNew, [H]), !, numbers_in_line2(T, Rest).
numbers_in_line2(L, [HNew | Rest]) :-
    number_word(Word, HNew), atom_codes(Word, WordCodes), append(WordCodes, _, L), !, L=[_|Tail], numbers_in_line2(Tail, Rest).
numbers_in_line2([H|T], Rest) :-
    (\+ char_type(H, digit)), numbers_in_line2(T, Rest).

first_and_last([H|T], Out) :- first_and_last_([H|T], H, Out).
first_and_last_([Last], H, Out) :- Out is 10 * H + Last, !.
first_and_last_([_|T], H, Out) :- first_and_last_(T, H, Out).

sum_list([], 0).
sum_list([H|T], Out) :- sum_list(T, Out2), Out is H+Out2.


part1(Input, Result, LineNumbers) :-
    maplist(numbers_in_line, Input, Numbers),
    maplist(first_and_last, Numbers, LineNumbers),
    sum_list(LineNumbers, Result).

part2(Input, Result, LineNumbers) :-
    maplist(numbers_in_line2, Input, Numbers),
    maplist(first_and_last, Numbers, LineNumbers),
    sum_list(LineNumbers, Result).

diff([], [], _).
diff([H|L], [H|R], I) :- I2 is I+1, diff(L,R,I2).
diff([HL|TL], [HR|TR], I) :- print([HL, HR, I]), nl, I2 is I+1, diff(TL, TR, I2).
diff([], R, I) :- print([l_short, R, I]).
difF(L, [], I) :- print([r_short, L, I]).

:- phrase_from_file(lines(L), "day1.example.in"), part1(L, 142, _).

:- phrase_from_file(lines(L), "day1.in"), part1(L, 55172, _).%, print(Out).

:- phrase_from_file(lines(L), "day1.example2.in"), part2(L, 281, _).

:- phrase_from_file(lines(L), "day1.in"), part2(L, 54925, _).%, print(Out), print(LineNumbers).