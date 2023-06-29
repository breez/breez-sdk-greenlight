#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi

echo "JitPack building version $VERSION."
cd $(dirname $0)

curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/0.1.0/bindings-android-0.1.0.aar --insecure --output bindings-android-0.1.0.aar
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/0.1.0/bindings-android-0.1.0.module --insecure --output bindings-android-0.1.0.module
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/0.1.0/bindings-android-0.1.0-sources.jar --insecure --output bindings-android-0.1.0-sources.jar
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/0.1.0/bindings-android-0.1.0.pom --insecure --output bindings-android-0.1.0.pom

mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-0.1.0.aar -DpomFile=bindings-android-0.1.0.pom
mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-0.1.0.module -DpomFile=bindings-android-0.1.0.pom
mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-0.1.0-sources.jar -DpomFile=bindings-android-0.1.0.pom

ls /home/jitpack/.m2/repository/breez_sdk/bindings-android/0.1.0/
