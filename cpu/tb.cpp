#include <bits/stdint-uintn.h>
#include <stdio.h>
#include <stdint.h>
#include "Vcpu.h"
#include "verilated.h"

void step(Vcpu* top, uint64_t &time);

int main(int argc, char** argv, char** env) {
    Verilated::commandArgs(argc, argv);
    Vcpu* top = new Vcpu;
    uint64_t time = 0;
    top->clk = 0;

    while (!Verilated::gotFinish() && time < 10) {
        step(top, time);
        printf("instr_addr: %08X\n", top->instr_addr);
    }

    delete top;
    return 0;
}

void step(Vcpu* top, uint64_t &time) {
    top->clk = 0;
    top->eval();
    top->clk = 1;
    top->eval();
    time++;
}
