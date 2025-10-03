%https://github.com/Boolector/boolector/blob/master/src/btorrewrite.c

cnf(shL, axiom, (a * '2') = (shL(a))).

cnf(shL3, axiom, (a * '3') = (shL(a) + a)).

cnf(shR, axiom, (a / '2') = shR(a)).

cnf(mul_assoc, axiom,
    N * (M * K) = (N * M) * K).

cnf(zero, definition, b = '0' <=> a + b = a).

cnf(mul_one, axiom,
    N * '1' = N).

cnf(mul_zero, axiom,
    N * '0' = '0').

cnf(goal, conjecture, true = false).