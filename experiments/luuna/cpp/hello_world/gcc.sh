export WD=$TEST_ROOT/cpp/hello_world/

export BUILD_TITLE="Compilation de hello_world C++ (gcc)"
export EXEC_TITLE="Éxectution de hello_world C++ (gcc)"

export CLEAN="rm -f hello_world"
export BUILD="/data/ewan/gcc/out/bin/g++ -O3 -o hello_world hello_world.cpp"
export EXEC="./hello_world"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
