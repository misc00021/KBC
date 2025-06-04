cnf('append_nil', axiom, append(nil, X, X)).
cnf('append_cons', axiom, append(cons(X,Xs), Ys) = cons(X, append(Xs,Ys))).
cnf(append_assoc, axiom, append(append(Xs,Ys),Zs) = append(Xs,append(Ys,Zs))).

cnf(rev_rev, axiom, comp(rev,rev) = id).
cnf(comp_assoc, axiom, comp(comp(X,Y),Z) = comp(X,comp(Y,Z))).
cnf(comp_id, axiom, comp(id,X) = X).
cnf(comp_id2, axiom, comp(X,id) = X).
cnf(map_fuse, axiom, map(comp(F,G)) = comp(map(F), map(G))).
cnf(map_id, axiom, map(id) = id).