import UserNotifications
import Foundation

struct AddressTxsConfirmedRequest: Codable {
    let address: String
}

class RedeemSwapTask : TaskProtocol {
    fileprivate let TAG = "RedeemSwapTask"
    
    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: LogStream?
    internal var receivedPayment: Payment? = nil
    
    init(payload: String, logger: LogStream?, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
    }
    
    public func onEvent(e: BreezEvent) {}
    
    func start(breezSDK: BlockingBreezServices) throws {
        var addressTxsConfirmedRequest: AddressTxsConfirmedRequest? = nil
        do {
            addressTxsConfirmedRequest = try JSONDecoder().decode(AddressTxsConfirmedRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger?.log(l: LogEntry(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR"))
            self.onShutdown()
            throw e
        }

        do {
            try breezSDK.redeemSwap(swapAddress: addressTxsConfirmedRequest!.address)
            self.logger?.log(l: LogEntry(tag: TAG, line: "Found swap for \(addressTxsConfirmedRequest!.address)", level: "DEBUG"))
            let successRedeemSwap = ResourceHelper.shared.getString(key: Constants.SWAP_TX_CONFIRMED_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_TITLE)
            self.displayPushNotification(title: successRedeemSwap)
        } catch let e {
            self.logger?.log(l: LogEntry(tag: TAG, line: "Failed to process swap notification: \(e)", level: "ERROR"))
            self.onShutdown()
        }
    }

    func onShutdown() {
        let failRedeemSwap = ResourceHelper.shared.getString(key: Constants.SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE)
        self.displayPushNotification(title: failRedeemSwap)
    }
}
