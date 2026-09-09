// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "CmuxMagnetSidebar",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "CmuxMagnetSidebar", targets: ["CmuxMagnetSidebar"]),
    ],
    targets: [
        .target(name: "CmuxMagnetSidebar", path: "Sources/CmuxMagnetSidebar"),
    ]
)
