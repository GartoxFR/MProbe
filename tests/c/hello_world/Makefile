CC = gcc
CC_FLAGS = -O3
BIN = build/hello_world

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(BIN)

probe_build: 
	mkdir -p build
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) $(CC_FLAGS) -o $(BIN) hello_world.c

$(BIN): hello_world.c
	mkdir -p build
	$(CC) $(CC_FLAGS) -o $@ $<

