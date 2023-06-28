O ?= results/last
O := $(abspath $O)
TESTS_SRC = $(wildcard tests/*/*)
TESTS = $(TESTS_SRC:tests/%=%/build) $(TESTS_SRC:tests/%=%/exec)


.PHONY = tools clean tests $(TESTS)

BIN_FOLDER = bin

CARGO = cargo
CARGO_FLAGS = --release

RUST_BUILD_PATH = target/release

PROC_PROBE_FINAL_BIN = $(BIN_FOLDER)/proc_probe
PROC_PROBE_DIR = tools/proc_probe
PROC_PROBE_MANIFEST = $(PROC_PROBE_DIR)/Cargo.toml
PROC_PROBE_INTERMEDIATE_BIN = $(abspath $(PROC_PROBE_DIR)/$(RUST_BUILD_PATH)/proc_probe)
PROC_PROBE_DEPS = $(PROC_PROBE_DIR)/$(RUST_BUILD_PATH)/proc_probe.d

TOOLS += $(PROC_PROBE_FINAL_BIN)
DEPS += $(PROC_PROBE_DEPS)

PROCESS_SAMPLES_FINAL_BIN = $(BIN_FOLDER)/process_samples
PROCESS_SAMPLES_DIR = tools/process_samples
PROCESS_SAMPLES_MANIFEST = $(PROCESS_SAMPLES_DIR)/Cargo.toml
PROCESS_SAMPLES_INTERMEDIATE_BIN = $(abspath $(PROCESS_SAMPLES_DIR)/$(RUST_BUILD_PATH)/process_samples)
PROCESS_SAMPLES_DEPS = $(PROCESS_SAMPLES_DIR)/$(RUST_BUILD_PATH)/process_samples.d

TOOLS += $(PROCESS_SAMPLES_FINAL_BIN)
DEPS += $(PROCESS_SAMPLES_DEPS)

PROBE := $(abspath $(PROC_PROBE_FINAL_BIN))

tools: | $(BIN_FOLDER) $(TOOLS) 

tests: $(TESTS)

-include $(DEPS)

$(PROC_PROBE_FINAL_BIN): $(PROC_PROBE_INTERMEDIATE_BIN) 
	@cp $< $@

$(PROC_PROBE_INTERMEDIATE_BIN): $(PROC_PROBE_MANIFEST) 
	@$(CARGO) build $(CARGO_FLAGS) --manifest-path=$<

$(PROCESS_SAMPLES_FINAL_BIN): $(PROCESS_SAMPLES_INTERMEDIATE_BIN) 
	@cp $< $@

$(PROCESS_SAMPLES_INTERMEDIATE_BIN): $(PROCESS_SAMPLES_MANIFEST)
	@$(CARGO) build $(CARGO_FLAGS) --manifest-path=$<


$(BIN_FOLDER):
	@mkdir -p $(BIN_FOLDER)
	
clean:
	@rm -rf $(BIN_FOLDER)
	@$(CARGO) clean --manifest-path=$(PROCESS_SAMPLES_MANIFEST)
	@$(CARGO) clean --manifest-path=$(PROC_PROBE_MANIFEST)


%/exec: $(O)/%/exec/memory.svg $(O)/%/exec/duration.svg
	@$(NOOP)

%/build: $(O)/%/build/memory.svg $(O)/%/build/duration.svg
	@$(NOOP)

%/memory.svg: %/detail.json
	$(PROCESS_SAMPLES_FINAL_BIN) -q -t memory -o $@ $<

%/duration.svg: %/detail.json
	$(PROCESS_SAMPLES_FINAL_BIN) -q -t duration -o $@ $<

$(O)/%/build/detail.json: FORCE
	mkdir -p $(@D)
	$(MAKE) PROBE=$(PROBE) PROBE_OUTPUT=$@ -C tests/$* probe_build

$(O)/%/exec/detail.json: FORCE | $(O)/%/build/detail.json 
	mkdir -p $(@D)
	$(MAKE) PROBE=$(PROBE) PROBE_OUTPUT=$@ -C tests/$* probe_exec

FORCE:

.PRECIOUS: %/memory.svg %/duration.svg $(O)/%/build/detail.json $(O)/%/exec/detail.json
