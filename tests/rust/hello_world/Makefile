BIN := $(abspath target/release/hello_world)

.PHONY = probe_exec probe_build

-include target/release/hello_world.d

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) $(BIN)

probe_build: 
	@cargo clean
	$(PROBE) -o $(PROBE_OUTPUT) -- cargo build --release

$(BIN): Cargo.toml
	cargo build --release

