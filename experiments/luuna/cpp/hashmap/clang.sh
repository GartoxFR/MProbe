export WD=$TEST_ROOT/cpp/hashmap/

export BUILD_TITLE="Compilation de hashmap C++ (clang)"
export EXEC_TITLE="Éxectution de hashmap C++ (clang)"

export CLEAN="rm -f hashmap"
export BUILD="/data/ewan/clang/out/bin/clang++ -O3 -o hashmap hashmap.cpp"
export EXEC="./hashmap 1000000"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
