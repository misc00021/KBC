cnf(plus_zero, axiom,
    '0' + X = X).
cnf(minus_minus, axiom,
    -X + X = '0').
cnf(associativity, axiom,
    X + (Y + Z) = (X + Y) + Z).

cnf(goal, conjecture,
    true = false).
