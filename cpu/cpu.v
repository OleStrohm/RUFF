module cpu (
    input var clk
);
    bit [7:0] onchip_mem [0:32'h1000 - 1] /*verilator public*/;

    bit [31:0] regs[0:15] /*verilator public*/;

    /* verilator lint_off UNUSED */
    bit [31:0] pc /*verilator public*/;
    bit [31:0] instr /*verilator public*/;
    bit [15:0] prefix16;
    bit [3:0] regSel0 /*verilator public*/;
    bit [3:0] regSel1 /*verilator public*/;
    bit [31:0] extImm8;
    bit [3:0] shift;
    bit [63:0] not_imm8;
    bit [31:0] imm8;

    assign pc = regs[15];
    assign instr = {
        onchip_mem[pc+3],
        onchip_mem[pc+2],
        onchip_mem[pc+1],
        onchip_mem[pc]
    };
    assign prefix16 = instr[31:16];
    assign regSel0 = instr[15:12];
    assign regSel1 = instr[11:8];
    assign extImm8 = { 24'b0, instr[11:4] };
    assign shift = instr[3:0];
    assign not_imm8 = {extImm8, extImm8} >> (32 - 2 * shift);
    assign imm8 = not_imm8[31:0];
    /* verilator lint_on UNUSED */

    always_ff @ (posedge clk) begin
        regs[15] <= regs[15] + 4;
        if (prefix16 == 0) begin
            regs[regSel0] <= imm8;
        end else begin
            regs[regSel0] <= regs[regSel1];
        end

    end


    initial begin
        regs[0] = 32'h0;
        regs[1] = 32'h0;
        regs[2] = 32'h0;
        regs[3] = 32'h0;
        regs[4] = 32'h0;
        regs[5] = 32'h0;
        regs[6] = 32'h0;
        regs[7] = 32'h0;
        regs[8] = 32'h0;
        regs[9] = 32'h0;
        regs[10] = 32'h0;
        regs[11] = 32'h0;
        regs[12] = 32'h0;
        regs[13] = 32'h0;
        regs[14] = 32'h0;
        regs[15] = 32'h0;
    end

endmodule

