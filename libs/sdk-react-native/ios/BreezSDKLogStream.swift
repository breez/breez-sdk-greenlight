import BreezSDK
import Foundation

class BreezSDKLogStream: LogStream {
    static let emitterName: String = "breezSdkLog"
    
    func log(l: LogEntry) {
        if RNBreezSDK.hasListeners {
            RNBreezSDK.emitter.sendEvent(withName: BreezSDKLogStream.emitterName,
                                         body: BreezSDKMapper.dictionaryOf(logEntry: l))
        }
    }
}
