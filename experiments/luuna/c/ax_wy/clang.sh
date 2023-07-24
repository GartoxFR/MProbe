export WD=$TEST_ROOT/c/ax_wy/

export BUILD_TITLE="Compilation de ax_wy C (clang)"
export EXEC_TITLE="Éxectution de ax_wy C (clang)"

export CLEAN="rm -f ax_wy"
export BUILD="/data/ewan/clang/out/bin/clang -O3 -o ax_wy ax_wy.c"
export EXEC="./ax_wy"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
