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
cnf(rule_0, axiom, '*'(X, X) = pow(X, '2')).
cnf(rule_1, axiom, '*'(X, Y) = '*'(Y, X)).
cnf(rule_2, axiom, '*'(X, '1') = X).
cnf(rule_3, axiom, '*'('1', X) = X).
cnf(rule_4, axiom, (is_not_zero(X)) => '/'(X, X) = '1').
cnf(rule_5, axiom, '/'(X, '1') = X).
cnf(rule_6, axiom, pow(X, '1') = X).
cnf(rule_7, axiom, pow('1', '2') = '1').
cnf(rule_8, axiom, (is_not_zero(X)) => pow(X, '-1') = '/'('1', X)).
cnf(rule_9, axiom, '*'(X, '*'(X, Y)) = '*'(pow(X, '2'), Y)).
cnf(rule_10, axiom, (is_not_zero(Y) & is_not_zero(Z)) => '*'(X, '/'(Y, Z)) = '/'(X, '/'(Z, Y))).
cnf(rule_11, axiom, (is_not_zero(X) & is_not_zero(Y)) => '*'(X, pow(X, Y)) = pow(X, '+'(Y, '1'))).
cnf(rule_12, axiom, '*'(Y, '*'(Y, X)) = '*'(X, pow(Y, '2'))).
cnf(rule_13, axiom, '*'(X, '*'(Y, Z)) = '*'(Y, '*'(X, Z))).
cnf(rule_14, axiom, '*'('*'(X, Y), Z) = '*'(X, '*'(Y, Z))).
cnf(rule_15, axiom, (is_not_zero(Y) & is_not_zero(Z)) => '*'('/'(X, Y), Z) = '/'(X, '/'(Y, Z))).
cnf(rule_16, axiom, (is_not_zero(Y)) => '/'(X, '*'(Y, X)) = '/'('1', Y)).
cnf(rule_17, axiom, (is_not_zero(X) & is_not_zero(Y)) => '/'(X, '/'(Y, X)) = '/'(pow(X, '2'), Y)).
cnf(rule_18, axiom, (is_not_zero(X)) => '/'(X, '/'('1', X)) = pow(X, '2')).
cnf(rule_19, axiom, (is_not_zero(Y)) => '/'(X, '/'('1', Y)) = '*'(X, Y)).
cnf(rule_20, axiom, (is_not_zero(X)) => '/'(X, pow(X, '2')) = '/'('1', X)).
cnf(rule_21, axiom, (is_not_zero(Z) & is_not_zero(X)) => '/'(X, '/'(Y, Z)) = '/'(Z, '/'(Y, X))).
cnf(rule_22, axiom, (is_not_zero(Z) & is_not_zero(Y)) => '/'('*'(X, Y), Z) = '/'(X, '/'(Z, Y))).
cnf(rule_23, axiom, (is_not_zero(Y)) => '/'('/'(X, Y), Y) = '/'(X, pow(Y, '2'))).
cnf(rule_24, axiom, (is_not_zero(Y) & is_not_zero(Z)) => '/'('/'(X, Y), Z) = '/'(X, '*'(Y, Z))).
cnf(rule_25, axiom, (is_not_zero(X)) => '/'(pow(X, '2'), X) = X).
cnf(rule_26, axiom, (is_not_zero(Y) & is_not_zero(X)) => pow(X, '+'(Y, Y)) = pow(pow(X, Y), '2')).
cnf(rule_27, axiom, pow(X, '+'('1', '1')) = pow(X, '2')).
cnf(rule_28, axiom, pow(X, '+'(Y, Z)) = pow(X, '+'(Z, Y))).
cnf(rule_29, axiom, pow('1', '+'(X, '1')) = pow('1', X)).
cnf(rule_30, axiom, pow('1', '+'(X, '2')) = pow('1', X)).
cnf(rule_31, axiom, pow('1', '+'('1', X)) = pow('1', X)).
cnf(rule_32, axiom, pow('1', '+'('2', X)) = pow('1', X)).
cnf(rule_33, axiom, (is_not_zero(X)) => pow('/'('1', X), '2') = '/'('1', pow(X, '2'))).
cnf(rule_34, axiom, (is_not_zero(Y) & is_not_zero(X)) => '/'(pow(X, Y), X) = pow(X, '+'(Y, '-1'))).
cnf(rule_35, axiom, pow(X, '+'('2', '-1')) = X).
cnf(rule_36, axiom, (is_not_zero(Y) & is_not_zero(X)) => '/'(pow(X, Y), X) = pow(X, '+'('-1', Y))).
cnf(rule_37, axiom, pow(X, '+'('-1', '-1')) = '/'('1', pow(X, '2'))).
cnf(rule_38, axiom, '*'(X, '*'(X, pow(Y, '2'))) = pow('*'(X, Y), '2')).
cnf(rule_39, axiom, (is_not_zero(X) & is_not_zero(Z)) => '*'(X, '*'(Y, pow(X, Z))) = '*'(Y, pow(X, '+'(Z, '1')))).
cnf(rule_40, axiom, (is_not_zero(X) & is_not_zero(Z)) => '*'(X, '*'(Y, pow(X, Z))) = '*'(pow(X, '+'(Z, '1')), Y)).
cnf(rule_41, axiom, '*'(X, '*'(pow(Y, '2'), X)) = pow('*'(X, Y), '2')).
cnf(rule_42, axiom, '*'(Y, '*'(Y, '*'(X, Z))) = '*'(X, '*'(pow(Y, '2'), Z))).
cnf(rule_43, axiom, (is_not_zero(X)) => '*'(X, pow('/'(Y, X), '2')) = '/'(pow(Y, '2'), X)).
cnf(rule_44, axiom, (is_not_zero(Y) & is_not_zero(Z)) => '*'(pow(Y, '+'(Z, Z)), X) = '*'(X, pow(pow(Y, Z), '2'))).
cnf(rule_45, axiom, (is_not_zero(Y) & is_not_zero(Z) & is_not_zero(X)) => '*'(pow(X, Y), pow(X, Z)) = pow(X, '+'(Y, Z))).
cnf(rule_46, axiom, '*'(pow(X, '2'), pow(Y, '2')) = pow('*'(X, Y), '2')).
cnf(rule_47, axiom, (is_not_zero(Y)) => '/'(X, '/'(Y, '/'(X, Y))) = pow('/'(X, Y), '2')).
cnf(rule_48, axiom, '/'(X, '/'(Y, '*'(Z, W))) = '/'(Z, '/'(Y, '*'(X, W)))).
cnf(rule_49, axiom, (is_not_zero(Y) & is_not_zero(X)) => '/'(X, '/'(pow(X, '2'), Y)) = '/'(Y, X)).
cnf(rule_50, axiom, (is_not_zero(X) & is_not_zero(Y)) => '/'(X, '/'(pow(Y, '2'), X)) = pow('/'(X, Y), '2')).
cnf(rule_51, axiom, (is_not_zero(Y)) => '/'(pow(X, '2'), '*'(Y, X)) = '/'(X, Y)).
cnf(rule_52, axiom, (is_not_zero(Z)) => '/'(X, '/'(Y, '*'(X, Z))) = '/'(pow(X, '2'), '/'(Y, Z))).
cnf(rule_53, axiom, (is_not_zero(Y) & is_not_zero(X)) => '/'(pow(X, '+'(Y, '1')), X) = pow(X, Y)).
cnf(rule_54, axiom, (is_not_zero(Y) & is_not_zero(X)) => '/'(pow(X, '+'('1', Y)), X) = pow(X, Y)).
cnf(rule_55, axiom, (is_not_zero(X)) => '/'(pow('*'(X, Y), '2'), X) = '*'(X, pow(Y, '2'))).
cnf(rule_56, axiom, pow(X, '+'(Y, '+'('1', '1'))) = pow(X, '+'(Y, '2'))).
cnf(rule_57, axiom, (is_not_zero(Y) & is_not_zero(X)) => pow(X, '+'('1', '+'(Y, Y))) = '*'(X, pow(pow(X, Y), '2'))).
cnf(rule_58, axiom, pow(X, '+'('1', '+'(Y, '1'))) = pow(X, '+'(Y, '2'))).
cnf(rule_59, axiom, pow(X, '+'('1', '+'('1', Y))) = pow(X, '+'(Y, '2'))).
cnf(rule_60, axiom, pow(X, '+'(Y, '+'(Z, W))) = pow(X, '+'(Y, '+'(W, Z)))).
cnf(rule_61, axiom, pow('1', '+'(X, '+'(X, '2'))) = pow(pow('1', X), '2')).
cnf(rule_62, axiom, pow('1', '+'(X, '+'(Y, '1'))) = pow('1', '+'(X, Y))).
cnf(rule_63, axiom, pow('1', '+'(X, '+'(Y, '2'))) = pow('1', '+'(X, Y))).
cnf(rule_64, axiom, (is_not_zero(X)) => '/'(pow('/'('1', X), Y), X) = pow('/'('1', X), '+'(Y, '1'))).
cnf(rule_65, axiom, (is_not_zero(X) & is_not_zero(Y)) => pow(X, '+'(Y, '+'(Y, Y))) = pow(pow(X, Y), '+'('2', '1'))).
cnf(rule_66, axiom, (is_not_zero(X) & is_not_zero(Y)) => pow(pow(X, '+'(Y, Y)), '2') = pow(pow(X, Y), '+'('2', '2'))).
cnf(rule_67, axiom, pow(X, '+'(Y, '+'(Y, '2'))) = pow(pow(X, '+'(Y, '1')), '2')).
cnf(rule_68, axiom, (is_not_zero(X) & is_not_zero(Y)) => pow(X, '+'('1', '+'(Y, '-1'))) = pow(X, Y)).
cnf(goal, conjecture, X / '0' = '0').
