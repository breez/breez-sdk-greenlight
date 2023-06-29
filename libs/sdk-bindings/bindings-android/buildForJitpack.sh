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
yes | $ANDROID_HOME/tools/bin/sdkmanager --install "ndk;23.1.7779620"
ls $ANDROID_HOME
echo "---"
ls $ANDROID_HOME/ndk
echo "---"
ls $ANDROID_HOME/ndk-bundle
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle
make init
make bindings-android
cd bindings-android
./gradlew publishToMavenLocal -PlibraryVersion=$VERSION
