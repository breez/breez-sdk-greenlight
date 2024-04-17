import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "ServiceLogger"
)
#else
fileprivate var logger = OSLog.disabled
#endif

class ServiceLogger {
    var logStream: LogStream?
    
    init(logStream: LogStream?) {
        self.logStream = logStream
    }
    
    func log(tag: String, line: String, level: String) {
        if let logger = logStream {
            logger.log(l: LogEntry(line: line, level: level))
        } else {
            switch(level) {
                case "ERROR":
                    os_log("[%{public}@] %{public}@", log: logger, type: .error, tag, line)
                    break
                case "INFO", "WARN":
                    os_log("[%{public}@] %{public}@", log: logger, type: .info, tag, line)
                    break
                case "TRACE":
                    os_log("[%{public}@] %{public}@", log: logger, type: .debug, tag, line)
                    break
                default:
                    os_log("[%{public}@] %{public}@", log: logger, type: .default, tag, line)
                    return
            }
        }
    }
}
