probe_build:
	cd rust ; \
	./x clean ; \
	$(PROBE) -o $(PROBE_OUTPUT) -- ./x build library -j 6

probe_exec:
	$(PROBE) -o $(PROBE_OUTPUT) 
