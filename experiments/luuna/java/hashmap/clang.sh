export WD=$TEST_ROOT/java/hashmap/

export BUILD_TITLE="Compilation de hashmap Java (OpenJDK built with clang)"
export EXEC_TITLE="Éxectution de hashmap Java (OpenJDK built with clang)"

export CLEAN=""
JAVA_HOME=/data/ewan/java/build_clang/images/jdk
export BUILD="$JAVA_HOME/bin/javac Hashmap.java"
export EXEC="$JAVA_HOME/bin/java Hashmap 1000000"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
