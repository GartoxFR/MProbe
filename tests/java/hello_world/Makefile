CC = javac
BIN = Main.class

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- java Main

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) Main.java

$(BIN): Main.java
	$(CC) $<

