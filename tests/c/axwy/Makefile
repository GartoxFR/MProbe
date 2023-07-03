CC = gcc
CC_FLAGS = -O3
BIN = build/axwy

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(BIN) 2048 1024

probe_build: 
	mkdir -p build
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) $(CC_FLAGS) -o $(BIN) axwy.c

$(BIN): axwy.c
	mkdir -p build
	$(CC) $(CC_FLAGS) -o $@ $<

