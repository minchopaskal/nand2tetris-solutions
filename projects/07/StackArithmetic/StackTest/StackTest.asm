// push Constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true0
D; JEQ
@SP
A=M
A=A-1
M=0
@__cont0
0; JMP
(__eq.true0)
@SP
A=M
A=A-1
M=-1
(__cont0)
// push Constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true1
D; JEQ
@SP
A=M
A=A-1
M=0
@__cont1
0; JMP
(__eq.true1)
@SP
A=M
A=A-1
M=-1
(__cont1)
// push Constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
// eq
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true2
D; JEQ
@SP
A=M
A=A-1
M=0
@__cont2
0; JMP
(__eq.true2)
@SP
A=M
A=A-1
M=-1
(__cont2)
// push Constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// LT
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true3
D; JLT
@SP
A=M
A=A-1
M=0
@__cont3
0; JMP
(__eq.true3)
@SP
A=M
A=A-1
M=-1
(__cont3)
// push Constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
// LT
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true4
D; JLT
@SP
A=M
A=A-1
M=0
@__cont4
0; JMP
(__eq.true4)
@SP
A=M
A=A-1
M=-1
(__cont4)
// push Constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
// LT
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true5
D; JLT
@SP
A=M
A=A-1
M=0
@__cont5
0; JMP
(__eq.true5)
@SP
A=M
A=A-1
M=-1
(__cont5)
// push Constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// gt
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true6
D; JGT
@SP
A=M
A=A-1
M=0
@__cont6
0; JMP
(__eq.true6)
@SP
A=M
A=A-1
M=-1
(__cont6)
// push Constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
// gt
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true7
D; JGT
@SP
A=M
A=A-1
M=0
@__cont7
0; JMP
(__eq.true7)
@SP
A=M
A=A-1
M=-1
(__cont7)
// push Constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
// gt
@SP
M=M-1
A=M
D=M
A=A-1
MD=M-D
@__eq.true8
D; JGT
@SP
A=M
A=A-1
M=0
@__cont8
0; JMP
(__eq.true8)
@SP
A=M
A=A-1
M=-1
(__cont8)
// push Constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1
// push Constant 53
@53
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
// push Constant 112
@112
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
// neg
@SP
A=M
A=A-1
M=-M
// and
@SP
M=M-1
A=M
D=M
A=A-1
M=M&D
// push Constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1
// or
@SP
M=M-1
A=M
D=M
A=A-1
M=M|D
// not
@SP
A=M
A=A-1
M=!M
