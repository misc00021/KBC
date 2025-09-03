cnf(comm_add, axiom, '+'(X, Y) = '+'(Y, X)).
cnf(comm_mul, axiom, '*'(X, Y) = '*'(Y, X)).
cnf(assoc_add, axiom, '+'(X, '+'(Y, Z)) = '+'('+'(X, Y), Z)).
cnf(assoc_mul, axiom, '*'(X, '*'(Y, Z)) = '*'('*'(X, Y), Z)).
cnf(sub_canon, axiom, '-'(X, Y) = '+'(X, '*'(neg('1'), Y))).
cnf(div_canon, axiom, (is_not_zero(Y)) => '/'(X, Y) = '*'(X, pow(Y, neg('1')))).
cnf(zero_add, axiom, '+'(X, '0') = X).
cnf(zero_mul, axiom, '*'(X, '0') = '0').
cnf(one_mul, axiom, '*'(X, '1') = X).
cnf(add_zero, axiom, X = '+'(X, '0')).
cnf(mul_one, axiom, X = '*'(X, '1')).
cnf(cancel_sub, axiom, '-'(X, X) = '0').
cnf(cancel_div, axiom, (is_not_zero(X)) => '/'(X, X) = '1').
cnf(distribute, axiom, '*'(X, '+'(Y, Z)) = '+'('*'(X, Y), '*'(X, Z))).
cnf(factor, axiom, '+'('*'(X, Y), '*'(X, Z)) = '*'(X, '+'(Y, Z))).
cnf(pow_mul, axiom, (is_not_zero(X)) => '*'(pow(X, Y), pow(X, Z)) = pow(X, '+'(Y, Z))).
cnf(pow0, axiom, (is_not_zero(X)) => pow(X, '0') = '1').
cnf(pow1, axiom, pow(X, '1') = X).
cnf(pow2, axiom, pow(X, '2') = '*'(X, X)).
cnf(pow_recip, axiom, (is_not_zero(X)) => pow(X, neg('1')) = '/'('1', X)).
cnf(recip_mul_div, axiom, (is_not_zero(X)) => '*'(X, '/'('1', X)) = '1').
cnf(goal, conjecture, true = false).
