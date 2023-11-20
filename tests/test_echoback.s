	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p1_m2p0_a2p1_f2p2_d2p2_zicsr2p0_zifencei2p0"
	.file	"test_echoback.c"
	.globl	main                            # -- Begin function main
	.p2align	2
	.type	main,@function
main:                                   # @main
# %bb.0:
	addi	sp, sp, -48
	sd	ra, 40(sp)                      # 8-byte Folded Spill
	sd	s0, 32(sp)                      # 8-byte Folded Spill
	addi	s0, sp, 48
	li	a0, 0
	sw	a0, -20(s0)
	j	.LBB0_1
.LBB0_1:                                # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_2 Depth 2
	lui	a0, 65536
	sd	a0, -32(s0)
	j	.LBB0_2
.LBB0_2:                                #   Parent Loop BB0_1 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	ld	a0, -32(s0)
	lbu	a0, 5(a0)
	andi	a0, a0, 1
	bnez	a0, .LBB0_4
	j	.LBB0_3
.LBB0_3:                                #   in Loop: Header=BB0_2 Depth=2
	j	.LBB0_2
.LBB0_4:                                #   in Loop: Header=BB0_1 Depth=1
	ld	a0, -32(s0)
	lbu	a0, 0(a0)
	sb	a0, -33(s0)
	lbu	a0, -33(s0)
	li	a1, 97
	blt	a0, a1, .LBB0_7
	j	.LBB0_5
.LBB0_5:                                #   in Loop: Header=BB0_1 Depth=1
	lbu	a1, -33(s0)
	li	a0, 122
	blt	a0, a1, .LBB0_7
	j	.LBB0_6
.LBB0_6:                                #   in Loop: Header=BB0_1 Depth=1
	lbu	a0, -33(s0)
	addiw	a0, a0, -32
	sb	a0, -33(s0)
	j	.LBB0_7
.LBB0_7:                                #   in Loop: Header=BB0_1 Depth=1
	lbu	a0, -33(s0)
	ld	a1, -32(s0)
	sb	a0, 0(a1)
	j	.LBB0_1
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
                                        # -- End function
	.ident	"Homebrew clang version 17.0.5"
	.section	".note.GNU-stack","",@progbits
	.addrsig
