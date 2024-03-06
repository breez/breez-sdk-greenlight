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
    
    func log(tag: String?, line: String, level: String) {
        if let logger = logStream {
            logger.log(l: LogEntry(tag: tag, line: line, level: level))
        } else {
            switch(level) {
                case "ERROR":
                    os_log.error("[\(tag ?? "greenlight")] \(line)")
                    break
                case "WARN":
                    os_log.warning("[\(tag ?? "greenlight")] \(line)")
                    break
                case "INFO":
                    os_log.info("[\(tag ?? "greenlight")] \(line)")
                    break
                case "DEBUG":
                    os_log.debug("[\(tag ?? "greenlight")] \(line)")
                    break
                case "TRACE":
                    os_log.trace("[\(tag ?? "greenlight")] \(line)")
                    break
                default:
                    return
            }
        }
    }
}
