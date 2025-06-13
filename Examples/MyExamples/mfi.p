cnf(l_plus_zero, axiom,
    '0' + X = X).
<<<<<<< HEAD
cnf(r_plus_zero, axiom,
=======
cnf(zero_plus, axiom,
>>>>>>> 2ea8716 (conditional)
    X + '0' = X).
cnf(minus_minus, axiom,
    -X + X = '0').
cnf(associativity, axiom,
    X + (Y + Z) = (X + Y) + Z).

cnf(goal, conjecture,
    -(-X) = X).
