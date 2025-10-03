cnf(add_comm, axiom,
    add(N, M) = add(M, N)).

cnf(add_assoc, axiom,
    add(N, add(M, K)) = add(add(N, M), K)).

cnf(add_zero, axiom,
    add(N, zero) = N).

cnf(add_inv, axiom,
    add(N, neg(N)) = zero).

cnf(mul_comm, axiom,
    mul(N, M) = mul(M, N)).

cnf(mul_assoc, axiom,
    mul(N, mul(M, K)) = mul(mul(N, M), K)).

cnf(mul_one, axiom,
    mul(N, one) = N).

cnf(mul_zero, axiom,
    mul(N, zero) = zero).

cnf(distrib_left, axiom,
    mul(N, add(M, K)) = add(mul(N, M), mul(N, K))).

cnf(distrib_right, axiom,
    mul(add(M, K), N) = add(mul(M, N), mul(K, N))).

cnf(mul_inv, axiom, ((X = one)) => (mul(X, inv(X)) = one)).


cnf(goal, conjecture,
    mul(one, inv(one)) = one).