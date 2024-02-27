import Foundation

struct Constants {
    // Message Data
    static let MESSAGE_DATA_TYPE = "notification_type"
    static let MESSAGE_DATA_PAYLOAD = "notification_payload"
    
    static let MESSAGE_TYPE_PAYMENT_RECEIVED = "payment_received"
    static let MESSAGE_TYPE_LNURL_PAY_INFO = "lnurlpay_info"
    static let MESSAGE_TYPE_LNURL_PAY_INVOICE = "lnurlpay_invoice"
    
    // Resource Identifiers
    static let LNURL_PAY_INFO_NOTIFICATION_TITLE = "lnurl_pay_info_notification_title"
    static let LNURL_PAY_INVOICE_NOTIFICATION_TITLE = "lnurl_pay_invoice_notification_title"
    static let LNURL_PAY_METADATA_PLAIN_TEXT = "lnurl_pay_metadata_plain_text"
    static let LNURL_PAY_NOTIFICATION_FAILURE_TITLE = "lnurl_pay_notification_failure_title"
    static let PAYMENT_RECEIVED_NOTIFICATION_TITLE = "payment_received_notification_title"
    
    // Resource Identifier Defaults
    static let DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE = "Retrieving Payment Information"
    static let DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE = "Fetching invoice"
    static let DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT = "Pay with LNURL"
    static let DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE = "Receive Payment Failed"
    static let DEFAULT_PAYMENT_RECEIVED_NOTIFICATION_TITLE = "Received %d sats"
}
