public struct ServiceConfig {
    // MARK: - Constants
    public struct Constants {
        /// Automatic channel setup fee limit (in msats)  for the default instance of ServiceConfig
        public static let defaultAutoChannelSetupFeeLimitMsats = UInt64(0)
    }
    
    // MARK: - Default instance
    /// The default ServiceConfig object
    public static let `default`: ServiceConfig = ServiceConfig(autoChannelSetupFeeLimitMsats: ServiceConfig.Constants.defaultAutoChannelSetupFeeLimitMsats)
    
    // MARK: - Properties
    /// Automatic channel setup fee limit (in msats) 
    public var autoChannelSetupFeeLimitMsats: UInt64! = UInt64(0)
    
    // MARK: - Life Cycle
    public init(autoChannelSetupFeeLimitMsats: UInt64) {
        self.autoChannelSetupFeeLimitMsats = autoChannelSetupFeeLimitMsats
    }
}
