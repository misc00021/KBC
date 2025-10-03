cnf(plus_assoc, axiom,
    A + (B + C) = (A + B) + C).

%cnf(plus_comm, axiom,
%    A + B = B + A).

cnf(plus_identity, axiom,
    A + '0' = A).

cnf(plus_idempotent, axiom,
    A + A = A).

cnf(times_assoc, axiom,
    A * (B * C) = (A * B) * C).

cnf(times_left_identity, axiom,
    '1' * A = A).

cnf(times_right_identity, axiom,
    A * '1' = A).

cnf(times_self_star, axiom,
    A * star(A) = star(A)).

cnf(times_self_star, axiom,
    star(A) * A = star(A)).

cnf(times_left_distrib, axiom,
    A * (B + C) = (A * B) + (A * C)).

cnf(times_right_distrib, axiom,
    (A + B) * C = (A * C) + (B * C)).

cnf(star_strong_left_unroll, axiom,
    '1' + (A * star(A)) = star(A)).

cnf(star_strong_right_unroll, axiom,
    '1' + (star(A) * A) = star(A)).

cnf(star_idempotent, axiom,
    star(A) * star(A) = star(A)).

cnf(star_saturation, axiom,
    (star(star(A))) = star(A)).

cnf(star_identity, axiom,
    star('1') = '1').

cnf(star_annihilator, axiom,
    start('0') = '1').

cnf(goal, conjecture, A = B).