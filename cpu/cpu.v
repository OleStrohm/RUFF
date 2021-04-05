module cpu (
    input clk,
    output var [31:0] instr_addr
);

    initial instr_addr = 0;

    always_ff @ (posedge clk)
        instr_addr <= instr_addr + 1;

endmodule

