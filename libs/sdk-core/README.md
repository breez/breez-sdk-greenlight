# sdk-core
The core rust library.

## Build
The library can be built for android & ios. For android it is built in all 4 architectures and for iOS the output is a universal static library.

### Prerequisites:
* set the ANDROID_NDK_HOME env variable to your sdk home folder
* install cargo-ndk to provide simplified android build: ```cargo install cargo-ndk```

### Run the build:
On first usage you will need to run:
```
cd sdk-core
make init
```
Then for iOS:
```
make ios
```
And for android
```
make android
```
All artifacts are written to the libs/target directory.

## Test
`cargo test`
