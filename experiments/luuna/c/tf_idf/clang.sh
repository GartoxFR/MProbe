export WD=$TEST_ROOT/c/tf_idf/code

export BUILD_TITLE="Compilation de tf_idf C (clang)"
export EXEC_TITLE="Éxectution de tf_idf C (clang)"

export CLEAN="make clean"
export BUILD="make GCC=/data/ewan/clang/out/bin/clang"
export EXEC="bin/tf_idf 'war' /data/ewan/corpus/gutenberg/*"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
