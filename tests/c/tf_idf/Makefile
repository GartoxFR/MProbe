BIN = code/bin/tf_idf

.PHONY = probe_exec probe_build

probe_exec: $(BIN)
	$(PROBE) -o $(PROBE_OUTPUT) --title "Execution de tf_idf C"  -- $(BIN) "war" ../../../corpus/gutenberg/*

probe_build:
	$(MAKE) -C code clean
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de tf_idf C"   -- $(MAKE) -C code

$(BIN):
	$(MAKE) -C code

