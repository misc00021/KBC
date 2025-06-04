% Abb. 1 - https://arxiv.org/pdf/2404.12336

% --- Gate Rules ---
cnf(gate_left, axiom,
    mux(S, B, C) = mux(S, and(B, maskb(S)), C)).

cnf(gate_right, axiom,
    mux(S, B, C) = mux(S, B, and(C, maskc(not(S))))).

% --- Mask Propagation Rules ---
cnf(propagate_mask, axiom,
    and(op1(A, B), masko(S)) = op1(and(A, maska(S)), and(B, maskb(S)))).

cnf(propagate_mask_left, axiom,
    and(op2(A, B), masko(S)) = op2(and(A, maska(S)), B)).

cnf(propagate_mux_mask, axiom,
    and(mux(S1, A, B), masko(S2)) = mux(S1, and(A, maska(S2)), and(B, maskb(S2)))).

cnf(propagate_mux_mask_right, axiom,
    and(mux(S1, A, B), masko(S2)) = mux(and(S1, S2), A, and(B, maskb(S2)))).

cnf(propagate_mux_mask_left, axiom,
    and(mux(S1, A, B), masko(S2)) = mux(or(S1, S2), and(A, maska(S2)), B)).

cnf(combine_masks, axiom,
    and(maska(S1), maska(S2)) = maska(and(S1, S2))).

% --- Transparent Register Rules ---
cnf(transp_reg_left, axiom,
    mux(S, B, C) = mux(S, treg(B, S), C)).

cnf(transp_reg_right, axiom,
    mux(S, B, C) = mux(S, B, treg(C, not(S)))).

cnf(transp_reg_mask, axiom,
    and(A, mask(S)) = and(treg(A, S), mask(S))).

cnf(transp_reg_saturate, axiom,
    or(A, mask(S)) = or(treg(A, not(S)), mask(S))).

cnf(transp_reg_reg, axiom,
    reg(A, En) = reg(treg(A, En), En)).

cnf(propagate_treg_op, axiom,
    treg(op(A, B), S) = op(treg(A, S), treg(B, S))).

cnf(propagate_treg_mux, axiom,
    treg(mux(S1, A, B), S2) = mux(treg(S1, S2), treg(A, S2), treg(B, S2))).

cnf(combine_transp_regs, axiom,
    treg(treg(A, S1), S2) = treg(A, and(S1, S2))).

% --- Clock Gating and Retime ---
cnf(retime_boolean, axiom,
    op3(reg(A, En), reg(B, En)) = reg(op3(A, B), En)).

cnf(clock_gate_reg, axiom,
    treg(reg(A, En), reg(B, En)) = reg(A, and(En, B))).

cnf(goal, conjecture, true = false).