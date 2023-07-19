.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) --title "Execution de tf_idf Python" -- python3 tf_idf.py "war" ../../../corpus/gutenberg/*

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT) --title "Compilation de tf_idf Python"

