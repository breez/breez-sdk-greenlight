#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi

echo "JitPack building version $VERSION."
cd $(dirname $0)

ls
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/0.1.0/bindings-android-0.1.0.aar --output bindings-android-0.1.0.aar
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/0.1.0/bindings-android-0.1.0.pom --output bindings-android-0.1.0.pom
ls

echo "---"

ls ~/.m2/repository

mvn install:install-file -Dfile=bindings-android-0.1.0.aar -DpomFile=indings-android-0.1.0.pom

ls ~/.m2/repository

# cd ..
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# source "$HOME/.cargo/env"
# echo "ls ANDROID_HOME"
# ls $ANDROID_HOME
# echo "---"
# # yes | $ANDROID_HOME/tools/bin/sdkmanager --install "ndk;23.1.7779620"
# # echo "ls ANDROID_HOME/ndk"
# # ls $ANDROID_HOME/ndk
# # echo "---"
# echo "ls ANDROID_HOME/ndk"
# ls $ANDROID_HOME/ndk
# echo "---"
# export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/23.1.7779620
# make init
# make bindings-android
# cd bindings-android
# ./gradlew publishToMavenLocal -PlibraryVersion=$VERSION
