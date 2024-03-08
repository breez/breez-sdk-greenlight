import UserNotifications

public protocol TaskProtocol : EventListener {
    var payload: String { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }
    
    func start(breezSDK: BlockingBreezServices) throws
    func onShutdown()
}

extension TaskProtocol {
    func displayPushNotification(title: String, logger: ServiceLogger) {
        logger.log(tag: "TaskProtocol", line:"displayPushNotification \(title)", level: "INFO")
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
