:- use_module(library(pio)).
:- use_module(library(lists)).
:- use_module(library(apply)).
:- set_prolog_flag(double_quotes, codes).

lines([])     --> call(eos), !.
lines([L|Ls]) --> line(L), lines(Ls).

line([])      --> eol, !.
line([C|Cs])  --> [C], line(Cs).

eol --> ("\r\n" | "\n" | call(eos)), !.

digits([H|T]) --> [H], {char_type(H, digit)}, !, (digits(T) | {T=[]}).
number(N) --> digits(D), {number_chars(N, D)}.
word([H|T]) --> [H], {char_type(H, alpha)}, !, (word(T) | {T=[]}).

eos([],[]).
