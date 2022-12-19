# Breez SDK

The Breez SDK enables mobile developers to integrate Lightning and bitcoin payments into their apps with a very shallow learning curve. The use cases are endless – from social apps that want to integrate tipping between users to content-creation apps interested in adding bitcoin monetization. Crucially, this SDK is an end-to-end, non-custodial, drop-in solution powered by Greenlight, a built-in LSP, on-chain interoperability, third-party fiat on-ramps, and other services users and operators need.
   
The Breez SDK provides the following services:
* Sending payments (via various protocols such as: bolt11, keysend, lnurl-pay, lightning address, etc.)
* Receiving payments (via various protocols such as: bolt11, lnurl-withdraw, etc.)
* Fetching node status (e.g. balance, max allow to pay, max allow to receive, on-chain balance, etc.)
* Connecting to a new or existing node.

This diagram is a high-level description of the Breez SDK:
![sdk](https://user-images.githubusercontent.com/5394889/174237369-05aad114-4af8-448e-9fbb-ad6adff835a5.png)

Youcan watch a basic demo here: https://twitter.com/Breez_Tech/status/1602650230088151040?s=20&t=w7Ej2oXjZ0hsXiwZV8K4SA

### Signer
This module handles everything related to the signing of lightning messages. It is initialized with the user’s seed.
### InputParser
This module parses user input that is related to sending and receiving of payments. It identifies the protocol (lightning, lnurl-pay, lightning address, lnurl-withdraw, btc address, etc.) and the related data. Apps should use this parser to interpret users input, display the details to users and then execute the specific action (pay or receive).
### LightningNode
This is an interface that defines how the SDK interacts with the user’s Lightning node. The interface defines methods to create a new node, connect to an existing node, pay or create invoices. It also provides access to the node’s low-level functionality such as: listing peers, graph information etc. Currently we only have one provider (Greenlight) but we can add more providers in the future.
### BTCSwapper
This module provides the ability to send or receive on-chain payments via submarine swaps. Send to a BTC address is done by a reverse submarine swap and receive by a regular submarine swap. It includes refund functionality as well.
### FiatCurrencies
This module provides fiat currencies conversion services and fiat on-ramp service (via MoonPay).
### LSP
This module provides the interface of interacting with one or more LSPs.

## Build & Testing
The libs folder contains three sub folders and is a structured as a cargo workspace:
* sdk-core: the core SDK rust library. 
* sdk-bindings: ffi bindings for Kotlin & Swift. 
* sdk-flutter: a flutter plugin (includes ffi bindings for dart).

See instruction in each sub project readme on how to build, test and run.
