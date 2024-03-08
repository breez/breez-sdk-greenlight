import Foundation

public class ResourceHelper {
    public static let shared = ResourceHelper()
    
    private init() {/* must use shared instance */}
    
    public func getString(key: String, fallback: String) -> String {
        return getString(key: key, validateContains: nil, fallback: fallback)
    }
    
    public func getString(key: String, validateContains: String?, fallback: String) -> String {
        if let str = Bundle.main.object(forInfoDictionaryKey: key) as? String {
            if validateContains == nil || str.contains(validateContains!) {
                return str
            }
        }
        return fallback
    }}
