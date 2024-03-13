package breez_sdk_notification

data class ServiceConfig(
    /** Channel setup fee limit (in msats) */
    val channelFeeLimitMsat: ULong = defaultChannelFeeLimitMsat,
) {
    companion object {
        /** Channel setup fee limit (in msats) for the default ServiceConfig */
        const val defaultChannelFeeLimitMsat: ULong = 0u

        /** The default ServiceConfig */
        fun default(): ServiceConfig = ServiceConfig(defaultChannelFeeLimitMsat)
    }
}