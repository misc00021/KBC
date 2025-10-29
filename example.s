	.text
	.file	"example.c"
	.globl	example                         # -- Begin function example
	.p2align	4, 0x90
	.type	example,@function
example:                                # @example
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	%edi, -4(%rbp)
	xorl	%ecx, %ecx
	subl	-4(%rbp), %ecx
	xorl	%eax, %eax
	subl	%ecx, %eax
	xorl	%ecx, %ecx
	subl	-4(%rbp), %ecx
	addl	%ecx, %eax
	addl	-4(%rbp), %eax
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end0:
	.size	example, .Lfunc_end0-example
	.cfi_endproc
                                        # -- End function
	.ident	"clang version 19.1.2 (git@cc.cdl.uni-saarland.de:group_name/c4.git 3f79fac64fcd0d2087d40bfc420d71b97fd7359f)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
