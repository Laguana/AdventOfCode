:- use_module(library(pio)).
:- use_module(library(lists)).
:- use_module(library(apply)).
:- set_prolog_flag(double_quotes, codes).

lines([])     --> call(eos), !.
lines([L|Ls]) --> line(L), lines(Ls).

line([])      --> ("\r\n" | "\n" | call(eos)), !.
line([C|Cs])  --> [C], line(Cs).

digits([H|T]) --> [H], {char_type(H, digit)}, !, (digits(T) | {T=[]}).

eos([],[]).
