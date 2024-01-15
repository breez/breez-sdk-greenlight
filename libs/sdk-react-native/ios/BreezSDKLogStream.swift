import Foundation
import BreezSDK

class BreezSDKLogStream: LogStream {
    static let emitterName: String = "breezSdkLog"
    
    func log(l: LogEntry) {
        RNBreezSDK.emitter.sendEvent(withName: BreezSDKLogStream.emitterName,
                                     body: BreezSDKMapper.dictionaryOf(logEntry: l))
    }
}
