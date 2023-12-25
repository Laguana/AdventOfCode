:- load_files(['../aoc_lib.pl']).

input(I) --> edges(g{}, I).
edges(In, Out) --> word_atom(From), ": ", dests(To), eol, (edges(In.put(From, To), Out), ! | {Out = In.put(From,To)}).
dests([H|T]) --> word_atom(H), (" ", !, dests(T) | {T=[]}).

bidirectionalize(In, Out) :-
    dict_pairs(In, _, Pairs),
    bidirectionalize(In, Pairs, Out).
bidirectionalize(In, [], In) :- !.
bidirectionalize(In, [From-To|T], Out) :-
    addEdges(In, To,From,Mid),
    bidirectionalize(Mid, T, Out).

addEdges(G, [],_,G) :- !.
addEdges(G, [H|T], To, Out) :-
    (D = G.get(H),!;D=[]),
    addEdges(G.put(H, [To|D]), T, To, Out).

% https://stackoverflow.com/questions/28917290/how-can-i-find-bridges-in-an-undirected-graph/28917697#28917697
% Actually nevermind, none of that.
tarjan(G, O) :-
    dict_keys(G, K),
    tarjan(pre{},low{}, G, K, [], O).
tarjan(_,_,_, [], O, O) :- !.
tarjan(Pre, Low, G, [V|T], Acc, O) :-
    _ = Pre.get(V) -> tarjan(Pre, Low, G, T, Acc, O)
    ;
    tarjan(Pre, Low, G, V, V, MOutPre, MOutLow, Acc, MO),
    tarjan(MOutPre,MOutLow, G, T, MO, O).
tarjan(Pre, Low, G, U, V, OutPre, OutLow, Acc, O) :-
    dict_size(Pre, Count),
    NPre = Pre.put(V, Count),
    NLow = Low.put(V, Count),
    print(V-Count),nl,
    tarjan(NPre, NLow, G, U, V, G.get(V), OutPre, OutLow, Acc, O).
tarjan(Pre,Low,_,_,_,[],Pre,Low, Acc, Acc) :- !.
tarjan(Pre, Low, G, U, V, [W|T], OutPre, OutLow, Acc, O) :-
    WPre = Pre.get(W) ->
        (W \= U -> print(-V-W-Low.get(V)-WPre),nl,MinLow is min(Low.get(V), WPre), NLow = Low.put(V, MinLow);NLow = Low),
        tarjan(Pre, NLow, G, U, V, T, OutPre, OutLow, Acc, O)
    ;
    tarjan(Pre, Low, G, V, W, ROutPre, ROutLow, Acc, RO),
    !,
    VLow is min(ROutLow.get(V), ROutLow.get(W)),
    NNLow = ROutLow.put(V,VLow),
    print(V-W), print(ROutLow.get(W)-ROutPre.get(W)), nl,
    (ROutLow.get(W) = ROutPre.get(W) -> NAcc = [V-W|RO];NAcc = RO),
    tarjan(ROutPre, NNLow, G, U, V, T, OutPre, OutLow, NAcc, O).

% Visual inspection / graphviz:
% mhb--zqg
% sjr--jlt
% fjn--mzb

remove_edge(In,From-To,Out) :-
    FIn = In.get(From),
    TIn = In.get(To),
    select(To, FIn, FOut),
    select(From, TIn, TOut),
    Out = In.put(From, FOut).put(To,TOut).
remove_edges(In, [H|T],Out) :-
    remove_edge(In, H, Mid),
    remove_edges(Mid, T, Out).
remove_edges(In, [], In).

region_size(G, N, O-R) :-
    region_size(G, G.get(N), [N], R),
    length(R, O).
region_size(_, [], Acc, Acc).
region_size(G, [H|T], Acc, R) :-
    member(H,Acc),!,
    region_size(G,T,Acc,R).
region_size(G, [H|T], Acc, R) :-
    NAcc = [H|Acc],
    append(G.get(H), T, NT),
    region_size(G, NT, NAcc, R).

part1(In, Out) :-
    bidirectionalize(In, BIn),
    remove_edges(BIn, [mhb-zqg,sjr-jlt,fjn-mzb], Partitioned),
    dict_keys(BIn, K),
    length(K,TotalSize),
    region_size(Partitioned, mhb, OneSize-Set1),
    region_size(Partitioned, zqg, OtherSize-Set2),
    sort(Set1, SS1), sort(Set2,SS2),
    length(SS1,LSS1), length(SS2, LSS2),
    print(OneSize-TotalSize-OtherSize-LSS1-LSS2),nl,
    Out is OneSize * OtherSize.


%:- phrase_from_file(input(I), "day25.example.in"), part1(I, ).

:- phrase_from_file(input(I), "day25.in"), part1(I, 606062).%Out), print(Out).

%:- phrase_from_file(input(I), "day25.example.in"),  part2(I, ).

%:- phrase_from_file(input(I), "day25.in"), part2(I, Out), print(Out).