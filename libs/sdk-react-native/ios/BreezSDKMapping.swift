import Foundation

class BreezSDKMapping {
    static func arrayOf(routeHints: [RouteHint]) -> [Any] {
        return routeHints.map { (routeHint) -> [String: Any] in return dictionaryOf(routeHint: routeHint) }
    }
    
    static func arrayOf(routeHintHops: [RouteHintHop]) -> [Any] {
        return routeHintHops.map { (routeHintHop) -> [String: Any?] in return dictionaryOf(routeHintHop: routeHintHop) }
    }
    
    static func dictionaryOf(bitcoinAddressData: BitcoinAddressData) -> [String: Any?] {
        return [
            "address": bitcoinAddressData.address,
            "network": self.valueOf(network: bitcoinAddressData.network),
            "amountSat": bitcoinAddressData.amountSat,
            "label": bitcoinAddressData.label,
            "message": bitcoinAddressData.message,
        ]
    }

    static func dictionaryOf(inputType: InputType) -> [String: Any?] {
        switch(inputType) {
        case let .bitcoinAddress(address):
            return ["type": "bitcoinAddress", "data": dictionaryOf(bitcoinAddressData: address)]
        case let .bolt11(invoice):
            return ["type": "bolt11", "data": dictionaryOf(lnInvoice: invoice)]
        case let .lnUrlAuth(data):
            return ["type": "lnUrlAuth", "data": dictionaryOf(lnUrlAuthRequestData: data)]
        case let .lnUrlError(data):
            return ["type": "lnUrlError", "data": dictionaryOf(lnUrlErrorData: data)]
        case let .lnUrlPay(data):
            return ["type": "lnUrlPay", "data": dictionaryOf(lnUrlPayRequestData: data)]
        case let .lnUrlWithdraw(data):
            return ["type": "lnUrlWithdraw", "data": dictionaryOf(lnUrlWithdrawRequestData: data)]
        case let .nodeId(nodeId):
            return ["type": "nodeId", "data": nodeId]
        case let .url(url):
            return ["type": "url", "data": url]
        }
    }
    
    static func dictionaryOf(lnInvoice: LnInvoice) -> [String: Any?] {
        return [
            "bolt11": lnInvoice.bolt11,
            "payeePubkey": lnInvoice.payeePubkey,
            "paymentHash": lnInvoice.paymentHash,
            "description": lnInvoice.description,
            "descriptionHash": lnInvoice.descriptionHash,
            "amountMsat": lnInvoice.amountMsat,
            "timestamp": lnInvoice.timestamp,
            "expiry": lnInvoice.expiry,
            "routingHints": self.arrayOf(routeHints: lnInvoice.routingHints),
            "paymentSecret": lnInvoice.paymentSecret
        ]
    }
    
    static func dictionaryOf(lnUrlAuthRequestData: LnUrlAuthRequestData) -> [String: Any] {
        return ["k1": lnUrlAuthRequestData.k1]
    }
    
    static func dictionaryOf(lnUrlErrorData: LnUrlErrorData) -> [String: Any] {
        return ["reason": lnUrlErrorData.reason]
    }
    
    static func dictionaryOf(lnUrlPayRequestData: LnUrlPayRequestData) -> [String: Any] {
        return [
            "callback": lnUrlPayRequestData.callback,
            "minSendable": lnUrlPayRequestData.minSendable,
            "maxSendable": lnUrlPayRequestData.maxSendable,
            "metadataStr": lnUrlPayRequestData.metadataStr,
            "commentAllowed": lnUrlPayRequestData.commentAllowed,
        ]
    }
    
    static func dictionaryOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData) -> [String: Any] {
        return [
            "callback": lnUrlWithdrawRequestData.callback,
            "k1": lnUrlWithdrawRequestData.k1,
            "defaultDescription": lnUrlWithdrawRequestData.defaultDescription,
            "minWithdrawable": lnUrlWithdrawRequestData.minWithdrawable,
            "maxWithdrawable": lnUrlWithdrawRequestData.maxWithdrawable,
        ]
    }

    static func dictionaryOf(routeHint: RouteHint) -> [String: Any] {
        return ["hops": self.arrayOf(routeHintHops: routeHint.hops)]
    }
    
    static func dictionaryOf(routeHintHop: RouteHintHop) -> [String: Any?] {
        return [
            "srcNodeId": routeHintHop.srcNodeId,
            "shortChannelId": routeHintHop.shortChannelId,
            "feesBaseMsat": routeHintHop.feesBaseMsat,
            "feesProportionalMillionths": routeHintHop.feesProportionalMillionths,
            "cltvExpiryDelta": routeHintHop.cltvExpiryDelta,
            "htlcMinimumMsat": routeHintHop.htlcMinimumMsat,
            "htlcMaximumMsat": routeHintHop.htlcMaximumMsat
        ]
    }
    
    static func valueOf(network: Network) -> String {
        switch(network) {
        case .bitcoin: return "bitcoin"
        case .regtest: return "regtest"
        case .signet: return "signet"
        case .testnet: return "testnet"
        }
    }
}
