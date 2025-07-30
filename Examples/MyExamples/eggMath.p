% Commutativity and associativity
cnf(comm_add, axiom, plus(A, B) = plus(B, A)).
cnf(comm_mul, axiom, times(A, B) = times(B, A)).
cnf(assoc_add, axiom, plus(A, plus(B, C)) = plus(plus(A, B), C)).
cnf(assoc_mul, axiom, times(A, times(B, C)) = times(times(A, B), C)).

% Canonical and simplification forms
cnf(sub_to_add, axiom, minus(A, B) = plus(A, times('-1', B))).
cnf(div_to_mul, axiom, neq(B, '0') => div(A, B) = times(A, pow(B, '-1'))).
cnf(add_zero_right, axiom, plus(A, '0') = A).
cnf(mul_zero_right, axiom, times(A, '0') = '0').
cnf(mul_one_right, axiom, times(A, '1') = A).
cnf(add_zero_left, axiom, A = plus(A, '0')).  % From "add-zero"
cnf(mul_one_left, axiom, A = times(A, '1')).  % From "mul-one"
cnf(sub_cancel, axiom, minus(A, A) = '0').
cnf(cancel_div, axiom, neq(A, '0') => div(A, A) = '1').

% Distributivity and factoring
cnf(distribute, axiom, times(A, plus(B, C)) = plus(times(A, B), times(A, C))).
cnf(factor, axiom, plus(times(A, B), times(A, C)) = times(A, plus(B, C))).

% Powers
cnf(pow_mul_merge, axiom, times(pow(A, B), pow(A, C)) = pow(A, plus(B, C))).
cnf(pow_zero, axiom, neq(X, '0') => pow(X, '0') = '1').
cnf(pow_one, axiom, pow(X, '1') = X).
cnf(pow_two, axiom, pow(X, '2') = times(X, X)).
cnf(pow_minus_one, axiom, neq(X, '0') => pow(X, '-1') = div('1', X)).
cnf(recip_mul_div, axiom, neq(X, '0') => times(X, div('1', X)) = '1').

% Derivative rules
cnf(d_var, axiom, isSym(X) => der(X, X) = '1').
cnf(d_const, axiom, (isSym(X) & isConstOrDistinctVar(C, X)) => der(X, C) = '0').
cnf(d_add, axiom, der(X, plus(A, B)) = plus(der(X, A), der(X, B))).
cnf(d_mul, axiom, der(X, times(A, B)) = plus(times(A, der(X, B)), times(B, der(X, A)))).
cnf(d_sin, axiom, der(X, sin(X)) = cos(X)).
cnf(d_cos, axiom, der(X, cos(X)) = times('-1', sin(X))).
cnf(d_ln, axiom, neq(X, '0') => der(X, ln(X)) = div('1', X)).

cnf(d_power, axiom,
    (neq(F, '0') & neq(G, '0')) =>
    der(X, pow(F, G)) =
    times(
        pow(F, G),
        plus(
            times(der(X, F), div(G, F)),
            times(der(X, G), ln(F))
        )
    )
).

% Integration rules
cnf(i_one, axiom, int('1', X) = X).
cnf(i_power_const, axiom, isConst(C) => int(pow(X, C), X) = div(pow(X, plus(C, '1')), plus(C, '1'))).
cnf(i_cos, axiom, int(cos(X), X) = sin(X)).
cnf(i_sin, axiom, int(sin(X), X) = times('-1', cos(X))).
cnf(i_sum, axiom, int(plus(F, G), X) = plus(int(F, X), int(G, X))).
cnf(i_dif, axiom, int(minus(F, G), X) = minus(int(F, X), int(G, X))).
cnf(i_parts, axiom,
    int(times(A, B), X) =
    minus(
        times(A, int(B, X)),
        int(times(der(X, A), int(B, X)), X)
    )
).

cnf(goal, conjecture, div('1', sub(div(add('1', X), '2'), div(sub('1', X), '2'))) = div('1', X)).