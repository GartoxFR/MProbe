export WD=$TEST_ROOT/c/axwy/

export BUILD_TITLE="Compilation de axwy C (gcc)"
export EXEC_TITLE="Éxectution de axwy C (gcc)"

export CLEAN="rm -f axwy"
export BUILD="/data/ewan/tcc/out/bin/tcc -O3 -o axwy axwy.c"
export EXEC="./axwy"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
