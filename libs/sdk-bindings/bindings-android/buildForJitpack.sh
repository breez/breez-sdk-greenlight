#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi

echo "JitPack building version $VERSION."
cd $(dirname $0)

curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/$VERSION/bindings-android-$VERSION.aar --output bindings-android-$VERSION.aar
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/$VERSION/bindings-android-$VERSION.module --output bindings-android-$VERSION.module
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/$VERSION/bindings-android-$VERSION-sources.jar --output bindings-android-$VERSION-sources.jar
curl https://mvn.breez.technology/releases/breez_sdk/bindings-android/$VERSION/bindings-android-$VERSION.pom --output bindings-android-$VERSION.pom

mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-$VERSION.aar -DpomFile=bindings-android-$VERSION.pom
mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-$VERSION.module -DpomFile=bindings-android-$VERSION.pom
mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-$VERSION-sources.jar -DpomFile=bindings-android-$VERSION.pom -Dclassifier=sources

ls /home/jitpack/.m2/repository/breez_sdk/bindings-android
echo "---"
ls /home/jitpack/.m2/repository/breez_sdk/bindings-android/$VERSION/
