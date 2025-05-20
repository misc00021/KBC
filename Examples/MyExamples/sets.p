cnf(union_comm, axiom,
    union(A, B) = union(B, A)).

cnf(union_assoc, axiom,
    union(union(A, B), C) = union(A, union(B, C))).

cnf(union_empty, axiom,
    union(A, empty) = A).

cnf(union_full, axiom,
    union(A, full) = full).

cnf(union_self, axiom,
    union(A, A) = A).

cnf(inter_comm, axiom,
    inter(A, B) = inter(B, A)).

cnf(inter_assoc, axiom,
    inter(inter(A, B), C) = inter(A, inter(B, C))).

cnf(inter_empty, axiom,
    inter(A, empty) = empty).

cnf(inter_full, axiom,
    inter(A, full) = A).

cnf(inter_self, axiom,
    inter(A, A) = A).

cnf(diff_empty, axiom,
    diff(A, empty) = A).

cnf(diff_full, axiom,
    diff(A, full) = empty).

cnf(diff_self, axiom,
    diff(A, A) = empty).

    cnf(goal, conjecture,
    true = false).