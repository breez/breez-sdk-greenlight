import Foundation
import BreezSDK
import os.log

#if DEBUG && true
fileprivate var log = Logger(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "BreezSDKConnector"
)
#else
fileprivate var log = Logger(OSLog.disabled)
#endif

class BreezSDKConnector {
    private static var breezSDK: BlockingBreezServices? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKConnector")
    fileprivate static var sdkListener: EventListener? = nil
    
    static func register(connectRequest: ConnectRequest, logger: LogStream?, listener: EventListener) throws -> BlockingBreezServices? {
        try BreezSDKConnector.queue.sync { [] in
            BreezSDKConnector.sdkListener = listener
            if BreezSDKConnector.breezSDK == nil {
                BreezSDKConnector.breezSDK = try BreezSDKConnector.connectSDK(connectRequest: connectRequest, logger: logger)
            }
            return BreezSDKConnector.breezSDK
        }
    }
    
    static func unregister() {
        BreezSDKConnector.queue.sync { [] in
            BreezSDKConnector.sdkListener = nil
        }
    }
    
    static func connectSDK(connectRequest: ConnectRequest, logger: LogStream?) throws -> BlockingBreezServices? {
        if let logger = logger {
            try setLogStream(logStream: logger)
        }
        
        // Connect to the Breez SDK make it ready for use
        log.trace("Connecting to Breez SDK")
        let breezSDK = try connect(req: connectRequest, listener: BreezSDKEventListener())
        log.trace("Connected to Breez SDK")
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
