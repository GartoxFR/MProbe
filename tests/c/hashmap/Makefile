BIN = code/bin/hashmap

.PHONY = probe_exec probe_build

probe_exec: $(BIN)
	$(PROBE) -o $(PROBE_OUTPUT) -s 100 --title "Execution de hashmap C"  -- $(BIN) 100000000

probe_build:
	$(MAKE) -C code clean
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de hashmap C"   -- $(MAKE) -C code

$(BIN):
	$(MAKE) -C code

