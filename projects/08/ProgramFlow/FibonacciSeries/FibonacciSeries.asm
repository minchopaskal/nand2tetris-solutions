// push Argument 1
@1
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// pop Pointer 1
@SP
M=M-1
A=M
D=M
@THAT
M=D
// push Constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 0
@SP
M=M-1
@0
D=A
@THAT
A=D+M
D=A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
// push Constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 1
@SP
M=M-1
@1
D=A
@THAT
A=D+M
D=A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
// push Argument 0
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
M=M-1
A=M
D=M
A=A-1
M=M-D
// pop Argument 0
@SP
M=M-1
@0
D=A
@ARG
A=D+M
D=A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
// label MAIN_LOOP_START
(MAIN_LOOP_START)
// push Argument 0
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// if-goto COMPUTE_ELEMENT
@SP
M=M-1
A=M
D=M
@COMPUTE_ELEMENT
D;JGT
// goto END_PROGRAM
@END_PROGRAM
0;JMP
// label COMPUTE_ELEMENT
(COMPUTE_ELEMENT)
// push That 0
@0
D=A
@THAT
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push That 1
@1
D=A
@THAT
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=M+D
// pop That 2
@SP
M=M-1
@2
D=A
@THAT
A=D+M
D=A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
// push Pointer 1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=M+D
// pop Pointer 1
@SP
M=M-1
A=M
D=M
@THAT
M=D
// push Argument 0
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// sub
@SP
M=M-1
A=M
D=M
A=A-1
M=M-D
// pop Argument 0
@SP
M=M-1
@0
D=A
@ARG
A=D+M
D=A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
// goto MAIN_LOOP_START
@MAIN_LOOP_START
0;JMP
// label END_PROGRAM
(END_PROGRAM)
