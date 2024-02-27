import UserNotifications
import XCGLogger

public protocol TaskProtocol : EventListener {
    var payload: String { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }
    var logger: XCGLogger { get set }

    func start(breezSDK: BlockingBreezServices) throws
    func onShutdown()
}

extension TaskProtocol {    
    func displayPushNotification(title: String) {
        self.logger.info("displayPushNotification \(title)")
        
        guard
            let contentHandler = contentHandler,
            let bestAttemptContent = bestAttemptContent
        else {
            return
        }

        bestAttemptContent.title = title
        contentHandler(bestAttemptContent)
    }
}
