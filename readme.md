# RUFF - Relatively Understandable Flip-flops

DISCLAIMER: Almost all the features mentioned in this entire repository is features I expect to complete, not implemented features.

The RUFF cpu and assembly language will be my way of learning more about how to
create cpus in HDL, how to design an ISA and assembly language.

## CPU

The RUFF processor is a 32-bit cpu written in System Verilog, and aims to be a very
simple cpu that is easily understood, but also has enough features to be
able to run actual programs.

Future features include:

* USB serial communication
* SDRAM storage instead of on-chip memory

Read more at [./docs/cpu.md](./docs/cpu.md)

## ISA

The RUFF ISA is a RISC instruction set, with each instruction being exactly
32 bits wide. It is roughly based on instruction sets like ARM,
but aims to be far simpler

Future features include:

* Assembler
* LLVM Backend

## Resources

Most of the knowledge used to create and design I will have gathered from various
places, here are links to the ones I've found helpful in many different ways:

[_But how do it know?_](http://buthowdoitknow.com/)

* A book about how computers work, all the way from NAND gates to a full cpu.
Very simple and enjoyable read!

[Ben Eater](https://www.youtube.com/channel/UCS0N5baNlQWJCUrhCEo8WlA)

* Youtuber who makes videos about a variety of eletronic/computer engineering topics,
including a custom breadboard computer and the 6502.
