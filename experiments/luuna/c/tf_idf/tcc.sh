export WD=$TEST_ROOT/c/tf_idf/code

export BUILD_TITLE="Compilation de tf_idf C (tcc)"
export EXEC_TITLE="Éxectution de tf_idf C (tcc)"

export CLEAN="make clean"
export BUILD="make GCC=/data/ewan/tcc/out/bin/tcc DEPGENFLAGS="
export EXEC="bin/tf_idf 'war' /data/ewan/corpus/gutenberg/*"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
