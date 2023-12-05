:- load_files(['../aoc_lib.pl']).

almanac(almanac(Seeds, M)) --> "seeds: ", numbers(Seeds), eol, maps(M).
numbers([H|T]) --> number(H), !, (" ", numbers(T), ! | {T = []}).
maps([map(From, To, M)|T]) --> eol, word(From), "-to-", word(To), " map:", eol, mappings(M), (maps(T) | {T=[]}).
mappings([entry(Dest, Source, Length)|T]) 
    --> number(Dest), " ", number(Source), " ", number(Length), eol, (mappings(T) | {T=[]}).

seed_to_location(almanac(_, M), Seed, Location) :-
    almanac_map(M, "seed", "location", Seed, Location).

almanac_map(_, X, X, Y, Y).
almanac_map(M, From, To, Have, Want) :-
    member(map(From, Step, Conversions), M),
    convert(Conversions, Have, NewHave),
    almanac_map(M, Step, To, NewHave, Want).

convert(Conversions, In, Out) :-
    member(entry(Dest, Source, Length), Conversions),
    In >= Source, In =< Source+Length, !,
    Out is Dest + In-Source.
convert(_, In, In).

part1(A, Out) :-
    A = almanac(Seeds, _),
    maplist(seed_to_location(A), Seeds, Locations),
    min_member(Out, Locations).

seed_to_location_intervals(almanac(_, M), Seed, Location) :-
    almanac_map_intervals(M, "seed", "location", Seed, Location).

almanac_map_intervals(_, X, X, range(Start, _), Start).
almanac_map_intervals(M, From, To, Have, Want) :-
    member(map(From, Step, Conversions), M),
    convert_interval(Conversions, Have, NewHave),
    maplist(almanac_map_intervals(M, Step, To), NewHave, Wants),
    min_member(Want, Wants).

convert_interval([], X, [X]).
convert_interval([H|T], range(Start, Len), Out) :-
    H=entry(_, Source, SourceLen),
    (Source >= Start+Len | Source + SourceLen =< Start), !, convert_interval(T, range(Start, Len), Out).
%  |--------------|                      |-----------------|            |-----------|
% Start --Len-----                     Start ------Len-----           Start----Len---
%        |--------------|         |-----------------|             |------------------------|
%       Source -SourceLen       Source ----SourceLen           Source --SourceLen----------
convert_interval([entry(Dest, Source, SourceLen)|T], range(Start, Len), Out) :-
    RangeEnd is Start+Len,
    SourceEnd is Source+SourceLen,
    BeforeLen is Source-Start,
    AfterLen is RangeEnd-SourceEnd,
    (Start < Source -> IntersectStart = Source; IntersectStart = Start),
    (RangeEnd < SourceEnd -> IntersectEnd = RangeEnd; IntersectEnd = SourceEnd),
    IntersectLen is IntersectEnd-IntersectStart,
    (BeforeLen > 0 -> convert_interval(T, range(Start, BeforeLen), BeforeOut); BeforeOut = []),
    (AfterLen > 0 -> convert_interval(T, range(SourceEnd, AfterLen), AfterOut); AfterOut = []),
    % Oh hey, this intersection actually gets converted and is done with.
    ConvertedStart is Dest + (IntersectStart - Source),
    append([BeforeOut, [range(ConvertedStart, IntersectLen)],AfterOut], Out).

seeds_to_ranges([], []).
seeds_to_ranges([X,Y|T], [range(X,Y)|TR]) :- seeds_to_ranges(T,TR).

part2(A, Out) :-
    A = almanac(Seeds, _),
    seeds_to_ranges(Seeds, SeedRanges),
    maplist(seed_to_location_intervals(A), SeedRanges, Locations),
    min_member(Out, Locations).

:- phrase_from_file(almanac(A), "day5.example.in"), part1(A, 35).

:- phrase_from_file(almanac(A), "day5.in"), part1(A, 382895070).%Out), print(Out).

:- phrase_from_file(almanac(A), "day5.example.in"), part2(A, 46).

:- phrase_from_file(almanac(A), "day5.in"), part2(A, 17729182).%Out), print(Out).