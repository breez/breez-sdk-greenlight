import UserNotifications
import Foundation
import XCGLogger

struct LnurlInvoiceRequest: Codable {
    let reply_url: String
    let amount: UInt64
}

struct LnurlInvoiceResponse: Decodable, Encodable {
    let pr: String
    let routes: [String]
    
    init(pr: String, routes: [String]) {
        self.pr = pr
        self.routes = routes
    }
}

class LnurlPayInvoiceTask : LnurlPayTask {
    init(payload: String, logger: XCGLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(breezSDK: BlockingBreezServices) throws {
        var lnurlInvoiceRequest: LnurlInvoiceRequest? = nil
        do {
            lnurlInvoiceRequest = try JSONDecoder().decode(LnurlInvoiceRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.error("failed to decode payload: \(e)")
            self.displayPushNotification(title: self.failNotificationTitle)
            throw e
        }
        
        do {
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            let nodeInfo = try breezSDK.nodeInfo()
            if lnurlInvoiceRequest!.amount < 1000 || lnurlInvoiceRequest!.amount > nodeInfo.inboundLiquidityMsats {
                fail(withError: "Invalid amount requested \(lnurlInvoiceRequest!.amount)", replyURL: lnurlInvoiceRequest!.reply_url)
                return
            }
            let receiveResponse = try breezSDK.receivePayment(req: ReceivePaymentRequest(amountMsat: lnurlInvoiceRequest!.amount, description: metadata, useDescriptionHash: true))
            self.replyServer(encodable: LnurlInvoiceResponse(pr: receiveResponse.lnInvoice.bolt11, routes: []), replyURL: lnurlInvoiceRequest!.reply_url)
        } catch let e {
            self.logger.error("failed to process lnurl: \(e)")
            self.fail(withError: e.localizedDescription, replyURL: lnurlInvoiceRequest!.reply_url)
        }
    }
}
