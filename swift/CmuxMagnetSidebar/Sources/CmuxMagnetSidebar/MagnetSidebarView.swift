import SwiftUI

/// Sidebar UI for Magnet-style layout presets (CmuxExtensionKit host).
/// Live drag snap-zones need native cmux (#12230).
public struct MagnetSidebarView: View {
    public var onApply: (String) -> Void

    public init(onApply: @escaping (String) -> Void = { _ in }) {
        self.onApply = onApply
    }

    private let presets: [(id: String, label: String)] = [
        ("rows-3", "3 equal rows"),
        ("cols-3", "3 equal columns"),
        ("halves", "Halves"),
        ("grid-2x2", "2x2 grid"),
    ]

    public var body: some View {
        List {
            Section("Magnet layouts") {
                ForEach(presets, id: \.id) { preset in
                    Button(preset.label) { onApply(preset.id) }
                }
            }
            Section {
                Text("Blue drag snap-zones need native cmux (#12230). This picker drives `cmux-magnet apply`.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
}
