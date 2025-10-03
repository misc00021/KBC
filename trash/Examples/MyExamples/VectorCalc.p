%--------------------------
% Scalar arithmetic
%--------------------------

cnf(add_comm, axiom, add(X, Y) = add(Y, X)).
cnf(add_assoc, axiom, add(X, add(Y, Z)) = add(add(X, Y), Z)).
cnf(add_zero_left, axiom, add('0', X) = X).
cnf(add_zero_right, axiom, add(X, '0') = X).
cnf(neg_add_cancel, axiom, add(X, neg(X)) = '0').
cnf(mul_comm, axiom, mul(X, Y) = mul(Y, X)).
cnf(mul_assoc, axiom, mul(X, mul(Y, Z)) = mul(mul(X, Y), Z)).
cnf(mul_one_left, axiom, mul('1', X) = X).
cnf(mul_zero_left, axiom, mul('0', X) = '0').
cnf(left_distrib, axiom, mul(X, add(Y, Z)) = add(mul(X, Y), mul(X, Z))).

%--------------------------
% Vector arithmetic
%--------------------------

cnf(vec_add_comm, axiom, vadd(A, B) = vadd(B, A)).
cnf(vec_add_assoc, axiom, vadd(A, vadd(B, C)) = vadd(vadd(A, B), C)).
cnf(vec_add_zero_left, axiom, vadd('0v', A) = A).
cnf(vec_add_zero_right, axiom, vadd(A, '0v') = A).
cnf(vec_add_inv, axiom, vadd(A, vneg(A)) = '0v').

%--------------------------
% Vector calculus identities
%--------------------------

% Linearity
cnf(gradDist, axiom, appl('NAB', vadd(A, B)) = vadd(appl('NAB', A), appl('NAB', B))).
cnf(divDist, axiom, dot('NAB', vadd(A, B)) = add(dot('NAB', A), dot('NAB', B))).
cnf(curlDist, axiom, cross('NAB', vadd(A, B)) = vadd(cross('NAB', A), cross('NAB', B))).

% Product rules
cnf(divProdRule, axiom, dot('NAB', vmul(F, A)) = add(mul(appl('NAB', F), A), mul(F, dot('NAB', A)))).
cnf(gradProdRule, axiom, appl('NAB', mul(F, G)) = add(mul(appl('NAB', F), G), mul(F, appl('NAB', G)))).

% Chain rule
cnf(chain, axiom, appl('NAB', appl(F, G)) = appl(appl(appl('NAB', F), G), appl('NAB', G))).

% Identities
cnf(curlOfGradZero, axiom, cross('NAB', appl('NAB', F)) = '0v').
cnf(divOfCurlZero, axiom, dot('NAB', cross('NAB', A)) = '0').

% Dot and cross products
cnf(scalar_triple_product, axiom,
    dot(A, cross(B, C)) = dot(B, cross(C, A))).

cnf(vector_triple_product, axiom,
    cross(A, cross(B, C)) = vsub(vmul(B, dot(A, C)), vmul(C, dot(A, B)))).

cnf(cross_comm_anti, axiom,
    cross(A, B) = vneg(cross(B, A))).

cnf(dot_comm, axiom,
    dot(A, B) = dot(B, A)).

%--------------------------
% Goal: contradiction
%--------------------------

cnf(test_goal, negated_conjecture, add('0', X) != X).