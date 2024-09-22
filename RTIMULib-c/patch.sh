#!/bin/sh

DIR=$(dirname "$0")

cp $DIR/RTIMULib-c.h RTIMULib/RTIMULib
cp $DIR/RTIMULib-c.cpp RTIMULib/RTIMULib
git apply $DIR/RTIMULib-c.patch --directory RTIMULib
