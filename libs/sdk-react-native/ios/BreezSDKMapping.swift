import Foundation

class BreezSDKMapping {
    static func arrayOf(routeHints: [RouteHint]) -> [Any] {
        return routeHints.map { (routeHint) -> [String: Any] in return dictionaryOf(routeHint: routeHint) }
    }
    
    static func arrayOf(routeHintHops: [RouteHintHop]) -> [Any] {
        return routeHintHops.map { (routeHintHop) -> [String: Any?] in return dictionaryOf(routeHintHop: routeHintHop) }
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
    
    static func dictionaryOf(routeHint: RouteHint) -> [String: Any] {
        return ["hops": self.arrayOf(routeHintHops: routeHint.hops)]
    }
    
    static func dictionaryOf(routeHintHop: RouteHintHop) -> [String: Any?] {
        return ["srcNodeId": routeHintHop.srcNodeId,
                "shortChannelId": routeHintHop.shortChannelId,
                "feesBaseMsat": routeHintHop.feesBaseMsat,
                "feesProportionalMillionths": routeHintHop.feesProportionalMillionths,
                "cltvExpiryDelta": routeHintHop.cltvExpiryDelta,
                "htlcMinimumMsat": routeHintHop.htlcMinimumMsat,
                "htlcMaximumMsat": routeHintHop.htlcMaximumMsat]
    }

}
