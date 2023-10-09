// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

@state
M=0

(LOOP)
    @KBD
    D=M

    @WHITE
    D; JEQ

    @BLACK
    0; JMP

(WHITE)
    @state
    D=M

    @LOOP
    D; JEQ

    @state
    M=0

    @FILL
    0; JMP

(BLACK)
    @state
    D=M

    @LOOP
    D+1; JEQ

    @state
    M=-1

    @FILL
    0; JMP

(FILL)
    @0
    D=A

    @cnt
    M=D

(FILL_LOOP)
    @cnt
    D=M

    @8192
    D=A-D
    
    @LOOP // if we filled all the pixels go back to the beginning
    D; JEQ

    // M[SCREEN + cnt] = state
    @cnt
    D=M // D = cnt
    @SCREEN
    D=D+A // D = cnt + SCREEN

    @curr
    M=D // curr = SCREEN + cnt

    @state
    D=M // D = white/black

    @curr
    A=M
    M=D

    @cnt
    M=M+1

    @FILL_LOOP
    0; JMP
    


