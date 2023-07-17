BIN := $(abspath target/release/tf_idf)

.PHONY = probe_exec probe_build

-include target/release/tf_idf.d

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) --title "Execution de tf_idf Rust"  -- $(BIN) "opengl texture" ../../../corpus/opengl/*

probe_build: 
	@cargo clean
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de tf_idf Rust" -- cargo build --release -j 1

$(BIN): Cargo.toml
	cargo build --release

	

