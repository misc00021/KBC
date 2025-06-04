% Commutativity and associativity
cnf(add_comm, axiom, plus(A, B) = plus(B, A)).
cnf(mul_comm, axiom, times(A, B) = times(B, A)).
cnf(add_assoc, axiom, plus(A, plus(B, C)) = plus(plus(A, B), C)).
cnf(mul_assoc, axiom, times(A, times(B, C)) = times(times(A, B), C)).

% Canonical and simplification forms
cnf(sub_to_add, axiom, minus(A, B) = plus(A, times('-1', B))).
cnf(add_zero_right, axiom, plus(A,'0') = A).
cnf(mul_zero_right, axiom, times(A,'0') ='0').
cnf(mul_one_right, axiom, times(A,'1') = A).
cnf(sub_cancel, axiom, minus(A, A) ='0').
cnf(div_to_mul, axiom, neq(B, '0') => div(A, B) = times(A, pow(B, '-1'))).
cnf(cancel_div, axiom, neq(A, '0') => div(A, A) = '1').
cnf(div_one, axiom, div(X, '1') = X).

% Distributivity and factoring
cnf(distribute, axiom, times(A, plus(B, C)) = plus(times(A, B), times(A, C))).
cnf(factor, axiom, plus(times(A, B), times(A, C)) = times(A, plus(B, C))).
cnf(mul_frac, axiom, (neq(B, '0') & neq(D, '0')) => times(div(A, B), div(C, D)) = div(times(A, C), times(B, D))).
cnf(int_frac_mul, axiom, neq(X, '0') => times(A, div(B, X)) = div(times(A, B), X)).

% Powers
cnf(pow_mul_merge, axiom, times(pow(A, B), pow(A, C)) = pow(A, plus(B, C))).
cnf(pow_zero, axiom, neq(X, '0') => pow(X, '0') = '1').
cnf(pow_one, axiom, pow(X,'1') = X).
cnf(pow_two, axiom, pow(X,'2') = times(X, X)).
cnf(pow_minus_one, axiom, neq(X, '0') => pow(X, '-1') = div('1', X)).

% Derivative
cnf(d_var, axiom, isVar(X) => der(X) = '1').
cnf(d_const, axiom, isConst(X) => der(X) = '0').
cnf(d_add, axiom, der(plus(A, B)) = plus(der(A), der(B))).
cnf(d_mul, axiom, der(times(A, B)) = plus(times(der(A), B), times(A, der(B)))).
cnf(d_pow, axiom, (neq(A, '0') & neq(B, '0')) => der(pow(A, B)) = times(B, pow(A, minus(B, '1')))).
cnf(d_div, axiom, (neq(A, '0') & neq(B, '0')) => der(div(A, B)) = div(minus(times(der(A), B), times(A, der(B))), pow(B, '2'))).

cnf(goal, conjecture, neq(d, '0') => times(d, div('1', d)) = '1').