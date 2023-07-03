.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- python3 wx.py 1024

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT)

