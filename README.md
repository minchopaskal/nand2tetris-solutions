# nand2tetris-solutions

## Description

Solutions to Coursera's Nand2Tetris Course.

[Nand2Tetris Part 1](https://www.coursera.org/learn/build-a-computer)
[Nand2Tetris Part 2](https://www.coursera.org/learn/nand2tetris2)

## nand2tetris project files errata

```
--- a/projects/08/FunctionCalls/FibonacciElement/FibonacciElement.tst
+++ b/projects/08/FunctionCalls/FibonacciElement/FibonacciElement.tst
@@ -11,6 +11,8 @@ output-file FibonacciElement.out,
 compare-to FibonacciElement.cmp,
 output-list RAM[0]%D1.6.1 RAM[261]%D1.6.1;

+set RAM[0] 261,
+
 repeat 6000 {
   ticktock;
 }
```

```
--- a/projects/08/FunctionCalls/StaticsTest/StaticsTest.tst
+++ b/projects/08/FunctionCalls/StaticsTest/StaticsTest.tst
@@ -8,7 +8,7 @@ output-file StaticsTest.out,
 compare-to StaticsTest.cmp,
 output-list RAM[0]%D1.6.1 RAM[261]%D1.6.1 RAM[262]%D1.6.1;

-set RAM[0] 256,
+set RAM[0] 261,

 repeat 2500 {
   ticktock;
```