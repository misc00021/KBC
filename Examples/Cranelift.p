cnf(r1, axiom, iadd(x,'0') = x).
cnf(r2, axiom, isub(x,'0') = x).
cnf(r3, axiom, isub('0', x) = ineg(x)).
cnf(r4, axiom, iadd(x, ineg(y)) = isub(x, y)).
cnf(r5, axiom, iadd(ineg(y), x) = isub(x, y)).
cnf(r6, axiom, ineg(isub(y, x)) = isub(x, y)).
cnf(r7, axiom, isub(x, ineg(y)) = iadd(x, y)).
cnf(r8, axiom, ineg(ineg(x)) = x).
cnf(r9, axiom, imul(ineg(x), ineg(y)) = imul(x, y)).
cnf(r10, axiom, iabs(ineg(x)) = iabs(x)).
cnf(r11, axiom, iabs(iabs(x)) = iabs(x)).
cnf(r12, axiom, isub(x, x) ='0').
cnf(r13, axiom, imul(x,'1') = x).
cnf(r14, axiom, imul(x,'0') ='0').
cnf(r15, axiom, imul(x, neg('1')) = ineg(x)).
cnf(r16, axiom, iadd(bnot(x),'1') = ineg(x)).
cnf(r17, axiom, bnot(isub(x,'1')) = ineg(x)).
cnf(r18, axiom, bnot(iadd(x, neg('1'))) = ineg(x)).
cnf(r19, axiom, sdiv(x,'1') = x).
cnf(r20, axiom, udiv(x,'1') = x).
cnf(r21, axiom, imul(x, '2') = iadd(x, x)).
cnf(r22, axiom, power_of_two(C) -> imul(x, C) = ishl(x, log2(C))).
cnf(r23, axiom, power_of_two(C) -> imul(C, x) = ishl(x, log2(C))).
cnf(r24, axiom, fneg(fneg(x)) = x).
cnf(r25, axiom, fma(fneg(x), fneg(y), z) = fma(x, y, z)).
cnf(r26, axiom, fmul(fneg(x), fneg(y)) = fmul(x, y)).

cnf(r27, axiom, iadd(a, iadd(b, iadd(c, d))) = iadd(iadd(a, b), iadd(c, d))).
cnf(r28, axiom, iadd(iadd(iadd(a, b), c), d) = iadd(iadd(a, b), iadd(c, d))).
cnf(r29, axiom, imul(a, imul(b, imul(c, d))) = imul(imul(a, b), imul(c, d))).
cnf(r30, axiom, imul(imul(imul(a, b), c), d) = imul(imul(a, b), imul(c, d))).
cnf(r31, axiom, band(a, band(b, band(c, d))) = band(band(a, b), band(c, d))).
cnf(r32, axiom, band(band(band(a, b), c), d) = band(band(a, b), band(c, d))).
cnf(r33, axiom, bxor(a, bxor(b, bxor(c, d))) = bxor(bxor(a, b), bxor(c, d))).
cnf(r34, axiom, bxor(bxor(bxor(a, b), c), d) = bxor(bxor(a, b), bxor(c, d))).

cnf(r35, axiom, isub(a, isub(b, isub(c, d))) = iadd(isub(a, b), isub(c, d))).
cnf(r36, axiom, isub(a, isub(b, iadd(c, d))) = iadd(isub(a, b), iadd(c, d))).
cnf(r37, axiom, isub(a, iadd(b, isub(c, d))) = isub(isub(a, b), isub(c, d))).
cnf(r38, axiom, isub(a, iadd(b, iadd(c, d))) = isub(isub(a, b), iadd(c, d))).
cnf(r39, axiom, iadd(a, isub(b, isub(c, d))) = isub(iadd(a, b), isub(c, d))).
cnf(r40, axiom, iadd(a, isub(b, iadd(c, d))) = isub(iadd(a, b), iadd(c, d))).
cnf(r41, axiom, iadd(a, iadd(b, isub(c, d))) = iadd(iadd(a, b), isub(c, d))).

cnf(r42, axiom, isub(isub(isub(a, b), c), d) = isub(isub(a, b), iadd(c, d))).
cnf(r43, axiom, iadd(isub(isub(a, b), c), d) = isub(isub(a, b), isub(c, d))).
cnf(r44, axiom, isub(iadd(isub(a, b), c), d) = iadd(isub(a, b), isub(c, d))).
cnf(r45, axiom, iadd(iadd(isub(a, b), c), d) = iadd(isub(a, b), iadd(c, d))).
cnf(r46, axiom, isub(isub(iadd(a, b), c), d) = isub(iadd(a, b), iadd(c, d))).
cnf(r47, axiom, iadd(isub(iadd(a, b), c), d) = isub(iadd(a, b), isub(c, d))).
cnf(r48, axiom, isub(iadd(iadd(a, b), c), d) = iadd(iadd(a, b), isub(c, d))).

cnf(r49, axiom, halfWidth(k) -> sshr(imul(sextend(x), sextend(y)), k) = sextend(smulhi(x, y))).
cnf(r50, axiom, halfWidth(k) -> ushr(imul(uextend(x), uextend(y)), k) = uextend(umulhi(x, y))).

cnf(r51, axiom, fcvt_from_uint(uextend(x)) = fcvt_from_uint(x)).
cnf(r52, axiom, fcvt_from_sint(sextend(x)) = fcvt_from_sint(x)).

cnf(r53, axiom, iadd(bor(x, C), -C) = band(x, not(C))).

cnf(goal, conjecture, true = true).