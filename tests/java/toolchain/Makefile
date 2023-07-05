probe_build:
	rm -rf build 
	mkdir build
	cd build && bash ../jdk/configure
	$(PROBE) -o $(PROBE_OUTPUT) -- $(MAKE) -C build JOBS=6

probe_exec:
	$(PROBE) -o $(PROBE_OUTPUT) 
