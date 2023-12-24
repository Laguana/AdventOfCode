:- load_files(['../aoc_lib.pl']).

input([H|T]) --> hailstone(H), (eol, !, {T=[]} | input(T)).
hailstone(hailstone(X,Y,Z,DX,DY,DZ)) --> 
    number(X), ", ", number(Y), ", ", number(Z),
    " @ ", spaces, integer(DX), ", ", spaces, integer(DY), ", ", spaces, integer(DZ), eol.

intersect_xy(hailstone(X1,Y1,_,DX1,DY1,_), hailstone(X2,Y2,_,DX2,DY2,_), p(IX,IY)) :-
    % X1,Y1 + t1(DX1,DY1) = X2,Y2 + t2(DX2,DY2), t1 t2 both > 0
    % X1 + t1 DX1 = X2 + T2 DX2
    %      t1 DX1 = X2 + T2 DX2 - X1
    %      t1 DY1 = (X2 + T2 DX2 - X1)* DY1/DX1
    % Y1 + t1 DY1 = Y2 + T2 DY2
    % Y1 = (Y2 + T2 DY2) - (X2 + T2 DX2 - X1)* DY1/DX1
    % Y1 = Y2 + T2 DY2 - X2 M1 - T2 DX2 M1 + X1 M1
    % Y1 = Y2 - X2 M1 + X1 M1 + T2 (DY2 - DX2 M1) 
    % Y1 - Y2 + X2 M1 - X1 M1 = T2 (DY2 - DX2 M1)
    % T2 = (Y1 - Y2 + X2 M1 - X1 M1)/ (DY2 - DX2 M1)

    % T1 = (X2 + T2 DX2 - X1) / DX1
    M1 is DY1 / DX1,
    M2 is DY2 / DX2,
    M1 \= M2,
    T2 is (Y1 - Y2 + M1 * (X2 - X1)) / (DY2 - (DX2 * M1)),
    T1 is (X2 - X1 + (T2 * DX2) ) / DX1,
    !,
    T1 > 0,
    T2 > 0,
    IX is X1 + T1 * DX1,
    IY is Y1 + T1 * DY1.

intersect_in_bounds([], _, Out, Out) :- !.
intersect_in_bounds([H|T], Bounds, Acc, Out) :-
    intersect_in_bounds(H, T, Bounds, 0, HIntersect),
    TAcc is Acc + HIntersect,
    intersect_in_bounds(T, Bounds, TAcc, Out).
intersect_in_bounds(_, [], _, Out, Out) :- !.
intersect_in_bounds(A, [H|T], b(XL,XG,YL,YG), Acc, Out) :-
    (intersect_xy(A,H,p(X,Y)), X >= XL, X =< XG, Y >= YL, Y =< YG
    -> TAcc is Acc + 1
    ; TAcc = Acc),!,
    intersect_in_bounds(A, T, b(XL,XG,YL,YG), TAcc, Out).

part1(I, Bounds, O) :-
    intersect_in_bounds(I, Bounds, 0, O).

%ok, part2, uhh.... uhh
% RX,RY,RZ + k(DRX, DRY, DRZ) = every hailstone for some k
% i.e.
% RX,RY,RZ + ki(DRX,DRY, DRZ) = Xi,Yi,Zi + ki(DXi,DYi,DZi) for i from 0 to |hailstones|, ki > 0
% RX,RY,RZ is apparently unique (and thus everything)
% RX + Ki DRX = Xi + Ki DXi for each i, which gives us 2+N unknowns in N equations
% RY + Ki DRY = Yi + Ki DYi for each i, which is now another N equations with 2+N unknowns but only 2 new unknowns
% RZ + Ki DRZ = Zi + Ki DZi for each i, another N equations with 2 new unknowns
% I.E. 3N equations with N+6 variables
% this is some linear algebra that I have to go remember now >.>
% Although, not strictly linear since it has Ki DRX which are both unknown
%
% 3N equations in N+6 variables should mean that N=3 is fully determined...?
% R(XYZ) + K1(DR(XYZ)) = (XYZ)1 + K1(D(XYZ)1)
% R(XYZ) + K2(DR(XYZ)) = (XYZ)2 + K2(D(XYZ)1)
% R(XYZ) + K3(DR(XYZ)) = (XYZ)3 + K3(D(XYZ)1)

% RX + K1 DRX = X1 + K1 DX1
% (RX-X1) = K1(DX1-DRX)

% K1 = (RX-X1)/(DX1-DRX)
% K2 = (RX-X2)/(DX2-DRX)
% K3 = (RX-X3)/(DX3-DRX)

% RY + K1 DRY = Y1 + K1 DY1
% RY = Y1 + K1 (DY1 - DRY)
% RY = Y1 + (RX-X1)(DY1-DRY)/(DX1-DRX)

% RZ + K1 DRZ = Z1 + K1 DZ1
% RZ = Z1 + (RX-X1)(DZ1-DRZ)/(DX1-DRX)

% RY + K2 DRY = Y2 + K2 DY2
% K2 DRY = (Y2 - RY) + K2 DY2
% DRY = (Y2 - RY)/K2 + DY2
% DRY = (Y2 - Y1 + (RX-X1)(DY1-DRY)/(DX1-DRX)) (DX2-DRX)/(RX-X2) + DY2
% ...

% ok fine z3 it is.

/*
[x,y,z,dx,dy,dz,k1,k2,k3] = [z3.Real('x'), z3.Real('y'), z3.Real('z'), z3.Real('dx'), z3.Real('dy'), z3.Real('dz'), z3.Real('k1'), z3.Real('k2'), z3.Real('k3')]
s = z3.Solver()
s.add(x + k1 * dx == 309254625334097 - 42 * k1)
s.add(y + k1 * dy == 251732589486275 - 22 * k1)
s.add(z + k1 * dz == 442061964691135 - 45 * k1)
s.add(x + k2 * dx == 494902262649699 - 345 * k2)
s.add(y + k2 * dy == 448845738683125 - 319 * k2)
s.add(z + k2 * dz == 408766676225787 - 201 * k2)
s.add(x + k3 * dx == 281199817421623 + 89 * k3)
s.add(y + k3 * dy == 235413393248399 + 152 * k3)
s.add(z + k3 * dz == 236652333766125 - 70 * k3)
s.check()
s.model()
[k1 = 950225482094,
 dz = 296,
 dx = -65,
 k3 = 324090870134,
 dy = -86,
 k2 = 584973040098,
 z = 118035075297081,
 x = 331109811422259,
 y = 312547020340291]
*/






:- phrase_from_file(input(I), "day24.example.in"), part1(I, b(7,27,7,27), 2).

:- phrase_from_file(input(I), "day24.in"), part1(I, b(200000000000000, 400000000000000, 200000000000000, 400000000000000), 31921).%Out), print(Out).

%:- phrase_from_file(input(I), "day24.example.in"),  part2(I, ).

%:- phrase_from_file(input(I), "day24.in"), part2(I, 761691907059631).%Out), print(Out).