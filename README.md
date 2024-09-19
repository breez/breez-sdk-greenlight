# **Breez SDK - Native *(Greenlight Implementation)***

## **Overview**

## **What Is the Breez SDK?**

The Breez SDK provides developers with a end-to-end solution for integrating self-custodial Lightning payments into their apps and services. It eliminates the need for third parties, simplifies the complexities of Bitcoin and Lightning, and enables seamless onboarding for billions of users to the future of peer-to-peer payments.

To provide the best experience for their end-users, developers can choose between the following implementations:

- [Breez SDK - Native *(Greenlight Implementation)*](https://sdk-doc.breez.technology/)
- [Breez SDK - Nodeless *(Liquid Implementation)*](https://sdk-doc-liquid.breez.technology/)

**The Breez SDK is free for developers.** 

Learn more about the Breez SDK — download our one pager [here](https://drive.google.com/file/d/1TDspNJOvrX_lZUxipeBzitPIWXIdSsLy/view?usp=sharing).

## **What Is the Breez SDK - Native *(Greenlight Implementation)*?**

It's a cloud-based Lightning integration that offers a self-custodial, end-to-end solution for integrating Lightning payments, utilizing nodes-on-demand provided by Blockstream’s Greenlight, with built-in Lightning Service Providers (LSP), on-chain interoperability, and third-party fiat on-ramps.

**Core Functions**

- **Sending payments** *via protocols such as: bolt11, keysend, lnurl-pay, lightning address, btc address.*
- **Receiving payments** *via protocols such as: bolt11, lnurl-withdraw, btc address.*
- **Interacting with a node** *e.g. balance, max allow to pay, max allow to receive, on-chain balance.*

**Key Features**

- [x]  On-chain interoperability
- [x]  Built-in LSP
- [x]  Integrated watchtower
- [x]  LNURL functionality
- [x]  Multi-app support
- [x]  Multi-device support
- [x]  Real-time state backup
- [x]  Keys are only held by users
- [x]  Fiat on-ramps
- [x]  Open-source

## Getting Started

Head over to the [Breez SDK - Native *(Greenlight Implementation)* documentation](https://sdk-doc.breez.technology/) to start implementing Lightning into your app.

Note: You'll need an API key to use the *Greenlight* implementation. For more info, email us at contact@breez.technology.

## **API**

API documentation is [here](https://breez.github.io/breez-sdk-greenlight/breez_sdk_core/).

## **Command line**

[Breez sdk-cli](https://github.com/breez/breez-sdk-greenlight/tree/main/tools/sdk-cli) is a command line client that allows you to interact with and test the functionality of the Breez SDK.

## **Support**

Have a question for the team? Join our [Telegram channel](https://t.me/breezsdk) or email us at [contact@breez.technology](mailto:contact@breez.technology) 

## How Does Native *(Greenlight Implementation)* Work?

The Breez SDK - Native *(Greenlight implementation)* allows end-users to send and receive payments using the Breez SDK through several key components:

- **Signer**: The app integrating the Breez SDK runs a validating signer that interacts with the end-user node.
- **Node**: End-user nodes are hosted on Blockstream’s Greenlight cloud infrastructure. The SDK creates a node when an end-user needs to send or receive a payment via the Lightning Network. Each end-user has their own node.
- **Lightning Service Providers (LSP)**: Design partners use LSPs, operated by entities other than Breez, to facilitate channel connectivity. LSP nodes are connected to Breez’s routing nodes, which in turn connect to other nodes in the Lightning Network.
- **Submarine Swaps**: Submarine swaps and reverse submarine swaps are used for transactions involving BTC addresses (on-chain). When receiving funds, submarine swaps convert the BTC to the user node on the Lightning Network. When sending funds to BTC addresses, reverse submarine swaps convert Lightning Network funds back to BTC.

![Breez SDK - Greenlight](https://github.com/breez/breez-sdk-docs/raw/main/src/images/BreezSDK_Greenlight.png)

## **Build & Test**

The libs folder contains three sub-folders and is a structured as a cargo workspace:

- **sdk-core**: the core SDK rust library.
- **sdk-bindings**: ffi bindings for Kotlin, Python, Swift, C#, and Go.
- **sdk-flutter**: a flutter plugin (includes ffi bindings for dart).
- **sdk-react-native**: a react-native plugin.

The tools folder contains a simple command line interface (sdk-cli) to the SDK.

See the instructions in each sub-project readme on how to build, test, and run.

## SDK Development Roadmap

- [x]  ‘On-the-fly’ channel creation
- [x]  Send/receive bolt11
- [x]  LNURL-Pay
- [x]  LNURL-Withdraw
- [x]  Backup/restore using mnemonics
- [x]  Send zero-amount invoices
- [x]  Fiat currencies
- [x]  Send spontaneous payments
- [x]  Send to a Lightning address
- [x]  Send to BIP 21
- [x]  cli inteface
- [x]  Swift bindings
- [x]  Kotlin bindings
- [x]  API key
- [x]  Receive via on-chain address
- [x]  React Native bindings
- [x]  LNURL-Auth
- [x]  Send to an on-chain address
- [x]  MoonPay fiat on-ramp
- [x]  C# bindings
- [x]  Python bindings
- [x]  Spend all funds
- [x]  Webhook for receiving payments
- [x]  Offline receive via notifications
- [ ]  LSPS2 support
- [ ]  LSPS1 support
- [ ]  Send/receive bolt12
- [ ]  Simplifed key management (cloud key backup)
- [ ]  WebAssembly support
- [ ]  Improve key share
- [ ]  Close channels to a predefined address
- [ ]  Make mempool.space dependency optional
- [ ]  Receive 0 amount invoice via LSP (unified QR)
- [ ]  Async payments
- [ ]  LDK support
- [ ]  Splicing
