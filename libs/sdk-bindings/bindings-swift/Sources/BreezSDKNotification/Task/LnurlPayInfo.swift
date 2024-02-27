import UserNotifications
import Foundation
import XCGLogger

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
    init(payload: String, logger: XCGLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INFO_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(breezSDK: BlockingBreezServices) throws {
        var lnurlInfoRequest: LnurlInfoRequest? = nil
        do {
            lnurlInfoRequest = try JSONDecoder().decode(LnurlInfoRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.error("failed to decode payload: \(e)")
            self.displayPushNotification(title: self.failNotificationTitle)
            throw e
        }

        do {
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            let nodeInfo = try breezSDK.nodeInfo()
            replyServer(encodable: LnurlInfoResponse(callback: lnurlInfoRequest!.callback_url, maxSendable: nodeInfo.inboundLiquidityMsats, minSendable: UInt64(1000), metadata: metadata, tag: "payRequest"),
                        replyURL: lnurlInfoRequest!.reply_url)
        } catch let e {
            self.logger.error("failed to process lnurl: \(e)")
            fail(withError: e.localizedDescription, replyURL: lnurlInfoRequest!.reply_url)
        }
    }
}
