cnf(l_plus_zero, axiom,
    '0' + X = X).
cnf(r_plus_zero, axiom,
    X + '0' = X).
cnf(minus_minus, axiom,
    -X + X = '0').
cnf(associativity, axiom,
    X + (Y + Z) = (X + Y) + Z).

cnf(goal, conjecture,
    -(-X) = X).
