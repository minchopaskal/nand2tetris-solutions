// push Constant 3030
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D
// push Constant 3040
@3040
D=A
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
// push Constant 32
@32
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop This 2
@SP
M=M-1
@2
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
// push Constant 46
@46
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop That 6
@SP
M=M-1
@6
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
// push Pointer 0
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Pointer 1
@THAT
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
// push This 2
@2
D=A
@THIS
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
// push That 6
@6
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
