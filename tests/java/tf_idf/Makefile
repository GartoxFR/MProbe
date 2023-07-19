BIN = Main.class IdMap.class

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) --title "Execution de tf_idf Java" -- java  Main war ../../../corpus/gutenberg/*

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de tf_idf Java" -- javac Main.java IdMap.java

$(BIN): Main.java IdMap.java
	javac Main.java IdMap.java

