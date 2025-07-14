00200b18 <_ZN9fibonacci9fibonacci17h10ba975679d46493E>:
  200b18: 02050a63     	beqz	a0, 0x200b4c <_ZN9fibonacci9fibonacci17h10ba975679d46493E+0x34>
  200b1c: 00000713     	li	a4, 0x0
  200b20: 00100613     	li	a2, 0x1
  200b24: 000025b7     	lui	a1, 0x2
  200b28: eef58693     	addi	a3, a1, -0x111
  200b2c: 00e60733     	add	a4, a2, a4
  200b30: 02d775b3     	remu	a1, a4, a3
  200b34: fff50513     	addi	a0, a0, -0x1
  200b38: 00060713     	mv	a4, a2
  200b3c: 00058613     	mv	a2, a1
  200b40: fe0516e3     	bnez	a0, 0x200b2c <_ZN9fibonacci9fibonacci17h10ba975679d46493E+0x14>
  200b44: 00058513     	mv	a0, a1
  200b48: 00008067     	ret
  200b4c: 00100593     	li	a1, 0x1
  200b50: 00058513     	mv	a0, a1
  200b54: 00008067     	ret
