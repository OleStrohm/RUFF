module cpu (
    input var clk
);
    bit [7:0] onchip_mem [0:32'h1000 - 1] /*verilator public*/;
    logic [31:0] instr_addr /*verilator public*/ = 0; 

    always_ff @ (posedge clk)
        instr_addr <= instr_addr + 4;

endmodule

