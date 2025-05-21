% Commutativity and associativity
cnf(add_comm, axiom, plus(A, B) = plus(B, A)).
cnf(mul_comm, axiom, times(A, B) = times(B, A)).
cnf(add_assoc, axiom, plus(A, plus(B, C)) = plus(plus(A, B), C)).
cnf(mul_assoc, axiom, times(A, times(B, C)) = times(times(A, B), C)).

% Canonical and simplification forms (no division)
cnf(sub_to_add, axiom, minus(A, B) = plus(A, times(minus_one, B))).
cnf(add_zero_right, axiom, plus(A, zero) = A).
cnf(mul_zero_right, axiom, times(A, zero) = zero).
cnf(mul_one_right, axiom, times(A, one) = A).
cnf(add_zero_left, axiom, A = plus(A, zero)).
cnf(mul_one_left, axiom, A = times(A, one)).
cnf(sub_cancel, axiom, minus(A, A) = zero).

% Distributivity and factoring
cnf(distribute, axiom, times(A, plus(B, C)) = plus(times(A, B), times(A, C))).
cnf(factor, axiom, plus(times(A, B), times(A, C)) = times(A, plus(B, C))).

% Powers (only safe ones)
cnf(pow_mul_merge, axiom, times(pow(A, B), pow(A, C)) = pow(A, plus(B, C))).
cnf(pow_one, axiom, pow(X, one) = X).
cnf(pow_two, axiom, pow(X, two) = times(X, X)).

cnf(goal, conjecture, 'true' = 'false').