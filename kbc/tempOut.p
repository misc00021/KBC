cnf(comm_add, axiom, '+'(A, B) = '+'(B, A)).
cnf(comm_mul, axiom, '*'(A, B) = '*'(B, A)).
cnf(assoc_add, axiom, '+'(A, '+'(B, C)) = '+'('+'(A, B), C)).
cnf(assoc_mul, axiom, '*'(A, '*'(B, C)) = '*'('*'(A, B), C)).
cnf(sub_canon, axiom, '-'(A, B) = '+'(A, '*'('-1', B))).
cnf(div_canon, axiom, (is_not_zero(B)) => '/'(A, B) = '*'(A, pow(B, '-1'))).
cnf(zero_add, axiom, '+'(A, '0') = A).
cnf(zero_mul, axiom, '*'(A, '0') = '0').
cnf(one_mul, axiom, '*'(A, '1') = A).
cnf(add_zero, axiom, A = '+'(A, '0')).
cnf(mul_one, axiom, A = '*'(A, '1')).
cnf(cancel_sub, axiom, '-'(A, A) = '0').
cnf(cancel_div, axiom, (is_not_zero(A)) => '/'(A, A) = '1').
cnf(distribute, axiom, '*'(A, '+'(B, C)) = '+'('*'(A, B), '*'(A, C))).
cnf(factor, axiom, '+'('*'(A, B), '*'(A, C)) = '*'(A, '+'(B, C))).
cnf(pow_mul, axiom, '*'(pow(A, B), pow(A, C)) = pow(A, '+'(B, C))).
cnf(pow0, axiom, (is_not_zero(X)) => pow(X, '0') = '1').
cnf(pow1, axiom, pow(X, '1') = X).
cnf(pow2, axiom, pow(X, '2') = '*'(X, X)).
cnf(pow_recip, axiom, (is_not_zero(X)) => pow(X, '-1') = '/'('1', X)).
cnf(recip_mul_div, axiom, (is_not_zero(X)) => '*'(X, '/'('1', X)) = '1').
cnf(d_variable, axiom, (is_sym(X)) => d(X, X) = '1').
cnf(d_constant, axiom, (is_sym(X) & is_const_or_distinct_var(C, X)) => d(X, C) = '0').
cnf(d_add, axiom, d(X, '+'(A, B)) = '+'(d(X, A), d(X, B))).
cnf(d_mul, axiom, d(X, '*'(A, B)) = '+'('*'(A, d(X, B)), '*'(B, d(X, A)))).
cnf(d_sin, axiom, d(X, sin(X)) = cos(X)).
cnf(d_cos, axiom, d(X, cos(X)) = '*'('-1', sin(X))).
cnf(d_ln, axiom, (is_not_zero(X)) => d(X, ln(X)) = '/'('1', X)).
cnf(d_power, axiom, (is_not_zero(F) & is_not_zero(G)) => d(X, pow(F, G)) = '*'(pow(F, G), '+'('*'(d(X, F), '/'(G, F)), '*'(d(X, G), ln(F))))).
cnf(i_one, axiom, i('1', X) = X).
cnf(i_power_const, axiom, (is_const(C)) => i(pow(X, C), X) = '/'(pow(X, '+'(C, '1')), '+'(C, '1'))).
cnf(i_cos, axiom, i(cos(X), X) = sin(X)).
cnf(i_sin, axiom, i(sin(X), X) = '*'('-1', cos(X))).
cnf(i_sum, axiom, i('+'(F, G), X) = '+'(i(F, X), i(G, X))).
cnf(i_dif, axiom, i('-'(F, G), X) = '-'(i(F, X), i(G, X))).
cnf(i_parts, axiom, i('*'(A, B), X) = '-'('*'(A, i(B, X)), i('*'(d(X, A), i(B, X)), X))).
cnf(goal, conjecture, true = false).
