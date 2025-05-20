cnf(double_negation, axiom,
    not(not(P)) = P).

cnf(de_morgan_or, axiom,
    not(or(P, Q)) = and(not(P), not(Q))).

cnf(de_morgan_and, axiom,
    not(and(P, Q)) = or(not(P), not(Q))).

cnf(distribute_left, axiom,
    and(P, or(Q, R)) = or(and(P, Q), and(P, R))).

cnf(distribute_right, axiom,
    and(or(P, Q), R) = or(and(P, R), and(Q, R))).

cnf(goal_false_equals_true, negated_conjecture,
    false = true).