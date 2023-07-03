.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- python3 hello_world.py

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT)

