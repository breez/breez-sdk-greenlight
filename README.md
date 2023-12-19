# Breez SDK

## Overview
The Breez SDK enables mobile developers to integrate Lightning and bitcoin payments into their apps with a very shallow learning curve. The use cases are endless – from social apps that want to integrate tipping between users to content-creation apps interested in adding bitcoin monetization. Crucially, this SDK is an end-to-end, non-custodial, drop-in solution powered by Greenlight, a built-in LSP, on-chain interoperability, third-party fiat on-ramps, and other services users and operators need. The Breez SDK is free for developers.
   
The Breez SDK provides the following services:
* Sending payments (via various protocols such as: bolt11, keysend, lnurl-pay, lightning address, etc.)
* Receiving payments (via various protocols such as: bolt11, lnurl-withdraw, etc.)
* Fetching node status (e.g. balance, max allow to pay, max allow to receive, on-chain balance, etc.)
* Connecting to a new or existing node.

Download our one pager [here](https://drive.google.com/file/d/1MBINTyEXX9tFLVXd3QoTUKLNWgjgWN2I/view?usp=drivesdk). 

To get started with the Breez SDK, follow [these examples](https://sdk-doc.breez.technology/).

Note: in order to use the Breez SDK you need an API key from Breez, please email us at contact@breez.technology for more information.

## Demo

https://user-images.githubusercontent.com/31890660/208511040-989ff42c-ceb8-4a34-b2cb-a17a0a8c0150.mp4

For a higher resolution video, click [here](https://youtu.be/PRVWB4K52Es).

## Features

![Screenshot_2023-06-27-20-18-57-35_e2d5b3f32b79de1d45acd1fad96fbb0f](https://github.com/breez/breez-sdk/assets/31890660/e54cf75a-b9e4-43c7-a448-34da4022c59f)

## Using the SDK
To get started with the Breez SDK, follow [these examples](https://sdk-doc.breez.technology/).

## API
API documentation is [here](https://breez.github.io/breez-sdk-rustdoc/doc/breez_sdk_core/). 

## Support
Join [this telegram group](https://t.me/breezsdk).

## Architecture
This diagram is a high-level description of the Breez SDK:

![SDK Architecture](https://user-images.githubusercontent.com/31890660/208512955-6e648b86-4c8a-457a-b844-3dda8b2aa8ec.png)

## Command line
[Breez sdk-cli](https://github.com/breez/breez-sdk/tree/main/tools/sdk-cli) is a command line client that allows you to test the functionality of the Breez SDK.

## Build & Test
The libs folder contains three sub folders and is a structured as a cargo workspace:
* **sdk-core**: the core SDK rust library. 
* **sdk-bindings**: ffi bindings for Kotlin, Python, Swift, C# and Go. 
* **sdk-flutter**: a flutter plugin (includes ffi bindings for dart).
* **sdk-react-native**: a react-native plugin.

The tools folder contains a simple command line interface (sdk-cli) to the SDK.
See the instructions in each sub project readme on how to build, test and run.

## TODOs
- [x] ‘On-the-fly’ channel creation
- [x] Send/receive bolt11
- [x] LNURL-Pay
- [x] LNURL-Withdraw
- [x] Backup/restore using mnemonics 
- [x] Send zero-amount invoices
- [x] Fiat currencies
- [x] Send spontaneous payments
- [x] Send to a Lightning address
- [x] Send to BIP 21
- [x] cli inteface 
- [x] Swift bindings
- [x] Kotlin bindings
- [x] API key
- [x] Receive via on-chain address
- [x] React Native bindings
- [x] LNURL-Auth
- [x] Send to an on-chain address
- [x] MoonPay fiat on-ramp
- [x] C# bindings 
- [x] Python bindings 
- [x] Spend all funds
- [x] Webhook for receiving payments
- [x] Offline receive via notifications
- [ ] LSPS2 support 
- [ ] LSPS1 support
- [ ] Send/receive bolt12
- [ ] Simplifed key management (cloud key backup)
- [ ] WebAssembly support
- [ ] Improve key share 
- [ ] Close channels to a predefined address 
- [ ] Make mempool.space dependency optional 
- [ ] Receive 0 amount invoice via LSP (unified QR)
- [ ] Async payments
- [ ] LDK support
- [ ] Splicing
