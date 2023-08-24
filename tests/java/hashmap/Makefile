CC = javac
BIN = Hashmap.class

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- java  Hashmap 100000000

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) Hashmap.java

$(BIN): Hashmap.java
	$(CC) $<

