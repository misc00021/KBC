cnf(add_assoc, axiom,
    (X + Y) + Z = X + (Y + Z)).

cnf(add_comm, axiom,
    X + Y = Y + X).

cnf(add_id, axiom,
    X + zero = X).

cnf(add_inv, axiom,
    X + X = zero).

cnf(mul_assoc, axiom,
    (X * Y) * Z = X * (Y * Z)).

cnf(mul_comm, axiom,
    X * Y = Y * X).

cnf(mul_dist, axiom,
    X * (Y + Z) = (X*Y) + (X*Z)).

cnf(goal, conjecture, X = X).