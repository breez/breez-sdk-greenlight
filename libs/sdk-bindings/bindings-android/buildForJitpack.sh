#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi

echo "JitPack building version $VERSION."
cd $(dirname $0)
cd ..
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle
make init
make bindings-android
cd bindings-android
./gradlew publishToMavenLocal -PlibraryVersion=$VERSION
