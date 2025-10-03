cnf(bvadd_comm, axiom,
    add(X, Y) = add(Y, X)).

cnf(bvadd_assoc, axiom,
    add(add(X, Y), Z) = add(X, add(Y, Z))).

cnf(bvadd_id, axiom,
    add(X, zero) = X).

cnf(bvadd_neg, axiom,
    add(X, neg(X)) = zero).

cnf(bvsub_self, axiom,
    sub(X, X) = zero).

cnf(bvsub_def, axiom,
    sub(X, Y) = add(X, neg(Y))).

cnf(bvmul_comm, axiom,
    mul(X, Y) = mul(Y, X)).

cnf(bvmul_assoc, axiom,
    mul(mul(X, Y), Z) = mul(X, mul(Y, Z))).

cnf(bvmul_id, axiom,
    mul(X, one) = X).

cnf(bvmul_zero, axiom,
    mul(X, zero) = zero).

cnf(bvand_comm, axiom,
    and(X, Y) = and(Y, X)).

cnf(bvand_assoc, axiom,
    and(and(X, Y), Z) = and(X, and(Y, Z))).

cnf(bvand_id, axiom,
    and(X, allones) = X).

cnf(bvand_zero, axiom,
    and(X, zero) = zero).

cnf(bvor_comm, axiom,
    or(X, Y) = or(Y, X)).

cnf(bvor_assoc, axiom,
    or(or(X, Y), Z) = or(X, or(Y, Z))).

cnf(bvor_id, axiom,
    or(X, zero) = X).

cnf(bvor_neg, axiom,
    or(X, not(X)) = allones).

cnf(bvxor_comm, axiom,
    xor(X, Y) = xor(Y, X)).

cnf(bvxor_assoc, axiom,
    xor(xor(X, Y), Z) = xor(X, xor(Y, Z))).

cnf(bvxor_id, axiom,
    xor(X, zero) = X).

cnf(bvxor_self, axiom,
    xor(X, X) = zero).

cnf(bvshl_zero, axiom,
    shl(X, zero) = X).

cnf(bvshr_zero, axiom,
    shr(X, zero) = X).

cnf(bvand_self, axiom,
    and(X, X) = X).

cnf(bvor_self, axiom,
    or(X, X) = X).

cnf(bvxor_zero, axiom,
    xor(X, zero) = X).

cnf(bvnot_self, axiom,
    not(X) = sub(neg(X), one)).

cnf(bvshl_def, axiom,
    shl(X, Y) = mul(X, shl(one, Y))).

cnf(bvsub_zero, axiom,
    sub(X, zero) = X).

cnf(bvsub_id, axiom,
    sub(zero, X) = neg(X)).

cnf(bvadd_sub, axiom,
    add(X, neg(Y)) = sub(X, Y)).

cnf(bvsub_add, axiom,
    sub(X, neg(Y)) = add(X, Y)).

cnf(bvand_allones, axiom,
    and(X, allones) = X).

cnf(bvor_allzeros, axiom,
    or(X, zero) = X).

cnf(bvxor_allzeros, axiom,
    xor(X, zero) = X).

cnf(bvand_or, axiom,
    and(X, or(Y, Z)) = or(and(X, Y), and(X, Z))).

cnf(bvor_and, axiom,
    or(X, and(Y, Z)) = and(or(X, Y), or(X, Z))).

cnf(bvand_absorb, axiom,
    and(X, or(X, Y)) = X).

cnf(bvor_absorb, axiom,
    or(X, and(X, Y)) = X).

cnf(bvshl_zero_shift, axiom,
    shl(X, zero) = X).

cnf(bvshr_zero_shift, axiom,
    shr(X, zero) = X).

cnf(bvshl_allzeros, axiom,
    shl(zero, Y) = zero).

cnf(bvshr_allzeros, axiom,
    shr(zero, Y) = zero).

cnf(bvor_and_not, axiom,
    or(and(X, Y), and(X, not(Y))) = X).

cnf(bvshl_and, axiom,
    shl(and(X, Y), Z) = and(shl(X, Z), shl(Y, Z))).

cnf(bvshr_and, axiom,
    shr(and(X, Y), Z) = and(shr(X, Z), shr(Y, Z))).

cnf(goal, conjecture,
    true = false).