// push Constant 10
@10
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
// push Constant 21
@21
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 22
@22
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Argument 2
@SP
M=M-1
@2
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
// pop Argument 1
@SP
M=M-1
@1
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
// push Constant 36
@36
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop This 6
@SP
M=M-1
@6
D=A
@THIS
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
// push Constant 42
@42
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 45
@45
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 5
@SP
M=M-1
@5
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
// push Constant 510
@510
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Temp 6
@SP
M=M-1
A=M
D=M
@R11
M=D
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
// push That 5
@5
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
// sub
@SP
M=M-1
A=M
D=M
A=A-1
M=M-D
// push This 6
@6
D=A
@THIS
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push This 6
@6
D=A
@THIS
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
// sub
@SP
M=M-1
A=M
D=M
A=A-1
M=M-D
// push Temp 6
@R11
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
