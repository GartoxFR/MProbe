#!/usr/bin/env bash

if [ -z $TEST_ROOT ]; then
    TEST_ROOT=/home/ewan/work/tests
fi

if [ -z $OUTPUT ]; then
    OUTPUT=/home/ewan/work/results/exp
fi

if [ -z $1 ]; then 
    echo "./exp.sh <expe_dir>"
    exit 1
fi

mkdir -p OUTPUT

for file in $(find $1 -type f)
do
    DIRECTORY=${file%%.*}
    OUTPUT_DIRECTORY=$OUTPUT/${DIRECTORY#$1}
    mkdir -p $OUTPUT_DIRECTORY

    source $file
    pushd $WD
    $CLEAN
    echo Build: $BUILD
    mprobe -s $BUILD_SAMPLE_PERIOD --title "$BUILD_TITLE" -o $OUTPUT_DIRECTORY/build.json -- $BUILD
    echo Exec: $EXEC
    mprobe -s $EXEC_SAMPLE_PERIOD --title "$EXEC_TITLE" -o $OUTPUT_DIRECTORY/exec.json -- $EXEC
    popd

    unset WD 
    unset CLEAN
    unset BUILD
    unset EXEC
    unset BUILD_TITLE
    unset EXEC_TITLE
    unset BUILD_SAMPLE_PERIOD
    unset EXEC_SAMPLE_PERIOD

done
