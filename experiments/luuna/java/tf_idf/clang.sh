export WD=$TEST_ROOT/java/tf_idf/

export BUILD_TITLE="Compilation de tf_idf Java (OpenJDK built with clang)"
export EXEC_TITLE="Éxectution de tf_idf Java (OpenJDK built with clang)"

export CLEAN=""
JAVA_HOME=/data/ewan/java/build_clang/images/jdk
export BUILD="$JAVA_HOME/bin/javac Main.java IdMap.java"
export EXEC="$JAVA_HOME/bin/java Main 'war' /data/ewan/corpus/gutenberg/*"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
