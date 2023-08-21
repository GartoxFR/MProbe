export WD=$TEST_ROOT/rust/hashmap/

export BUILD_TITLE="Compilation de hashmap Rust"
export EXEC_TITLE="Éxectution de hashmap Rust"

export CLEAN="cargo clean"
export BUILD="cargo +stage2 build --release -j 1"
export EXEC="target/release/hashmap 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
