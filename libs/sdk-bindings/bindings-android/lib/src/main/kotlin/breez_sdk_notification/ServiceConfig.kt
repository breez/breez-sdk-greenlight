package breez_sdk_notification

data class ServiceConfig(
    /** Automatic channel setup fee limit (in msats) */
    val autoChannelSetupFeeLimitMsats: ULong = defaultAutoChannelSetupFeeLimitMsats,
) {
    companion object {
        /** Automatic channel setup fee limit (in msats) for the default ServiceConfig */
        const val defaultAutoChannelSetupFeeLimitMsats: ULong = 0u

        /** The default ServiceConfig */
        fun default(): ServiceConfig = ServiceConfig(defaultAutoChannelSetupFeeLimitMsats)
    }
}