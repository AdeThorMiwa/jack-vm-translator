// function Class1.set 0
(Class1__Class1.set)
// push argument 0
	@ARG
	D=M
	@0
	A=D+A
	D=M
	@SP
	A=M
	M=D
	@SP
	M=M+1
// pop static 0
	@Class1.0
	D=A
	@R13
	M=D
	@SP
	AM=M-1
	D=M
	M=0
	@R13
	A=M
	M=D
// push argument 1
	@ARG
	D=M
	@1
	A=D+A
	D=M
	@SP
	A=M
	M=D
	@SP
	M=M+1
// pop static 1
	@Class1.1
	D=A
	@R13
	M=D
	@SP
	AM=M-1
	D=M
	M=0
	@R13
	A=M
	M=D
// push constant 0
	@0
	D=A
	@SP
	A=M
	M=D
	@SP
	M=M+1
// return
	@LCL
	D=M
	@R6
	M=D
	@R6
	D=M
	@5
	D=D-A
	A=D
	D=M
	@R7
	M=D
	@SP
	AM=M-1
	D=M
	M=0
	@ARG
	A=M
	M=D
	@ARG
	D=M+1
	@SP
	M=D
	@R6
	D=M
	@1
	D=D-A
	A=D
	D=M
	@THAT
	M=D
	@R6
	D=M
	@2
	D=D-A
	A=D
	D=M
	@THIS
	M=D
	@R6
	D=M
	@3
	D=D-A
	A=D
	D=M
	@ARG
	M=D
	@R6
	D=M
	@4
	D=D-A
	A=D
	D=M
	@LCL
	M=D
	@R6
	D=M
	@4
	D=D-A
	A=D
	A=M
	0;JMP
// function Class1.get 0
(Class1__Class1.get)
// push static 0
	@Class1.0
	D=M
	@SP
	A=M
	M=D
	@SP
	M=M+1
// push static 1
	@Class1.1
	D=M
	@SP
	A=M
	M=D
	@SP
	M=M+1
// sub
	@SP
	AM=M-1
	D=M
	M=0
	@SP
	AM=M-1
	D=M-D
	@SP
	A=M
	M=D
	@SP
	M=M+1
// return
	@LCL
	D=M
	@R6
	M=D
	@R6
	D=M
	@5
	D=D-A
	A=D
	D=M
	@R7
	M=D
	@SP
	AM=M-1
	D=M
	M=0
	@ARG
	A=M
	M=D
	@ARG
	D=M+1
	@SP
	M=D
	@R6
	D=M
	@1
	D=D-A
	A=D
	D=M
	@THAT
	M=D
	@R6
	D=M
	@2
	D=D-A
	A=D
	D=M
	@THIS
	M=D
	@R6
	D=M
	@3
	D=D-A
	A=D
	D=M
	@ARG
	M=D
	@R6
	D=M
	@4
	D=D-A
	A=D
	D=M
	@LCL
	M=D
	@R6
	D=M
	@4
	D=D-A
	A=D
	A=M
	0;JMP
// function Sys.init 0
(Sys__Sys.init)
// push constant 6
	@6
	D=A
	@SP
	A=M
	M=D
	@SP
	M=M+1
// push constant 8
	@8
	D=A
	@SP
	A=M
	M=D
	@SP
	M=M+1
// call Class1.set 2
	@$ret__1
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
