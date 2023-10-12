// push Constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 0
@SP
M=M-1
@0
D=A
@LCL
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
// label LOOP_START
(LOOP_START)
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
// push Local 0
@0
D=A
@LCL
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
// pop Local 0
@SP
M=M-1
@0
D=A
@LCL
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
// if-goto LOOP_START
@SP
M=M-1
A=M
D=M
@LOOP_START
D;JGT
// push Local 0
@0
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
