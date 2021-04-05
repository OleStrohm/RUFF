TOP_LEVEL = cpu.v
SIM_CPP = tb.cpp

build: $(TOP_LEVEL) $(SIM_CPP)
	verilator -Wall --cc $(TOP_LEVEL) --exe --build $(SIM_CPP)

test: build
	./obj_dir/Vcpu

clean:
	rm -r ./obj_dir/

.PHONY: build clean all
