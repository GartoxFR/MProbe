export WD=$TEST_ROOT/java/tf_idf/

export BUILD_TITLE="Compilation de tf_idf Java"
export EXEC_TITLE="Éxectution de tf_idf Java"

export CLEAN=""
JAVA_HOME=/data/ewan/java/build_stage2/images/jdk
export BUILD="$JAVA_HOME/bin/javac Main.java IdMap.java"
export EXEC="$JAVA_HOME/bin/java Main 2048 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
