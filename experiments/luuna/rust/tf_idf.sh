export WD=$TEST_ROOT/rust/tf_idf/

export BUILD_TITLE="Compilation de tf_idf Rust"
export EXEC_TITLE="Éxectution de tf_idf Rust"

export CLEAN="cargo clean"
export BUILD="cargo +stage2 build --release"
export EXEC="target/release/tf_idf 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
