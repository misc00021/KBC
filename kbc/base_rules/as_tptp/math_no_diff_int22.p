cnf(comm_add, axiom, '+'(A, B) = '+'(B, A)).
cnf(comm_mul, axiom, '*'(A, B) = '*'(B, A)).
cnf(assoc_add, axiom, '+'(A, '+'(B, C)) = '+'('+'(A, B), C)).
cnf(assoc_mul, axiom, '*'(A, '*'(B, C)) = '*'('*'(A, B), C)).
cnf(sub_canon, axiom, '-'(A, B) = '+'(A, '*'(neg('1'), B))).
cnf(div_canon, axiom, (is_not_zero(B)) => '/'(A, B) = '*'(A, pow(B, neg('1')))).
cnf(zero_add, axiom, '+'(A, '0') = A).
cnf(zero_mul, axiom, '*'(A, '0') = '0').
cnf(one_mul, axiom, '*'(A, '1') = A).
cnf(add_zero, axiom, A = '+'(A, '0')).
cnf(mul_one, axiom, A = '*'(A, '1')).
cnf(cancel_sub, axiom, '-'(A, A) = '0').
cnf(cancel_div, axiom, (is_not_zero(A)) => '/'(A, A) = '1').
cnf(distribute, axiom, '*'(A, '+'(B, C)) = '+'('*'(A, B), '*'(A, C))).
cnf(factor, axiom, '+'('*'(A, B), '*'(A, C)) = '*'(A, '+'(B, C))).
cnf(pow_mul, axiom, (is_not_zero(A)) => '*'(pow(A, B), pow(A, C)) = pow(A, '+'(B, C))).
cnf(pow0, axiom, (is_not_zero(X)) => pow(X, '0') = '1').
cnf(pow1, axiom, pow(X, '1') = X).
cnf(pow2, axiom, pow(X, '2') = '*'(X, X)).
cnf(pow_recip, axiom, (is_not_zero(X)) => pow(X, neg('1')) = '/'('1', X)).
cnf(recip_mul_div, axiom, (is_not_zero(X)) => '*'(X, '/'('1', X)) = '1').
cnf(goal, conjecture, true = false).
