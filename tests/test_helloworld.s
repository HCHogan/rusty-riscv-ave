	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p1_m2p0_a2p1_f2p2_d2p2_zicsr2p0_zifencei2p0"
	.file	"test_helloworld.c"
	.globl	main                            # -- Begin function main
	.p2align	2
	.type	main,@function
main:                                   # @main
# %bb.0:
	addi	sp, sp, -32
	sd	ra, 24(sp)                      # 8-byte Folded Spill
	sd	s0, 16(sp)                      # 8-byte Folded Spill
	addi	s0, sp, 32
	li	a0, 0
	sw	a0, -20(s0)
	lui	a1, 65536
	sd	a1, -32(s0)
	ld	a2, -32(s0)
	li	a1, 72
	sb	a1, 0(a2)
	ld	a2, -32(s0)
	li	a1, 101
	sb	a1, 0(a2)
	ld	a2, -32(s0)
	li	a1, 108
	sb	a1, 0(a2)
	ld	a2, -32(s0)
	sb	a1, 0(a2)
	ld	a3, -32(s0)
	li	a2, 111
	sb	a2, 0(a3)
	ld	a4, -32(s0)
	li	a3, 44
	sb	a3, 0(a4)
	ld	a4, -32(s0)
	li	a3, 32
	sb	a3, 0(a4)
	ld	a4, -32(s0)
	li	a3, 119
	sb	a3, 0(a4)
	ld	a3, -32(s0)
	sb	a2, 0(a3)
	ld	a3, -32(s0)
	li	a2, 114
	sb	a2, 0(a3)
	ld	a2, -32(s0)
	sb	a1, 0(a2)
	ld	a2, -32(s0)
	li	a1, 100
	sb	a1, 0(a2)
	ld	a2, -32(s0)
	li	a1, 33
	sb	a1, 0(a2)
	ld	a2, -32(s0)
	li	a1, 10
	sb	a1, 0(a2)
	ld	ra, 24(sp)                      # 8-byte Folded Reload
	ld	s0, 16(sp)                      # 8-byte Folded Reload
	addi	sp, sp, 32
	ret
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
                                        # -- End function
	.ident	"Homebrew clang version 17.0.5"
	.section	".note.GNU-stack","",@progbits
	.addrsig
