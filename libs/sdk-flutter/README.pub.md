# Breez SDK plugin

[![pub package](https://img.shields.io/pub/v/breez_sdk.svg)](https://pub.dev/packages/breez_sdk)

## Table of contents
- [Description](#description)
- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)

## Description

This is a Flutter package that wraps the Dart bindings of [Breez SDK](https://github.com/breez/breez-sdk?tab=readme-ov-file#readme).

|             | Android | iOS   |
|-------------|---------|-------|
| **Support** | SDK 21+ | 12.0+ |

## Overview

Breez SDK enables developers to integrate Lightning and bitcoin payments into their apps with a very shallow learning curve. The use cases are endless – from social apps that want to integrate tipping between users to content-creation apps interested in adding bitcoin monetization. Crucially, this SDK is an end-to-end, non-custodial, drop-in solution powered by Greenlight, a built-in LSP, on-chain interoperability, third-party fiat on-ramps, and other services users and operators need. Breez SDK is free for developers.

Breez SDK provides the following services:

 - Sending payments (via various protocols such as: bolt11, keysend, lnurl-pay, lightning address, etc.)
 - Receiving payments (via various protocols such as: bolt11, lnurl-withdraw, etc.)
 - Fetching node status (e.g. balance, max allow to pay, max allow to receive, on-chain balance, etc.)
 - Connecting to a new or existing node.

and many more! See [Documentation](#documentation).

## Installation
To use this plugin, add `breez_sdk` as a [dependency in your pubspec.yaml file](https://flutter.dev/docs/development/platform-integration/platform-channels).

## Usage

To start using this package first import it in your Dart file.

```dart
import 'package:breez_sdk/breez_sdk.dart';
```

It's recommended to use a single instance of `BreezSDK()` throughout your app. Assuming `breezSDK` variable in the example points to this instance: 

Check whether Breez node services are initialized first by calling `isInitialized()` and then call `initialize()` to initialize Breez SDK's event & log streams, preferably on `main.dart`:

```dart
if (!await breezSDK.isInitialized()) {
  breezSDK.initialize();
}
```

Please refer to Dart examples on Breez SDK documentation for more information on features & capabilities of the Breez SDK.

## Documentation

- [Official Breez SDK documentation](https://sdk-doc.breez.technology/)
