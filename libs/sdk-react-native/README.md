# React Native Breez SDK

The Breez SDK enables mobile developers to integrate Lightning and bitcoin payments into their apps with a very shallow learning curve. The use cases are endless – from social apps that want to integrate tipping between users to content-creation apps interested in adding bitcoin monetization. Crucially, this SDK is an end-to-end, non-custodial, drop-in solution powered by Greenlight, a built-in LSP, on-chain interoperability, third-party fiat on-ramps, and other services users and operators need.
   
The Breez SDK provides the following services:
* Sending payments (via various protocols such as: bolt11, keysend, lnurl-pay, lightning address, etc.)
* Receiving payments (via various protocols such as: bolt11, lnurl-withdraw, etc.)
* Fetching node status (e.g. balance, max allow to pay, max allow to receive, on-chain balance, etc.)
* Connecting to a new or existing node.

## Installation

```console
$ npm install @breeztech/react-native-breez-sdk
```
or

```console
$ yarn add @breeztech/react-native-breez-sdk
```

### Important fix for React Native versions below 0.71.0

If your project uses a React Native version less < 0.71.0, and you want to build your app for Android, you might run into an error like this:

```
 2 files found with path 'lib/arm64-v8a/libc++_shared.so' from inputs:
      - /(...)/.gradle/caches/transforms-3/c476ede63d070b991438fe0d1c323931/transformed/jetified-react-native-0.68.6/jni/arm64-v8a/libc++_shared.so
      - /(...)/.gradle/caches/transforms-3/7c318ac8dd87c1f0c7540616d6d47bd8/transformed/jetified-breez-sdk-0.1.3/jni/arm64-v8a/libc++_shared.so
```

To fix this you need to disambiguate which file to use by adding the following snippet to your app's `android/app/build.gradle`:

``` gradle
android {
    // ...
    packagingOptions {
        pickFirst 'lib/x86/libc++_shared.so'
        pickFirst 'lib/x86_64/libc++_shared.so'
        pickFirst 'lib/armeabi-v7a/libc++_shared.so'
        pickFirst 'lib/arm64-v8a/libc++_shared.so'
    }
}
```

Both the Breez SDK as well as React Native package the `libc++_shared.so` native library.
React Native versions below 0.71.0 have a [bug](https://github.com/facebook/react-native/issues/30297) where they cannot automatically handle multiple versions of this file.
This has been [fixed](https://github.com/facebook/react-native/pull/35093) in React Native 0.71.0 and thus the above snippet only needs to be added to projects using React Native < 0.71.0.

## Usage

For a more in-depth look at using the Breez SDK, you can follow [these examples](https://sdk-doc.breez.technology/) in the SDK overview.

Please contact [support@breez.technology](mailto:support@breez.technology?subject=Breez%20API%20Key) to request a Breez API Key. 
```ts
import React, { useEffect } from "react"
import { 
    defaultConfig,
    EnvironmentType,    
    sendPayment,
    connect 
} from "@breeztech/react-native-breez-sdk";
import BuildConfig from "react-native-build-config"

const App = () => (
    ...

    const payInvoice = async (bolt11: string, sats?: number) => {
        // Pay invoice
        const payment = await sendPayment(bolt11, sats)
    }

    useEffect(() => {
        const asyncFn = async () => {
            
            // create the specific node configuration
            const nodeConfig = {
                type: "greenlight",
                config: {
                    partnerCredentials: {
                        deviceKey: null,
                        deviceCert: null
                    }
                }
            }
            
            // Construct the sdk default config
            const config = await defaultConfig(EnvironmentType.PRODUCTION, BuildConfig.BREEZ_API_KEY, nodeConfig)            

             // Connect to the Breez SDK make it ready to use
            await connect(config, seed)            
        }

        asyncFn()
    }, [])

    ...
)

export default App
```

## Example

In the `example` folder of the [Breez SDK repository](https://github.com/breez/breez-sdk/tree/main/libs/sdk-react-native/example) you will find a basic application for using Breez SDK. Change directory into the folder and install the dependencies:
```console
$ yarn
```
Then to run on android:
```console
$ yarn android
```
or for iOS:
```console
$ yarn pods && yarn ios
```
