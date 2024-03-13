import UserNotifications
import Foundation

struct LnurlInfoRequest: Codable {
    let callback_url: String
    let reply_url: String
}

struct LnurlInfoResponse: Decodable, Encodable {
    let callback: String
    let maxSendable: UInt64
    let minSendable: UInt64
    let metadata: String
    let tag: String
    
    init(callback: String, maxSendable: UInt64, minSendable: UInt64, metadata: String, tag: String) {
        self.callback = callback
        self.maxSendable = maxSendable
        self.minSendable = minSendable
        self.metadata = metadata
        self.tag = tag
    }
}

class LnurlPayInfoTask : LnurlPayTask {
    fileprivate let TAG = "LnurlPayInfoTask"
    
    init(payload: String, logger: ServiceLogger, config: ServiceConfig, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INFO_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, config: config, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(breezSDK: BlockingBreezServices) throws {
        var lnurlInfoRequest: LnurlInfoRequest? = nil
        do {
            lnurlInfoRequest = try JSONDecoder().decode(LnurlInfoRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger)
            throw e
        }
        
        do {
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            // Get the fee parameters offered by the LSP for opening a new channel
            let ofp = try breezSDK.openChannelFee(req: OpenChannelFeeRequest(amountMsat: nil)).feeParams
            // Calculate maximum receivable amount that falls within fee limits(in millisatoshis)
            let feeLimitMsats: UInt64 = config.autoChannelSetupFeeLimitMsats
            let nodeInfo = try breezSDK.nodeInfo()
            let maxReceivableMsatsThatFallsWithinFeeLimits = min(nodeInfo.maxReceivableMsat, feeLimitMsats / (UInt64(ofp.proportional) / 1000000))
            // Calculate maximum sendable amount(in millisatoshis)
            let maxSendable = max(nodeInfo.inboundLiquidityMsats, maxReceivableMsatsThatFallsWithinFeeLimits)
            // Get the minimum sendable amount(in millisatoshis), can not be less than 1 or more than maxSendable
            let minSendable: UInt64 = nodeInfo.inboundLiquidityMsats < UInt64(1000) ? ofp.minMsat :  UInt64(1000)
            if(minSendable > maxSendable) {
                throw InvalidMinSendable.largerThanMaxSendable
            }
            replyServer(encodable: LnurlInfoResponse(callback: lnurlInfoRequest!.callback_url, maxSendable: maxSendable, minSendable: minSendable, metadata: metadata, tag: "payRequest"),
                        replyURL: lnurlInfoRequest!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl: \(e)", level: "ERROR")
            fail(withError: e.localizedDescription, replyURL: lnurlInfoRequest!.reply_url)
        }
    }
}

private enum InvalidMinSendable: Error {
    case largerThanMaxSendable
}

extension InvalidMinSendable: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .largerThanMaxSendable:
            return NSLocalizedString("Minimum sendable amount can't be greater than maximum sendable amount.", comment: "InvalidMinSendable")
        }
    }
}
