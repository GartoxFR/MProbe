.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- python3 hashmap.py 1000000

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT)

