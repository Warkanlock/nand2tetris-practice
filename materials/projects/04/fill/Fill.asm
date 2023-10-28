// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// when key is pressed, screen should be black
// when no key is pressed, screen should be white

(INIT_VAR)
	@8192	
	D=A

	// set initialize to 8192 (32 * 256 pixels)
	@i 
	M=D

(INIT_LOOP)
	@i
	M=M-1
	D=M

	@INIT_VAR
	D;JLT // (index < 0, reset)

	@KBD
	D=M

	@PIXEL_BLACK
	D;JEQ

	@PIXEL_WHITE
	D;JMP

(PIXEL_BLACK)             
	@SCREEN
	D=A
	@i
	A=D+M
	M=-1  
	@INIT_LOOP  
	0;JMP

(PIXEL_WHITE)
	@SCREEN
	D=A                
	@i    
	A=D+M 
	M=0    
	@INIT_LOOP  
	0;JMP
