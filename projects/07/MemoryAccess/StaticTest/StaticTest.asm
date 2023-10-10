// push Constant 111
@111
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 333
@333
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 888
@888
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Static 8
@SP
M=M-1
A=M
D=M
@StaticTest.vm.8
M=D
// pop Static 3
@SP
M=M-1
A=M
D=M
@StaticTest.vm.3
M=D
// pop Static 1
@SP
M=M-1
A=M
D=M
@StaticTest.vm.1
M=D
// push Static 3
@StaticTest.vm.3
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Static 1
@StaticTest.vm.1
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
// push Static 8
@StaticTest.vm.8
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
