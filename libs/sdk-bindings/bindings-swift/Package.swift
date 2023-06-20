// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "bindings-swift",
    platforms: [
        .macOS(.v12),
        .iOS(.v11),
    ],
    products: [
        .library(name: "BreezSDK", targets: ["breez_sdkFFI", "BreezSDK"]),
    ],
    dependencies: [
    ],
    targets: [
        .binaryTarget(name: "breez_sdkFFI", path: "./breez_sdkFFI.xcframework"),
        .target(name: "BreezSDK", dependencies: ["breez_sdkFFI"]),
        .testTarget(name: "BreezSDKTests", dependencies: ["BreezSDK"]),
    ]
)
