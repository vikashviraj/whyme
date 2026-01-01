#!/bin/bash
export CXXFLAGS="-I/Library/Developer/CommandLineTools/SDKs/MacOSX26.2.sdk/usr/include/c++/v1"
export MACOSX_DEPLOYMENT_TARGET=15.0
cargo "$@"


