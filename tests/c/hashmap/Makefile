BIN = code/bin/hashmap

.PHONY = probe_exec probe_build

probe_exec: $(BIN)
	$(PROBE) -o $(PROBE_OUTPUT) --title "Execution de hashmap C"  -- $(BIN) 1000000

probe_build:
	$(MAKE) -C code clean
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de hashmap C"   -- $(MAKE) -C code

$(BIN):
	$(MAKE) -C code

