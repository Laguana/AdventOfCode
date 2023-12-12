:- load_files(['../aoc_lib.pl']).

all_blank(L) :-
    [Dot] = ".",
    maplist(=(Dot), L).
all_blank(indexed(L, _)) :-
    [Dot] = ".",
    maplist(=(Dot), L).

index(E, I, indexed(E,I)).
index(indexed(_,I), I).

blank_rows(Lines,BlankRows) :-
    length(Lines,MaxY),
    numlist(1,MaxY, Numbers),
    maplist(index, Lines, Numbers, Indexed),
    include(all_blank, Indexed, Filtered),
    maplist(index, Filtered, BlankRows).

blank_cols(Lines, BlankCols) :-
    [H|_] = Lines,
    length(H, MaxX),
    numlist(1,MaxX, Numbers),
    include(blank_col(Lines), Numbers, BlankCols).
blank_col(Lines, X) :-
    maplist(nth1(X), Lines, Col),
    all_blank(Col).

coordinate_key(X,Y,K) :- atom_concat(X, ':', Mid), atom_concat(Mid, Y, K).

sub(Len, E, V) :- V is Len-E.

galaxy_coordinates(Lines, BlankRows, BlankCols, Coordinates, P12) :-
    length(Lines, YMax),
    maplist(sub(YMax), BlankRows, BR),
    Lines = [H|_],
    length(H, XMax),
    maplist(sub(XMax), BlankCols, BC),
    galaxy_coordinates(Lines, BR, BC, 1, [], Coordinates, P12).
galaxy_coordinates([], _, _, _, Coordinates, Coordinates, _).
galaxy_coordinates([_|T], [H|BR], BlankCols, Y, CIn, COut, P12) :-
    length(T, H), !,
    (P12 = p1 -> Skip = 2;Skip = 1000000),
    YNext is Y+Skip,
    galaxy_coordinates(T, BR, BlankCols, YNext, CIn, COut, P12).
galaxy_coordinates([H|T], BR, BlankCols, Y, CIn, COut, P12) :-
    galaxy_coordinates_row(H, BlankCols, Y, 1, CIn, CNext, P12),
    Y1 is Y + 1,
    galaxy_coordinates(T, BR, BlankCols, Y1, CNext, COut, P12).

galaxy_coordinates_row([], [], _, _, C, C, _).
galaxy_coordinates_row([_|T], [H|BC], Y, X, CIn, COut, P12) :-
    length(T, H),!,
    (P12 = p1 -> Skip = 2;Skip = 1000000),
    XNext is X+Skip,
    galaxy_coordinates_row(T, BC, Y, XNext, CIn, COut, P12).
galaxy_coordinates_row([H|T],BC, Y, X, CIn, COut, P12) :-
    [Dot] = ".",
    XNext is X+1,
    (H = Dot 
        -> galaxy_coordinates_row(T, BC, Y, XNext, CIn, COut, P12)
        ; galaxy_coordinates_row(T, BC, Y, XNext, [p(X,Y)|CIn], COut, P12)
    ).

distances(L, D) :-
    distances(L, [], D).
distances([], D, D).
distances([H|T], Acc, D) :-
    pair_distances(H, T, New),
    append(New, Acc, NAcc),
    distances(T, NAcc, D).

pair_distances(P, L, D) :- pair_distance(P, L, [], D).
pair_distance(_, [], D, D).
pair_distance(p(X,Y), [p(X1, Y1)|T], Acc, D) :-
    Dist is abs(X-X1) + abs(Y-Y1),
    pair_distance(p(X,Y), T, [Dist|Acc], D).


part1(In, Out) :-
    blank_rows(In, BlankRows),
    blank_cols(In, BlankCols),
    !,
    galaxy_coordinates(In, BlankRows, BlankCols, Coordinates, p1),
    sort(Coordinates, Sorted),
    !,
    distances(Sorted, Distances),
    sum_list(Distances, Out).

part2(In, Out) :-
    blank_rows(In, BlankRows),
    blank_cols(In, BlankCols),
    !,
    galaxy_coordinates(In, BlankRows, BlankCols, Coordinates, p2),
    sort(Coordinates, Sorted),
    !,
    distances(Sorted, Distances),
    sum_list(Distances, Out).
    

:- phrase_from_file(lines(I), "day11.example.in"), part1(I, 374).

:- phrase_from_file(lines(I), "day11.in"), part1(I, 9312968).%Out), print(Out).

%:- phrase_from_file(lines(I), "day11.example.in"),  part2(I, 1030).

:- phrase_from_file(lines(I), "day11.in"), part2(I, 597714117556).%Out), print(Out).