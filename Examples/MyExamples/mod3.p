% Modulo 3 Arithmetic: Complete Ground Theory for ℤ/3ℤ

% --- Addition mod 3 ---
cnf(add_00, axiom, '0' + '0' = '0').
cnf(add_01, axiom, '0' + '1' = '1').
cnf(add_02, axiom, '0' + '2' = '2').
cnf(add_10, axiom, '1' + '0' = '1').
cnf(add_11, axiom, '1' + '1' = '2').
cnf(add_12, axiom, '1' + '2' = '0').
cnf(add_20, axiom, '2' + '0' = '2').
cnf(add_21, axiom, '2' + '1' = '0').
cnf(add_22, axiom, '2' + '2' = '1').

% --- Multiplication mod 3 ---
cnf(mul_00, axiom, '0' * '0' = '0').
cnf(mul_01, axiom, '0' * '1' = '0').
cnf(mul_02, axiom, '0' * '2' = '0').
cnf(mul_10, axiom, '1' * '0' = '0').
cnf(mul_11, axiom, '1' * '1' = '1').
cnf(mul_12, axiom, '1' * '2' = '2').
cnf(mul_20, axiom, '2' * '0' = '0').
cnf(mul_21, axiom, '2' * '1' = '2').
cnf(mul_22, axiom, '2' * '2' = '1').

% --- Optional: Identity Elements ---
cnf(add_identity_0, axiom, '0' + X = X).
cnf(mul_identity_1, axiom, '1' * X = X).

% --- Optional: Commutativity ---
cnf(add_comm, axiom, X + Y = Y + X).
cnf(mul_comm, axiom, X * Y = Y * X).

% --- Optional: Associativity ---
cnf(add_assoc, axiom, (X + Y) + Z = X + (Y + Z)).
cnf(mul_assoc, axiom, (X * Y) * Z = X * (Y * Z)).

% --- Optional: Distributivity ---
cnf(dist_left, axiom, X * (Y + Z) = (X*Y) + (X*Z)).
cnf(dist_right, axiom, (X + Y) * Z = (X*Z) + (Y*Z)).

cnf(goal, conjecture, (X+(X+X))+((Y+Y)+(Y+Y))=Y).