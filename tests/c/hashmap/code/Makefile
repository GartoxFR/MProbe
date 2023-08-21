# Code compiling variables
ECHO = echo
GCC ?= gcc
CCFLAGS = -O3 -pedantic -Wall -std=c99
LDFLAGS = -O3 -lm
EXE_NAME = hashmap
SOURCEDIR=src
BUILDDIR=build
BINDIR=bin
EXE = $(BINDIR)/$(EXE_NAME)
SOURCES = $(wildcard $(SOURCEDIR)/**/*.c) $(wildcard $(SOURCEDIR)/*.c)
OBJECTS = $(SOURCES:$(SOURCEDIR)/%.c=$(BUILDDIR)/%.o)
DEP=$(OBJECTS:%.o=%.d)
DEPGENFLAGS ?=-MMD

.PHONY = run code uml pdf clean

all: code

code: $(EXE)

run: code
	@./$(EXE)

# Clean up
clean:
	rm -f $(DEP) $(OBJECTS) $(EXE)

# Code rules
$(EXE) : $(OBJECTS)
	@mkdir -p $(@D)
	@$(GCC) -o $(@) $(OBJECTS) $(LDFLAGS)

-include $(DEP)

$(BUILDDIR)/%.o : $(SOURCEDIR)/%.c
	@mkdir -p $(@D)
	$(GCC) $(CCFLAGS) $(DEPGENFLAGS) -c $< -o $@

dir: 
	@mkdir -p $(BUILDDIR)
	@mkdir -p $(BINDIR)
