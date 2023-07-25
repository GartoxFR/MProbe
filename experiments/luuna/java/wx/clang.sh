export WD=$TEST_ROOT/java/wx/

export BUILD_TITLE="Compilation de wx Java (OpenJDK built with clang)"
export EXEC_TITLE="Éxectution de wx Java (OpenJDK built with clang)"

export CLEAN=""
JAVA_HOME=/data/ewan/java/build_clang/images/jdk
export BUILD="$JAVA_HOME/bin/javac Wx.java"
export EXEC="$JAVA_HOME/bin/java Wx 1024"

export BUILD_SAMPLE_PERIOD=100
export EXEC_SAMPLE_PERIOD=100
