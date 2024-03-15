import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "BreezSDKConnector"
)
#else
fileprivate var logger = OSLog.disabled
#endif

class BreezSDKConnector {
    private static var breezSDK: BlockingBreezServices? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKConnector")
    fileprivate static var sdkListener: EventListener? = nil
    
    static func register(connectRequest: ConnectRequest, listener: EventListener) throws -> BlockingBreezServices? {
        try BreezSDKConnector.queue.sync { [] in
            BreezSDKConnector.sdkListener = listener
            if BreezSDKConnector.breezSDK == nil {
                BreezSDKConnector.breezSDK = try BreezSDKConnector.connectSDK(connectRequest: connectRequest)
            }
            return BreezSDKConnector.breezSDK
        }
    }
    
    static func unregister() {
        BreezSDKConnector.queue.sync { [] in
            BreezSDKConnector.sdkListener = nil
        }
    }
    
    static func connectSDK(connectRequest: ConnectRequest) throws -> BlockingBreezServices? {
        // Connect to the Breez SDK make it ready for use
        os_log("Connecting to Breez SDK", log: logger, type: .debug)
        let breezSDK = try connect(req: connectRequest, listener: BreezSDKEventListener())
        os_log("Connected to Breez SDK", log: logger, type: .debug)
        return breezSDK
    }
}

class BreezSDKEventListener: EventListener {
    func onEvent(e: BreezEvent) {
        BreezSDKConnector.queue.async { [] in
            BreezSDKConnector.sdkListener?.onEvent(e: e)
        }
    }
}
