cnf(test, axiom, X+'1' = X - neg('1')).
%cnf(test2, axiom, X-Y = X-neg(neg(Y))).
cnf(test3, axiom, X+Y = Y+X).
cnf(test4, axiom, X+Y = X-neg(Y))
cnf(goal, conjecture, x=x).