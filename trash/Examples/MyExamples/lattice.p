cnf(idempotent_join, axiom,
    X ∨ X = X).

cnf(idempotent_meet, axiom,
    X ∧ X = X).

cnf(commutative_join, axiom,
    X ∨ Y = Y ∨ X).

cnf(commutative_meet, axiom,
    X ∧ Y = Y ∧ X).

cnf(associative_join, axiom,
    X ∨ (Y ∨ Z) = (X ∨ Y) ∨ Z).

cnf(associative_meet, axiom,
    X ∧ (Y ∧ Z) = (X ∧ Y) ∧ Z).

cnf(absorption_join, axiom,
    X ∨ (X ∧ Y) = X).

cnf(absorption_meet, axiom,
    X ∧ (X ∨ Y) = X).

cnf(goal, conjecture, x=x).