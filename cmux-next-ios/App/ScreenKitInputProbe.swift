// Compile probe only (scratch branch, never merged): the app side of the
// CmuxScreenKit input API builds for iOS.
import CmuxScreenKit
import CmuxScreenKitUI
import SwiftUI
import UIKit

@MainActor
struct ScreenKitInputProbe {
    func views(_ session: ScreenSession) -> some View {
        session.preferences.inputMode = .trackpad
        _ = session.hostCapabilities
        _ = session.supportsPixelScroll
        session.scrollPixels(dx: 1, dy: 1)
        session.click(.left, modifiers: [.command])
        let controller = ScreenViewController(session: session)
        controller.inputMode = .direct
        let features: ScreenFeatures = [.inputModeToggle]
        return VStack {
            ScreenView(session: session, inputMode: .trackpad)
            ScreenSessionView(session: session, features: features.union(.all)) {}
        }
    }
}
