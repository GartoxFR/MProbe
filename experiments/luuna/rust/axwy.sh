export WD=$TEST_ROOT/rust/axwy/

export BUILD_TITLE="Compilation de axwy Rust"
export EXEC_TITLE="Éxectution de axwy Rust"

export CLEAN="cargo clean"
export BUILD="cargo +stage2 build --release"
export EXEC="target/release/axwy 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
