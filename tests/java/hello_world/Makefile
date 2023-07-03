CC = javac
BIN = HelloWorld.class

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- java HelloWorld

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) HelloWorld.java

$(BIN): HelloWorld.java
	$(CC) $<

