// function Sys.init 0
(Sys.Sys.init)
// push Constant 4000
@4000
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
// push Constant 5000
@5000
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
// call Sys.main 0
@71
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@LCL
M=D
@5
D=D-A
@ARG
M=D
@Sys.Sys.main
0;JMP
(Sys.Sys.init$ret.0)
// pop Temp 1
@SP
M=M-1
A=M
D=M
@R6
M=D
// label Sys.Sys.init$LOOP
(Sys.Sys.init$LOOP)
// goto Sys.Sys.init$LOOP
@Sys.Sys.init$LOOP
0;JMP
// function Sys.main 5
(Sys.Sys.main)
@SP
A=M
M=0
A=A+1
M=0
A=A+1
M=0
A=A+1
M=0
A=A+1
M=0
A=A+1
D=A
@SP
M=D
// push Constant 4001
@4001
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
// push Constant 5001
@5001
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
// push Constant 200
@200
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 1
@SP
M=M-1
@1
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
// push Constant 40
@40
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 2
@SP
M=M-1
@2
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
// push Constant 6
@6
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop Local 3
@SP
M=M-1
@3
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
// push Constant 123
@123
D=A
@SP
A=M
M=D
@SP
M=M+1
// call Sys.add12 1
@238
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@LCL
M=D
@6
D=D-A
@ARG
M=D
@Sys.Sys.add12
0;JMP
(Sys.Sys.main$ret.1)
// pop Temp 0
@SP
M=M-1
A=M
D=M
@R5
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
// push Local 1
@1
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local 2
@2
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local 3
@3
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
// push Local 4
@4
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
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=M+D
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=M+D
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=M+D
// return
@LCL
D=M-1
@R13
M=D
@4
D=D-A
A=D
D=M
@R14
M=D
@SP
A=M-1
D=M
@ARG
A=M
M=D
D=A
@SP
M=D+1
@R13
A=M
D=M
@THAT
M=D
@R13
M=M-1
A=M
D=M
@THIS
M=D
@R13
M=M-1
A=M
D=M
@ARG
M=D
@R13
M=M-1
A=M
D=M
@LCL
M=D
@R14
A=M
0;JMP
// function Sys.add12 0
(Sys.Sys.add12)
// push Constant 4002
@4002
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
// push Constant 5002
@5002
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
// push Constant 12
@12
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
// return
@LCL
D=M-1
@R13
M=D
@4
D=D-A
A=D
D=M
@R14
M=D
@SP
A=M-1
D=M
@ARG
A=M
M=D
D=A
@SP
M=D+1
@R13
A=M
D=M
@THAT
M=D
@R13
M=M-1
A=M
D=M
@THIS
M=D
@R13
M=M-1
A=M
D=M
@ARG
M=D
@R13
M=M-1
A=M
D=M
@LCL
M=D
@R14
A=M
0;JMP
