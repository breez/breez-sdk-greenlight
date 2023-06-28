#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi


echo "JitPack building version $VERSION."
cd $(dirname $0)
./gradlew publishToMavenLocal -PlibraryVersion=$VERSION
