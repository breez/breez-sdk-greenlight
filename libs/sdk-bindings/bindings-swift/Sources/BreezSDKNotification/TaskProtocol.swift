import UserNotifications

public protocol TaskProtocol : EventListener {
    fileprivate let TAG = "TaskProtocol"
    
    var payload: String { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }
    var logger: LogStream? { get set }
    
    func start(breezSDK: BlockingBreezServices) throws
    func onShutdown()
}

extension TaskProtocol {
    func displayPushNotification(title: String) {
        self.logger?.log(l: LogEntry(tag: TAG,"displayPushNotification \(title)", level: "INFO"))
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
