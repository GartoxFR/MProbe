export WD=$TEST_ROOT/cpp/axwy/

export BUILD_TITLE="Compilation de axwy C++ (clang)"
export EXEC_TITLE="Éxectution de axwy C++ (clang)"

export CLEAN="rm -f axwy"
export BUILD="/data/ewan/clang/out/bin/clang++ -O3 -o axwy axwy.cpp"
export EXEC="./axwy 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
