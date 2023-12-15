:- load_files(['../aoc_lib.pl']).

input([H|T]) --> step(H), (",", input(T) | call(eol), {T=[]} ).
step([C|T]) --> [C], { \+ [C] = ","}, {\+ [C] = "\r"}, (step(T), ! | {T=[]}).

instruction(remove(Label)) --> label(Label), "-".
instruction(add(Label, Focal)) --> label(Label), "=", number(Focal).
label([H|T]) --> [H], {\+ member(H, "-=,")}, (label(T),! | {T=[]}).

hash(S,V) :-
    hash(S,0,V)
    .%,string_codes(SS, S), print(SS),print('='), print(V),nl.
hash([],A,A).
hash([H|T], A, V) :-
    AT is ((H+A)*17) mod 256,
    hash(T,AT, V).

part1(I, O) :-
    maplist(hash, I, V),
    sum_list(V, O).

parse_step(S, P) :-
    phrase(instruction(P), S).

score_box(Boxes, Key, Score) :-
    Box = Boxes.get(Key),
    reverse(Box, RBox),
    score_box(RBox, 1, 0, Score_),
    %print(Key),print(Box), print(Score_), nl,
    Score is Score_ * (Key+1).
score_box([], _, Score, Score).
score_box([lens(_,Focal)|T], Index, Acc, Score) :-
    TIndex = Index + 1,
    TAcc is Acc + (Index * Focal),
    score_box(T, TIndex, TAcc, Score).

run_instructions(I, O) :-
    run_instructions(I, boxes{}, O).
run_instructions([], Boxes, O) :-
    %print(Boxes),nl,

    dict_keys(Boxes, Keys),
    maplist(score_box(Boxes),Keys, Scores),
    sum_list(Scores, O).
run_instructions([remove(Label)|T], Boxes, O) :-
    %print(Boxes), print(remove(Label)), nl,
    hash(Label, V),
    (VBox = Boxes.get(V) 
    -> (
        select(lens(Label,_), VBox, VTBox),!
        ;VTBox = VBox),
        TBoxes = Boxes.put(V, VTBox)
    ;Boxes = TBoxes),
    run_instructions(T, TBoxes, O).
run_instructions([add(Label,Focal)|T], Boxes, O) :-
    %print(Boxes), print(add(Label, Focal)), nl,
    hash(Label, V),
    (VBox = Boxes.get(V),!;VBox = []),
    (select(lens(Label,_), VBox, lens(Label,Focal), VTBox),!;VTBox = [lens(Label,Focal)|VBox]),
    TBoxes = Boxes.put(V, VTBox),
    run_instructions(T, TBoxes, O).

part2(I, O) :-
    maplist(parse_step, I, Instructions),
    run_instructions(Instructions, O).


:- phrase_from_file(input(I), "day15.example.in"), part1(I, 1320).

:- phrase_from_file(input(I), "day15.in"), part1(I, 512950).%Out), print(Out).

:- phrase_from_file(input(I), "day15.example.in"),  part2(I, 145).

:- phrase_from_file(input(I), "day15.in"), part2(I, 247153).%Out), print(Out).