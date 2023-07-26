BIN := $(abspath target/release/allocations)

.PHONY = probe_exec probe_build

-include target/release/allocations.d

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) $(BIN) 2048 1024

probe_build: 
	@cargo clean
	$(PROBE) -o $(PROBE_OUTPUT) -- cargo build --release

$(BIN): Cargo.toml
	cargo build --release

	

