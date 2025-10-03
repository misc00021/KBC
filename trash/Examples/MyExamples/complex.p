tff(type, type, 'c' : ($i * $i) > $c).
tff(type, type, 'plus' : ($c * $c) > $c).
tff(type, type, 'real' : $c > $i).
tff(type, type, 'imag' : $c > $i).
tff(type, type, 'times' : ($c * $c) > $c).
tff(type, type, 'zero' : $c).
tff(type, type, 'one' : $c).
tff(type, type, 'cneg' : $c > $c).


% Addition (component-wise)
cnf(add_real, axiom,
    real(plus(c(R1,I1), c(R2,I2))) = R1 + R2).

cnf(add_imag, axiom,
    imag(plus(c(R1,I1), c(R2,I2))) = I1 + I2).

cnf(plus_def, axiom,
    plus(c(R1,I1), c(R2,I2)) = c(R1 + R2, I1 + I2)).

% Multiplication
cnf(mul_real, axiom,
    real(times(c(R1,I1), c(R2,I2))) = (R1*R2) - (I1*I2)).

cnf(mul_imag, axiom,
    imag(times(c(R1,I1), c(R2,I2))) = (R1*I2) + (I1*R2)).

cnf(times_def, axiom,
    times(c(R1,I1), c(R2,I2)) = c((R1*R2) - (I1*I2), (R1*I2) + (I1*R2))).

% Additive identity
cnf(add_id_left, axiom,
    '0' + X = X).

cnf(zero_def, axiom,
    zero = c('0','0')).

% Multiplicative identity
cnf(mul_id_left, axiom,
    '1' * X = X).

cnf(one_def, axiom,
    one = c('1','0')).

% Additive inverse
cnf(add_inv_def, axiom,
    X + neg(X) = '0').

cnf(neg_def, axiom,
    cneg(c(R,I)) = c(neg(R), neg(I))).

% Multiplicative inverse (partial; symbolic)
% cnf(mul_inv_def, axiom,
%    X * inv(X) = '1').

% Optional: Define negation and multiplication on real numbers here, or treat them as primitive.

% Real arithmetic axioms (assuming +, *, - on real parts are predefined)
cnf(real_add_assoc, axiom,
    (R1 + R2) + R3 = R1 + (R2 + R3)).

cnf(real_add_comm, axiom,
    R1 + R2 = R2 + R1).

cnf(real_add_id, axiom,
    '0' + R = R).

cnf(real_add_inv, axiom,
    R + neg(R) = '0').

cnf(real_mul_assoc, axiom,
    (R1 * R2) * R3 = R1 * (R2 * R3)).

cnf(real_mul_comm, axiom,
    R1 * R2 = R2 * R1).

cnf(real_mul_id, axiom,
    '1' * R = R).

cnf(real_dist, axiom,
    R1 * (R2 + R3) = (R1 * R2) + (R1 * R3)).

cnf(goal, conjecture, true = false).