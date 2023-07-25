export WD=$TEST_ROOT/rust/hello_world/

export BUILD_TITLE="Compilation de hello_world Rust"
export EXEC_TITLE="Éxectution de hello_world Rust"

export CLEAN="cargo clean"
export BUILD="cargo +stage2 build --release"
export EXEC="target/release/hello_world 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
