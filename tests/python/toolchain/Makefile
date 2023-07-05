probe_build:
	rm -rf build 
	mkdir build
	cd build && ../cpython/configure
	$(PROBE) -o $(PROBE_OUTPUT) -- $(MAKE) -C build -j 6

probe_exec:
	$(PROBE) -o $(PROBE_OUTPUT) 
