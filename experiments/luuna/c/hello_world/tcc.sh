export WD=$TEST_ROOT/c/hello_world/

export BUILD_TITLE="Compilation de hello_world C (gcc)"
export EXEC_TITLE="Éxectution de hello_world C (gcc)"

export CLEAN="rm -f hello_world"
export BUILD="/data/ewan/tcc/out/bin/tcc -O3 -o hello_world hello_world.c"
export EXEC="./hello_world"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
