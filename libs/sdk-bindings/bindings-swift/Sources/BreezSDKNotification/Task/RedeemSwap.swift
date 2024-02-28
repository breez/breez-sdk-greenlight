import UserNotifications
import Foundation
import XCGLogger

struct AddressTxsConfirmedRequest: Codable {
    let address: String
}

class RedeemSwapTask : TaskProtocol {
    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: XCGLogger
    internal var receivedPayment: Payment? = nil
    
    init(payload: String, logger: XCGLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
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
            self.logger.error("failed to decode payload: \(e)")
            self.onShutdown()
            throw e
        }

        do {
            try breezSDK.redeemSwap(swapAddress: addressTxsConfirmedRequest!.address)
            self.logger.debug("Found swap for \(addressTxsConfirmedRequest!.address)")
            let successRedeemSwap = ResourceHelper.shared.getString(key: Constants.SWAP_TX_CONFIRMED_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_TITLE)
            self.displayPushNotification(title: successRedeemSwap)
        } catch let e {
            self.logger.error("Failed to process swap notification: \(e)")
            self.onShutdown()
        }
    }

    func onShutdown() {
        let failRedeemSwap = ResourceHelper.shared.getString(key: Constants.SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_SWAP_TX_CONFIRMED_NOTIFICATION_FAILURE_TITLE)
        self.displayPushNotification(title: failRedeemSwap)
    }
}
