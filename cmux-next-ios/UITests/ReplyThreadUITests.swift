import XCTest

/// Chief's reply thread behaves like Messages' (verify/imsg-replies): the
/// thread is its own transcript, so my message in it keeps its delivery
/// status even when the main transcript shows that status on a later
/// message; the send button reads "Send Reply" while replying; and a sent
/// reply joins the open thread, which stays open in reply mode. Real
/// touches on the mock Chief (mock sign-in), whose history has an incoming
/// reply to one of my messages.
final class ReplyThreadUITests: XCTestCase {
    override func setUp() {
        continueAfterFailure = false
    }

    @MainActor
    func testReplyThreadShowsItsOwnStatusAndSendsAReply() throws {
        let app = XCUIApplication()
        app.launchEnvironment["CMUX_MOBILE_MOCK_AUTH"] = "1"
        app.launchArguments += ["-cmuxMobile.setupComplete", "YES", "-cmux-mobile-demo", "chief-open"]
        app.launch()

        let field = app.textViews["conversation.composer.text"]
        XCTAssertTrue(field.waitForExistence(timeout: 30), "Chief's composer\n\(app.debugDescription)")
        // The main transcript puts "Read" under my newest message only.
        let replies = app.buttons["1 Reply"].firstMatch
        XCTAssertTrue(replies.waitForExistence(timeout: 10), "the \"1 Reply\" link under my request\n\(app.debugDescription)")
        replies.tap()

        let thread = app.otherElements["conversation.replyThread"]
        XCTAssertTrue(thread.waitForExistence(timeout: 5), "the reply thread opens")
        // My request is the thread's newest message of mine: Messages shows its status there.
        let status = thread.descendants(matching: .any).matching(NSPredicate(format: "label BEGINSWITH %@", "Read")).firstMatch
        XCTAssertTrue(status.waitForExistence(timeout: 5), "my message in the thread keeps its status\n\(thread.debugDescription)")
        attach(app, "thread")

        let send = app.buttons["conversation.composer.send"]
        if !field.hasKeyboardFocus { field.tap() }
        field.typeText("On it")
        XCTAssertTrue(send.waitForExistence(timeout: 5))
        XCTAssertEqual(send.label, "Send Reply", "the send button names what it sends while replying")
        attach(app, "draft")

        send.tap()
        let sent = thread.descendants(matching: .any).matching(NSPredicate(format: "label CONTAINS %@", "On it")).firstMatch
        XCTAssertTrue(sent.waitForExistence(timeout: 5), "the reply joins the open thread\n\(thread.debugDescription)")
        XCTAssertTrue(thread.exists, "the thread stays open after a reply is sent")
        XCTAssertEqual(field.placeholderValue, "Reply", "the composer stays in reply mode")
        attach(app, "sent")
    }

    @MainActor
    private func attach(_ app: XCUIApplication, _ name: String) {
        let shot = XCTAttachment(screenshot: app.screenshot())
        shot.name = name
        shot.lifetime = .keepAlways
        add(shot)
    }
}

private extension XCUIElement {
    var hasKeyboardFocus: Bool { (value(forKey: "hasKeyboardFocus") as? Bool) ?? false }
}
