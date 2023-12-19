:- load_files(['../aoc_lib.pl']).

input(input(W,P)) --> workflows(workflows{}, W), eol, parts(P).
workflows(In, Out) --> word_atom(Name), "{", rules(R), "}", eol, (workflows(In.put(Name, R), Out), ! | {Out = In.put(Name, R)} ).
rules([rule(Property, Condition, Threshold, Destination)|T]) --> property(Property),condition(Condition),number(Threshold), ":", word_atom(Destination), ",", rules(T).
rules([rule(Destination)]) --> word_atom(Destination).
property(x) --> "x".
property(m) --> "m".
property(a) --> "a".
property(s) --> "s".
condition(less) --> "<".
condition(greater) --> ">".
parts([part{x:X, m:M, a:A, s:S}|T]) --> "{x=",number(X), ",m=",number(M), ",a=", number(A), ",s=", number(S), "}", eol, (parts(T), ! | {T = []}).

process_part(Workflows, Part, Result) :-
    process_part(Workflows, Part, in, Result).
process_part(_, _, 'A', accept) :- !.
process_part(_, _, 'R', reject) :- !.
process_part(Workflows, Part, State, Result) :-
    Flow = Workflows.get(State),
    first_match(Flow, Part, NewState),!,
    process_part(Workflows, Part, NewState, Result).

first_match([rule(Destination)], _, Destination).
first_match([rule(Prop, Cond, Thresh, Dest)|T], Part, NewState) :-
    V = Part.get(Prop),
    (Cond = less -> V < Thresh ; V > Thresh) -> NewState = Dest
    ; first_match(T, Part, NewState).

workflows_accept(Workflows, Part) :-
    process_part(Workflows, Part, accept).

score(Part, Score) :-
    Score is Part.x + Part.m + Part.a + Part.s .

part1(input(Workflow, Parts), Out) :-
    include(workflows_accept(Workflow), Parts, Accepted),
    maplist(score, Accepted, Scores),
    sumlist(Scores, Out).

get_constraints(Workflow, Accepting) :-
    get_constraints(Workflow, c{x:between(1,4000), m:between(1,4000), a:between(1,4000), s:between(1,4000)}, in, [], Accepting).
get_constraints(_, C, 'A', Acc, [C|Acc]) :- !.
get_constraints(_, _, 'R', Acc, Acc) :- !.
get_constraints(Workflow, Constraint, State, Acc, Accepting) :-
    Flow = Workflow.get(State),
    get_constraints_(Workflow, Constraint, Flow, Acc, Accepting).
get_constraints_(Workflow, Constraint, [rule(Dest)], Acc, Accepting) :-
    !, get_constraints(Workflow, Constraint, Dest, Acc, Accepting).
get_constraints_(Workflow, Constraint, [rule(Prop, Cond, Thresh,_)|T], Acc, Accepting) :-
    between(Low,High) = Constraint.get(Prop),
    (Cond = less -> Thresh =< Low; Thresh >= High),
    % This is strictly outside, skip this.
    !,
    get_constraints_(Workflow, Constraint, T, Acc, Accepting).
get_constraints_(Workflow, Constraint, [rule(Prop, Cond, Thresh,Dest)|_], Acc, Accepting) :-
    between(Low,High) = Constraint.get(Prop),
    (Cond=less -> Thresh > High; Thresh < Low),
    % This is strictly included, always take it
    !,
    get_constraints(Workflow, Constraint, Dest, Acc, Accepting).
get_constraints_(Workflow, Constraint, [rule(Prop, Cond, Thresh,Dest)|T], Acc, Accepting) :-
    between(Low,High) = Constraint.get(Prop),
    %Low1 is Low+1,
    %High1 is High-1,
    %between(Low1, High1, Thresh),
    % This is strictly between; there is a legit divergence
    !,
    (Cond = less -> 
        Mid is Thresh-1,
        PConstraint = Constraint.put(Prop, between(Low, Mid)),
        NConstraint = Constraint.put(Prop, between(Thresh, High))
    ; 
        Mid is Thresh+1,
        PConstraint = Constraint.put(Prop, between(Mid, High)),
        NConstraint = Constraint.put(Prop, between(Low, Thresh))
    ),

    get_constraints(Workflow, PConstraint, Dest, Acc, MAcc),!,
    get_constraints_(Workflow, NConstraint, T, MAcc, Accepting).

count_constraints(L, Out) :- count_constraints(L, 0, Out).
count_constraints([], Acc, Acc).
count_constraints([c{x:between(XL,XH), m:between(ML,MH), a:between(AL,AH), s:between(SL,SH)}|T], Acc, Out) :-
    Options is (XH-XL+1) * (MH-ML+1) * (AH-AL+1) * (SH-SL+1),
    NAcc is Acc + Options,
    !,
    count_constraints(T, NAcc, Out).

part2(input(Workflows,_), Out) :-
    get_constraints(Workflows, Accepting),
    count_constraints(Accepting, Out).

:- phrase_from_file(input(I), "day19.example.in"), part1(I, 19114).

:- phrase_from_file(input(I), "day19.in"), part1(I, 319295).%Out), print(Out).

:- phrase_from_file(input(I), "day19.example.in"),  part2(I, 167409079868000).

:- phrase_from_file(input(I), "day19.in"), part2(I, 110807725108076).%Out), print(Out).