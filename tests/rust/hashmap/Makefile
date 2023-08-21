BIN := $(abspath target/release/hashmap)

.PHONY = probe_exec probe_build

-include target/release/hashmap.d

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) --title "Execution de hashmap Rust"  -- $(BIN) 1000000

probe_build: 
	@cargo clean
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de hashmap Rust" -- cargo build --release -j 1

$(BIN): Cargo.toml
	cargo build --release

	

