00200b18 <_ZN9fibonacci9fibonacci17h10ba975679d46493E>:
  200b18: 04050a63     	beqz	a0, 0x200b6c <_ZN9fibonacci9fibonacci17h10ba975679d46493E+0x54>
  200b1c: 00000793     	li	a5, 0x0
  200b20: 00100613     	li	a2, 0x1
  200b24: 08d355b7     	lui	a1, 0x8d35
  200b28: a9558693     	addi	a3, a1, -0x56b
  200b2c: 000025b7     	lui	a1, 0x2
  200b30: eef58713     	addi	a4, a1, -0x111
  200b34: 00f607b3     	add	a5, a2, a5
  200b38: 02d7b5b3     	mulhu	a1, a5, a3
  200b3c: 40b78833     	sub	a6, a5, a1
  200b40: 00185813     	srli	a6, a6, 0x1
  200b44: 00b805b3     	add	a1, a6, a1
  200b48: 00c5d593     	srli	a1, a1, 0xc
  200b4c: 02e585b3     	mul	a1, a1, a4
  200b50: 40b785b3     	sub	a1, a5, a1
  200b54: fff50513     	addi	a0, a0, -0x1
  200b58: 00060793     	mv	a5, a2
  200b5c: 00058613     	mv	a2, a1
  200b60: fc051ae3     	bnez	a0, 0x200b34 <_ZN9fibonacci9fibonacci17h10ba975679d46493E+0x1c>
  200b64: 00058513     	mv	a0, a1
  200b68: 00008067     	ret
  200b6c: 00100593     	li	a1, 0x1
  200b70: 00058513     	mv	a0, a1
  200b74: 00008067     	ret
