// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AVSpeechSynthesizerBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "AVSpeechSynthesizerBridge",
            type: .static,
            targets: ["AVSpeechSynthesizerBridge"])
    ],
    targets: [
        .target(
            name: "AVSpeechSynthesizerBridge",
            path: "Sources/AVSpeechSynthesizerBridge",
            publicHeadersPath: "include")
    ]
)
