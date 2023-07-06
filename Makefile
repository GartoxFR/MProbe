O ?= results/last
O_ABS := $(abspath $O)
TESTS_SRC = $(wildcard tests/*/*)
TESTS = $(TESTS_SRC:tests/%=%/build) $(TESTS_SRC:tests/%=%/exec)
EXCLUDED_DEFAULT_TESTS = rust/toolchain/build rust/toolchain/exec c/toolchain/build c/toolchain/exec python/wx/exec \
						 python/toolchain/build python/toolchain/exec \
						 java/toolchain/build java/toolchain/exec

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
PROBE_METHOD ?= rss
PROBE += -m $(PROBE_METHOD)

tools:  $(TOOLS) 

tests: $(filter-out $(EXCLUDED_DEFAULT_TESTS),$(TESTS))

-include $(DEPS)

$(PROC_PROBE_FINAL_BIN): $(PROC_PROBE_INTERMEDIATE_BIN) | $(BIN_FOLDER)
	@cp $< $@

$(PROC_PROBE_INTERMEDIATE_BIN): $(PROC_PROBE_MANIFEST) 
	@$(CARGO) build $(CARGO_FLAGS) --manifest-path=$<

$(PROCESS_SAMPLES_FINAL_BIN): $(PROCESS_SAMPLES_INTERMEDIATE_BIN) | $(BIN_FOLDER)
	@cp $< $@

$(PROCESS_SAMPLES_INTERMEDIATE_BIN): $(PROCESS_SAMPLES_MANIFEST)
	@$(CARGO) build $(CARGO_FLAGS) --manifest-path=$<


$(BIN_FOLDER):
	@mkdir -p $(BIN_FOLDER)
	
clean:
	@rm -rf $(BIN_FOLDER)
	@$(CARGO) clean --manifest-path=$(PROCESS_SAMPLES_MANIFEST)
	@$(CARGO) clean --manifest-path=$(PROC_PROBE_MANIFEST)


%/all: %/build %/exec
	@$(NOOP)

%/exec: $(O_ABS)/%/exec/memory.svg 
	@$(NOOP)

%/build: $(O_ABS)/%/build/memory.svg
	@$(NOOP)

%/memory.svg: %/detail.json $(PROCESS_SAMPLES_FINAL_BIN)
	$(PROCESS_SAMPLES_FINAL_BIN) -q -t memory -o $@ $<

$(O_ABS)/%/build/detail.json: $(PROC_PROBE_FINAL_BIN) FORCE
	mkdir -p $(@D)
	$(MAKE) PROBE="$(PROBE)" PROBE_OUTPUT=$@ -C tests/$* probe_build

$(O_ABS)/%/exec/detail.json: $(PROC_PROBE_FINAL_BIN) FORCE | $(O_ABS)/%/build/detail.json 
	mkdir -p $(@D)
	$(MAKE) PROBE="$(PROBE)" PROBE_OUTPUT=$@ -C tests/$* probe_exec

FORCE:

.PRECIOUS: %/memory.svg %/duration.svg $(O_ABS)/%/build/detail.json $(O_ABS)/%/exec/detail.json
