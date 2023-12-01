file_search_path(library,'..').
[library('aoc_lib.pl')].


:- phrase_from_file(lines(L), "day0.in"), print(L).