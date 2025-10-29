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
cnf(guarded_rule_0, axiom, (is_not_zero(X)) => '/'(X, X) = '1').
cnf(guarded_rule_1, axiom, pow(X, '1') = X).
cnf(guarded_rule_2, axiom, (is_not_zero(X)) => pow(X, '-1') = '/'('1', X)).
cnf(guarded_rule_3, axiom, '*'(X, X) = pow(X, '2')).
cnf(guarded_rule_4, axiom, '*'(X, Y) = '*'(Y, X)).
cnf(guarded_rule_5, axiom, '*'(X, '1') = X).
cnf(guarded_rule_6, axiom, (is_not_zero(X)) => '*'(X, '/'('1', X)) = '1').
cnf(guarded_rule_7, axiom, (is_not_zero(Y)) => '*'(X, '/'('1', Y)) = '/'(X, Y)).
cnf(guarded_rule_8, axiom, '*'('*'(X, Y), Z) = '*'(X, '*'(Y, Z))).
cnf(guarded_rule_9, axiom, (is_not_zero(X)) => '*'(pow(X, Y), pow(X, Z)) = pow(X, '+'(Y, Z))).
cnf(guarded_rule_10, axiom, '*'('1', X) = X).
cnf(guarded_rule_11, axiom, '/'(X, '1') = X).
cnf(guarded_rule_12, axiom, pow('1', '2') = '1').
cnf(guarded_rule_13, axiom, (is_not_zero(X)) => '*'('/'('1', X), Y) = '/'(Y, X)).
cnf(guarded_rule_14, axiom, '*'(X, '*'(Y, Z)) = '*'(Y, '*'(X, Z))).
cnf(guarded_rule_15, axiom, (is_not_zero(X)) => '*'(X, '/'(Y, X)) = Y).
cnf(guarded_rule_16, axiom, (is_not_zero(Z)) => '*'(X, '/'(Y, Z)) = '/'('*'(X, Y), Z)).
cnf(guarded_rule_17, axiom, (is_not_zero(X)) => '/'('*'(X, Y), X) = Y).
cnf(guarded_rule_18, axiom, pow(X, '+'(Y, Z)) = pow(X, '+'(Z, Y))).
cnf(guarded_rule_20, axiom, (is_not_zero(X)) => '/'(pow(X, '2'), X) = X).
cnf(guarded_rule_21, axiom, (is_not_zero(X)) => '*'(X, pow(X, Y)) = pow(X, '+'(Y, '1'))).
cnf(guarded_rule_22, axiom, pow('1', '+'('2', '1')) = '1').
cnf(guarded_rule_24, axiom, pow('1', '+'('2', '2')) = '1').
cnf(guarded_rule_25, axiom, (is_not_zero(Y)) => '/'('*'(X, Y), Y) = X).
cnf(guarded_rule_27, axiom, pow('1', '+'(X, '2')) = pow('1', X)).
cnf(guarded_rule_28, axiom, pow(X, '+'('1', '1')) = pow(X, '2')).
cnf(guarded_rule_29, axiom, pow('1', '+'(X, '1')) = pow('1', X)).
cnf(guarded_rule_30, axiom, (is_not_zero(Y)) => '*'('/'(X, Y), Z) = '/'('*'(X, Z), Y)).
cnf(guarded_rule_31, axiom, (is_not_zero(Y) & is_not_zero(Z)) => '/'('/'(X, Y), Z) = '/'('/'(X, Z), Y)).
cnf(guarded_rule_33, axiom, (is_not_zero(Y) & is_not_zero(X)) => '/'('/'(X, Y), X) = '/'('1', Y)).
cnf(guarded_rule_35, axiom, (is_not_zero(X) & is_not_zero(Y)) => '/'(X, '/'(X, Y)) = Y).
cnf(goal, conjecture, true = false).
