import Foundation
import os.log

#if DEBUG && true
fileprivate var os_log = Logger(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "ServiceLogger"
)
#else
fileprivate var os_log = Logger(OSLog.disabled)
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
                    os_log.error("[\(tag)] \(line)")
                    break
                case "WARN":
                    os_log.warning("[\(tag)] \(line)")
                    break
                case "INFO":
                    os_log.info("[\(tag)] \(line)")
                    break
                case "DEBUG":
                    os_log.debug("[\(tag)] \(line)")
                    break
                case "TRACE":
                    os_log.trace("[\(tag)] \(line)")
                    break
                default:
                    return
            }
        }
    }
}
