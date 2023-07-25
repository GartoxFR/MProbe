export WD=$TEST_ROOT/java/hello_world/

export BUILD_TITLE="Compilation de hello_world Java (OpenJDK built with gcc)"
export EXEC_TITLE="Éxectution de hello_world Java (OpenJDK built with gcc)"

export CLEAN=""
JAVA_HOME=/data/ewan/java/build_gcc/images/jdk
export BUILD="$JAVA_HOME/bin/javac HelloWorld.java"
export EXEC="$JAVA_HOME/bin/java HelloWorld 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
