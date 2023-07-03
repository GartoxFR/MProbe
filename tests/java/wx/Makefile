CC = javac
BIN = Wx.class

.PHONY = probe_exec probe_build

probe_exec: $(BIN) 
	$(PROBE) -o $(PROBE_OUTPUT) -- java  Wx 1000

probe_build: 
	$(PROBE) -o $(PROBE_OUTPUT) -- $(CC) Wx.java

$(BIN): Wx.java
	$(CC) $<

