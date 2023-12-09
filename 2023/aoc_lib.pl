:- use_module(library(pio)).
:- use_module(library(lists)).
:- use_module(library(apply)).
:- use_module(library(dicts)).
:- set_prolog_flag(double_quotes, codes).

lines([])     --> call(eos), !.
lines([L|Ls]) --> line(L), lines(Ls).

line([])      --> eol, !.
line([C|Cs])  --> [C], line(Cs).

eol --> ("\r\n" | "\n" | call(eos)), !.

spaces --> [].
spaces --> " ", spaces.

digits([H|T]) --> [H], {char_type(H, digit)}, !, (digits(T) | {T=[]}).
number(N) --> digits(D), {number_chars(N, D)}.
integer(I) --> ("-", {Adj = -1} | {Adj = 1}), number(N), {I is Adj * N}.
word([H|T]) --> [H], {char_type(H, alpha)}, !, (word(T) | {T=[]}).
word_atom(A) --> word(W), {atom_codes(A, W)}.


eos([],[]).
