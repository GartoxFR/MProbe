# Code compiling variables
ECHO = echo
GCC = gcc
CCFLAGS = -O3 -ansi -pedantic -Wall -std=c2x 
LDFLAGS = -O3 -lm
EXE_NAME = tf_idf
SOURCEDIR=src
BUILDDIR=build
BINDIR=bin
EXE = $(BINDIR)/$(EXE_NAME)
SOURCES = $(wildcard $(SOURCEDIR)/**/*.c) $(wildcard $(SOURCEDIR)/*.c)
OBJECTS = $(SOURCES:$(SOURCEDIR)/%.c=$(BUILDDIR)/%.o)
DEP=$(OBJECTS:%.o=%.d)

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
	$(GCC) $(CCFLAGS) -MMD -c $< -o $@

dir: 
	@mkdir -p $(BUILDDIR)
	@mkdir -p $(BINDIR)
