export WD=$TEST_ROOT/cpp/tf_idf/code

export BUILD_TITLE="Compilation de tf_idf C++ (gcc)"
export EXEC_TITLE="Éxectution de tf_idf C++ (gcc)"

export CLEAN="make clean"
export BUILD="make GCC=/data/ewan/gcc/out/bin/g++"
export EXEC="LD_LIBRARY_PATH=/data/ewan/gcc/out/lib64:$LD_LIBRARY_PATH bin/tf_idf 'war' /data/ewan/corpus/gutenberg/*"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
