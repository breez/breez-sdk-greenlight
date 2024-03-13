public struct ServiceConfig {
    // MARK: - Constants
    public struct Constants {
        /// Channel setup fee limit (in msats)  for the default instance of ServiceConfig
        public static let defaultChannelFeeLimitMsat = UInt64(0)
    }
    
    // MARK: - Default instance
    /// The default ServiceConfig object
    public static let `default`: ServiceConfig = ServiceConfig(channelFeeLimitMsat: ServiceConfig.Constants.defaultChannelFeeLimitMsat)
    
    // MARK: - Properties
    /// Channel setup fee limit (in msats)
    public var channelFeeLimitMsat: UInt64! = UInt64(0)
    
    // MARK: - Life Cycle
    public init(channelFeeLimitMsat: UInt64) {
        self.channelFeeLimitMsat = channelFeeLimitMsat
    }
}
