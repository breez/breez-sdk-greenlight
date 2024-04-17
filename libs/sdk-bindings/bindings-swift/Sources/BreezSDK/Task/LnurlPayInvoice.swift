import UserNotifications
import Foundation

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
    fileprivate let TAG = "LnurlPayInvoiceTask"
    
    init(payload: String, logger: ServiceLogger, config: ServiceConfig, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, config: config, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(breezSDK: BlockingBreezServices) throws {
        var request: LnurlInvoiceRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlInvoiceRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
            throw e
        }
        
        do {
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            // Get channel setup fee for invoice amount
            let ofpResp = try breezSDK.openChannelFee(req: OpenChannelFeeRequest(amountMsat: request!.amount))
            // Check if channel setup fee is within fee limit
            let feeLimitMsat: UInt64 = config.channelFeeLimitMsat
            let isFeeWithinLimit = ofpResp.feeMsat! == 0 || ofpResp.feeMsat! <= feeLimitMsat
            // Get minimum amount LN service is willing to receive
            let minMsat: UInt64 = ofpResp.feeMsat! == 0 ? UInt64(1000) : ofpResp.feeParams.minMsat
            // Check if the invoice amount is larger than minimum accepted amount and if its fee is within fee limit
            if request!.amount < minMsat || !isFeeWithinLimit {
                fail(withError: "Invalid amount requested \(request!.amount)", replyURL: request!.reply_url)
                return
            }
            let receiveResponse = try breezSDK.receivePayment(req: ReceivePaymentRequest(amountMsat: request!.amount, description: metadata, useDescriptionHash: true))
            self.replyServer(encodable: LnurlInvoiceResponse(pr: receiveResponse.lnInvoice.bolt11, routes: []), replyURL: request!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl: \(e)", level: "ERROR")
            self.fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }
}
