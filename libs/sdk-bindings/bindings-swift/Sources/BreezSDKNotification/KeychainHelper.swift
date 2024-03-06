import Foundation
import KeychainAccess
import os.log

#if DEBUG && true
fileprivate var log = Logger(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "KeychainHelper"
)
#else
fileprivate var log = Logger(OSLog.disabled)
#endif

public class KeychainHelper {
    public static let shared = KeychainHelper()
    
    private init() {/* must use shared instance */}
    
    public func getFlutterString(accessGroup: String, key: String) -> String? {
        return self.getString(service: "flutter_secure_storage_service", accessGroup: accessGroup, key: key)
    }
    
    public func getString(service: String, accessGroup: String, key: String) -> String? {
        let keychain = Keychain(service: service, accessGroup: accessGroup)
        do {
            return try keychain.getString(key)
        } catch let error {
            log.error("Failed to restore \(key) from \(service) keychain. Error: \(error)")
        }
        
        return nil
    }
}
