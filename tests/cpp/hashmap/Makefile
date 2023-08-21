CC = g++
CC_FLAGS = -O3
BIN = build/hashmap

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(BIN) 1000000

probe_build: 
	mkdir -p build
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) $(CC_FLAGS) -o $(BIN) hashmap.cpp

$(BIN): hashmap.cpp
	mkdir -p build
	$(CC) $(CC_FLAGS) -o $@ $<

