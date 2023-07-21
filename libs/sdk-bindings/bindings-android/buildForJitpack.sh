#!/bin/bash

set -e

if [[ -z "$JITPACK" ]]; then
    exit 1
fi

VERSION=0.0.1

echo "JitPack building version $VERSION."
cd $(dirname $0)

# curl seems to be having issues with the cert of our repo, thus the insecure flag which is not ideal of course.
curl "https://github-registry-files.githubusercontent.com/579645777/77d37a80-2810-11ee-9e26-8075f8bc0a3d?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20230721%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20230721T195338Z&X-Amz-Expires=300&X-Amz-Signature=11fed5aaba053a2a867c3ab08780a2fa400785bebea4306e6105b27caa234ec4&X-Amz-SignedHeaders=host&actor_id=0&key_id=0&repo_id=579645777&response-content-disposition=filename%3Dbindings-android-0.0.1.aar&response-content-type=application%2Foctet-stream" --insecure --output bindings-android-$VERSION.aar
curl "https://github-registry-files.githubusercontent.com/579645777/7dc95b80-2810-11ee-89e7-707b260caf8b?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20230721%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20230721T195338Z&X-Amz-Expires=300&X-Amz-Signature=14ba696c90fdb33a41c1deb0301be11ad318fc50deb97f26d54144667a54a1c6&X-Amz-SignedHeaders=host&actor_id=0&key_id=0&repo_id=579645777&response-content-disposition=filename%3Dbindings-android-0.0.1.module&response-content-type=application%2Foctet-stream" --insecure --output bindings-android-$VERSION.module
curl "https://github-registry-files.githubusercontent.com/579645777/80c44c00-2810-11ee-975f-e30458ec4ed1?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20230721%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20230721T195338Z&X-Amz-Expires=300&X-Amz-Signature=09d4de0a47dec3d770ee68c5fae7fbd784b71348864d764df0e0982919a56dca&X-Amz-SignedHeaders=host&actor_id=0&key_id=0&repo_id=579645777&response-content-disposition=filename%3Dbindings-android-0.0.1-sources.jar&response-content-type=application%2Foctet-stream" --insecure --output bindings-android-$VERSION-sources.jar
curl "https://github-registry-files.githubusercontent.com/579645777/7b670180-2810-11ee-99ae-01ef7a8d2f85?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20230721%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20230721T195338Z&X-Amz-Expires=300&X-Amz-Signature=6ba3385ebf04c01c0781a8bde5f59af1e15de0984912eaac395bbb1a85a0e8fa&X-Amz-SignedHeaders=host&actor_id=0&key_id=0&repo_id=579645777&response-content-disposition=filename%3Dbindings-android-0.0.1.pom&response-content-type=application%2Foctet-stream" --insecure --output bindings-android-$VERSION.pom

cat /home/jitpack/build/libs/sdk-bindings/bindings-android/bindings-android-$VERSION.pom


mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-$VERSION.aar -DpomFile=bindings-android-$VERSION.pom
mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-$VERSION.module -DpomFile=bindings-android-$VERSION.pom
mvn org.apache.maven.plugins:maven-install-plugin:3.1.1:install-file -Dfile=bindings-android-$VERSION-sources.jar -DpomFile=bindings-android-$VERSION.pom -Dclassifier=sources

ls /home/jitpack/.m2/repository/breez_sdk/bindings-android
echo "---"
ls /home/jitpack/.m2/repository/breez_sdk/bindings-android/$VERSION/
cat /home/jitpack/.m2/repository/breez_sdk/bindings-android/$VERSION/bindings-android-$VERSION.module
