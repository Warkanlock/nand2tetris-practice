// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.


@R1
A=M

@R0
D=M 

@R2
M=0 

// loop to multiply R0*R1 i times
(LOOP)
	// jump if D <= 0
	@END
	D;JEQ

	@R1
	D=M

	@R2
	M=D+M // R1 + R2

	// create internal counter
	@R0
	D=M 

	// decrement D
	D=D-1 

	// assign D to M
	@R0 
	M=D 

	// loop
	@LOOP
	0;JMP // repeat loop

// end program
(END)
	@END
	0;JMP
