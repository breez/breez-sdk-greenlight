import UserNotifications
import XCGLogger
import os.log

public protocol NotificationServiceProtocol: UNNotificationServiceExtension {
    var logger: XCGLogger { get set }
    var breezSDK: BlockingBreezServices? { get set }
    var contentHandler: ((UNNotificationContent) -> Void)? { get set }
    var bestAttemptContent: UNMutableNotificationContent? { get set }
    var currentTask: TaskProtocol? { get set }

    func getConnectRequest() -> ConnectRequest?
}

extension NotificationServiceProtocol {
    init() {
        self.init()
        logger = {
            let log = XCGLogger.default
            log.setup(level: .debug, showThreadName: true, showLevel: true, showFileNames: true, showLineNumbers: true)
            return log
        }()
    }
        
    func didReceive(
        _ request: UNNotificationRequest,
        withContentHandler contentHandler: @escaping (UNNotificationContent) -> Void
    ) {
        self.logger.info("Notification received")
        self.contentHandler = contentHandler
        self.bestAttemptContent = (request.content.mutableCopy() as? UNMutableNotificationContent)
        
        guard let connectRequest = self.getConnectRequest() else {
            if let content = bestAttemptContent {
                contentHandler(content)
            }
            return
        }
        
        if let currentTask = self.getTaskFromNotification() {
            self.currentTask = currentTask
            
            DispatchQueue.main.async {
                do {
                    self.logger.info("Breez SDK is not connected, connecting....")
                    self.breezSDK = try BreezSDKConnector.register(connectRequest: connectRequest, logger: self.logger, listener: currentTask)
                    self.logger.info("Breez SDK connected successfully")
                    try currentTask.start(breezSDK: self.breezSDK!)
                } catch {
                    self.logger.error("Breez SDK connection failed \(error)")
                    self.shutdown()
                }
            }
        }
    }
    
    func getTaskFromNotification() -> TaskProtocol? {
        guard let content = bestAttemptContent else { return nil }
        guard let notificationType = content.userInfo[Constants.MESSAGE_DATA_TYPE] as? String else { return nil }
        self.logger.info("Notification payload: \(content.userInfo)")
        self.logger.info("Notification type: \(notificationType)")
        
        guard let payload = content.userInfo[Constants.MESSAGE_DATA_PAYLOAD] as? String else {
            contentHandler!(content)
            return nil
        }

        switch(notificationType) {
        case Constants.MESSAGE_TYPE_PAYMENT_RECEIVED:
            self.logger.info("payment_received data string: \(payload)")
            return ReceivePaymentTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INFO:
            self.logger.info("lnurlpay_info data string: \(payload)")
            return LnurlPayInfoTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        case Constants.MESSAGE_TYPE_LNURL_PAY_INVOICE:
            self.logger.info("lnurlpay_invoice data string: \(payload)")
            return LnurlPayInvoiceTask(payload: payload, logger: self.logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent)
        default:
            return nil
        }
    }
    
    func serviceExtensionTimeWillExpire() {
        self.logger.info("serviceExtensionTimeWillExpire()")
        
        // iOS calls this function just before the extension will be terminated by the system.
        // Use this as an opportunity to deliver your "best attempt" at modified content,
        // otherwise the original push payload will be used.
        self.shutdown()
    }
    
    private func shutdown() -> Void {
        self.logger.info("shutting down...")
        BreezSDKConnector.unregister()
        self.logger.info("task unregistered")
        self.currentTask?.onShutdown()
    }
}
