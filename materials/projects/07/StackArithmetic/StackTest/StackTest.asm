@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@EQ_START_1
D;JEQ
D=0
@SP
A=M
M=D
@SP
M=M+1
@EQ_END_2
0;JMP
(EQ_START_1)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(EQ_END_2)
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@EQ_START_3
D;JEQ
D=0
@SP
A=M
M=D
@SP
M=M+1
@EQ_END_4
0;JMP
(EQ_START_3)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(EQ_END_4)
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@EQ_START_5
D;JEQ
D=0
@SP
A=M
M=D
@SP
M=M+1
@EQ_END_6
0;JMP
(EQ_START_5)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(EQ_END_6)
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@LT_START_7
D;JLT
D=0
@SP
A=M
M=D
@SP
M=M+1
@LT_END_8
0;JMP
(LT_START_7)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(LT_END_8)
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@LT_START_9
D;JLT
D=0
@SP
A=M
M=D
@SP
M=M+1
@LT_END_10
0;JMP
(LT_START_9)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(LT_END_10)
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@LT_START_11
D;JLT
D=0
@SP
A=M
M=D
@SP
M=M+1
@LT_END_12
0;JMP
(LT_START_11)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(LT_END_12)
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@GT_START_13
D;JGT
D=0
@SP
A=M
M=D
@SP
M=M+1
@GT_END_14
0;JMP
(GT_START_13)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(GT_END_14)
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@GT_START_15
D;JGT
D=0
@SP
A=M
M=D
@SP
M=M+1
@GT_END_16
0;JMP
(GT_START_15)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(GT_END_16)
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@GT_START_17
D;JGT
D=0
@SP
A=M
M=D
@SP
M=M+1
@GT_END_18
0;JMP
(GT_START_17)
D=-1
@SP
A=M
M=D
@SP
M=M+1
(GT_END_18)
@57
D=A
@SP
A=M
M=D
@SP
M=M+1
@31
D=A
@SP
A=M
M=D
@SP
M=M+1
@53
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A+D
@SP
A=M
M=D
@SP
M=M+1
@112
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=A-D
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
M=-M
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=D&A
@SP
A=M
M=D
@SP
M=M+1
@82
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
@SP
AM=M-1
A=M
D=D|A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
M=!M
(END)
@END
0;JMP
