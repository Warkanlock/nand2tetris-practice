// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// when key is pressed, screen should be black
// when no key is pressed, screen should be white

(init)
	@8192
	D=A

	@inc // set inc to 8192 (32*56)
	M=D

(start)
	@inc
	M=M-1
	D=M

	@init
	D;JLT // jump if less than zero to initialize

	@KBD
	D=M

	// if D > 1 paint black
	@paint_black
	D;JLT

	// if not, paint blank
	@paint_white
	D;JEQ // you can use directly a JMP here without condition

(paint_black)
	@SCREEN
	D=A

	@inc
	A=D+M // value of (inc) + screen address
	M=-1

	@start
	0;JMP

(paint_white)
	@SCREEN
	D=A

	@inc
	A=D+M // value of (inc) + screen address
	M=1 // 1 for strips

	@start
	0;JMP
