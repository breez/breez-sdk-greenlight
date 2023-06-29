#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi

echo "JitPack building version $VERSION."
cd $(dirname $0)
cd ..
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
make init
make bindings-android
cd bindings-android
./gradlew publishToMavenLocal -PlibraryVersion=$VERSION
