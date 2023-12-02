:- use_module(library(pio)).
:- use_module(library(lists)).
:- use_module(library(apply)).
:- set_prolog_flag(double_quotes, codes).

lines([])     --> call(eos), !.
lines([L|Ls]) --> line(L), lines(Ls).

line([])      --> ("\r\n" | "\n" | call(eos)), !.
line([C|Cs])  --> [C], line(Cs).

eos([],[]).

game([GameNum| Draws]) --> "Game ", digits(D), {number_chars(GameNum, D)}, ": ", !, draws(Draws).
digits([H|T]) --> [H], {char_type(H, digit)}, !, (digits(T) | {T=[]}).
draws([H|T]) --> draw(H), !, ("; ", !, draws(T) | {T=[]}).
draw([H|T]) --> draw_(H), !, (", ", !, draw(T) | {T=[]}).
draw_([Num, Kind]) --> digits(H), {number_chars(Num, H)}, " ", color(Kind).
color(red) --> "red".
color(green) --> "green".
color(blue) --> "blue".

toGame(Line, Game) :- phrase(game(Game), Line).

fitsIn(Red, Green, Blue, Draw) :-
    fitsInCol(Draw, Red, red),
    fitsInCol(Draw, Green, green),
    fitsInCol(Draw, Blue, blue).

fitsInCol(Draw, Max, Col) :- member([N, Col], Draw), !, N =< Max.
fitsInCol(Draw, _, Col) :- \+ member([_, Col], Draw).

legalGame([_|Draws]) :- forall(member(D, Draws), fitsIn(12, 13, 14, D)).

part1(L, Result):-
    maplist(toGame, L, Games),
    include(legalGame, Games, Legal),
    maplist(nth0(0), Legal, GameNumbers),
    sum_list(GameNumbers, Result).


minCol(Draw, Col, N) :-
    member([N, Col], Draw), !.
minCol(Draw, Col, 0) :-
    \+ member([_, Col], Draw).

minColors([], 0, 0, 0).
minColors([Draw|T], R, G, B) :-
    minCol(Draw, red, RD),
    minCol(Draw, green, GD),
    minCol(Draw, blue, BD),
    minColors(T, RT,GT, BT),
    R is max(RD, RT),
    G is max(GD, GT),
    B is max(BD, BT).

power([_|Draws], Power) :- 
    minColors(Draws, R, G, B),
    Power is R*G*B.

part2(L, Result):-
    maplist(toGame, L, Games),
    maplist(power, Games, Powers),
    sum_list(Powers, Result).


:- phrase_from_file(lines(L), "day2.example.in"), part1(L, 8).

:- phrase_from_file(lines(L), "day2.in"), part1(L, 2268).%, print(Out).

:- phrase_from_file(lines(L), "day2.example.in"), part2(L, 2286).

:- phrase_from_file(lines(L), "day2.in"), part2(L, 63542).%), print(Out).