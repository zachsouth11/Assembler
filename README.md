# pa1-template2
Template for PA1 for CS4100 Spring 2022

This is an assembler I made for a compilers class.

This project is an assembler for Grumpy assembly code. The idea is to take instructions/labels from input and parse the input with a cooresponding Instruction Ste Architecture. Afer the parsing is done and if all the input was valid, tthe instructions get translated to byte code. There is a program assem that reads assembly code for the Grumpy virtual machine (GrumpyVM) and outputs corresponding bytecode.
This program takes the input from <filename.s> files in the tests directory and the program outputs the instructions as bytecode in to a <filename.o> file

to compile code run cargo test in terminal
to run testcases run ./test.sh