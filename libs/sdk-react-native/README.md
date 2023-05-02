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

## Usage
For a more in-depth look at using the Breez SDK, you can follow [these examples](https://sdk-doc.breez.technology/) in the SDK overview.

Please contact [support@breez.technology](mailto:support@breez.technology?subject=Breez%20API%20Key) to request a Breez API Key. 
```ts
import React, { useEffect } from "react"
import { 
    defaultConfig,
    EnvironmentType,
    initServices,
    sendPayment,
    start 
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
            // Get default config
            const config = await defaultConfig(EnvironmentType.PRODUCTION)
            // Set the apiKey from the gradle or xcode build
            config.apiKey = BuildConfig.BREEZ_API_KEY

             // Initialize Breez SDK
            await initServices(config, deviceKey, deviceCert, seed)
            await start()
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
