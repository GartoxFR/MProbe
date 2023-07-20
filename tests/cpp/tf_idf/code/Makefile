ECHO = echo
GCC ?= clang++
CCFLAGS = -O3 -Wall -Werror -std=c++20
LDFLAGS = 
EXE = tf_idf
SOURCEDIR=src
BUILDDIR=build
BINDIR=bin
SOURCES = $(wildcard $(SOURCEDIR)/*.cpp)
OBJECTS = $(SOURCES:$(SOURCEDIR)/%.cpp=$(BUILDDIR)/%.o)
DEP=$(OBJECTS:%.o=%.d)
# OBJECTS = demo1.o

.PHONY = all run

all: $(EXE)

$(EXE) : $(OBJECTS)
	@mkdir -p $(BINDIR)
	@$(GCC) -o $(BINDIR)/$(EXE) $(OBJECTS) $(LDFLAGS)

-include $(DEP)

$(BUILDDIR)/%.o : $(SOURCEDIR)/%.cpp
	@mkdir -p $(BUILDDIR)
	$(GCC) $(CCFLAGS) -MMD -c $< -o $@

clean:
	@rm -f $(DEP) $(OBJECTS) $(BINDIR)/$(EXE)

