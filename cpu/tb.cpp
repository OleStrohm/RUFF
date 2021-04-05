#include "Vcpu.h"
#include "Vcpu_cpu.h"
#include "verilated.h"
#include <fstream>
#include <ios>
#include <stdint.h>
#include <stdio.h>
#include <type_traits>

void step(Vcpu *top, uint64_t &time);
#define INIT_ROM(rom) init_rom<std::extent<decltype(rom)>::value>(rom);
#define QWORD(loc)                                                             \
  ((cpu->onchip_mem[loc + 3] << 24) | (cpu->onchip_mem[loc + 2] << 16) |       \
   (cpu->onchip_mem[loc + 1] << 8) | (cpu->onchip_mem[loc]))

template <unsigned int N> void init_rom(CData mem[N]) {
  std::ifstream rom("rom.bin", std::ios::in | std::ios::binary);

  char buffer[N];
  for (int i = 0; i < N; i++) {
    buffer[i] = 0xFF;
  }

  if (rom.is_open()) {
    rom.seekg(0, std::ios::beg);
    rom.read(buffer, N);
  }

  for (int i = 0; i < N; i++) {
    mem[i] = buffer[i];
  }

  rom.close();
}

int main(int argc, char **argv, char **env) {
  Verilated::commandArgs(argc, argv);
  auto top = new Vcpu;
  auto cpu = top->cpu;
  uint64_t time = 0;
  top->clk = 0;

  INIT_ROM(cpu->onchip_mem);

  while (!Verilated::gotFinish() && time < 10) {
    printf("rip: %08X\tinstr:%08X\n", cpu->instr_addr, QWORD(cpu->instr_addr));
    step(top, time);
  }

  delete top;
  return 0;
}

void step(Vcpu *top, uint64_t &time) {
  top->clk = 0;
  top->eval();
  top->clk = 1;
  top->eval();
  time++;
}
