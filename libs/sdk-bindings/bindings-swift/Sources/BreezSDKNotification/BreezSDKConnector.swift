import Foundation

class BreezSDKConnector {
    fileprivate let TAG = "BreezSDKConnector"
    
    private static var breezSDK: BlockingBreezServices? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKConnector")
    fileprivate static var sdkListener: EventListener? = nil
    
    static func register(connectRequest: ConnectRequest, logger: LogStream, listener: EventListener) throws -> BlockingBreezServices? {
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
    
    static func connectSDK(connectRequest: ConnectRequest, logger: LogStream) throws -> BlockingBreezServices? {
        try setLogStream(logStream: logger)
        
        // Connect to the Breez SDK make it ready for use
        logger.log(l: LogEntry(tag: TAG, line: "Connecting to Breez SDK", level: "TRACE"))
        let breezSDK = try connect(req: connectRequest, listener: BreezSDKEventListener())
        logger.log(l: LogEntry(tag: TAG, line: "Connected to Breez SDK", level: "TRACE"))
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
