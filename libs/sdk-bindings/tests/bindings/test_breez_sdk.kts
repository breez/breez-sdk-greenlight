
import breez_sdk.BreezEvent

class SDKListener : breez_sdk.EventListener {
    override fun onEvent(e: BreezEvent) {
        println(e.toString())
    }
}

class LogStreamListener : breez_sdk.LogStream {
    override fun log(l: breez_sdk.LogEntry) {
        if (l.level != "TRACE") {
            println(l.line)
        }
    }
}

try {
    breez_sdk.setLogStream(LogStreamListener(), breez_sdk.LevelFilter.TRACE)
    var seed = breez_sdk.mnemonicToSeed("repeat hawk combine screen network rhythm ritual social neither casual volcano powder")
    var config =
        breez_sdk.defaultConfig(
            breez_sdk.EnvironmentType.PRODUCTION,
            "code",
            breez_sdk.NodeConfig.Greenlight(breez_sdk.GreenlightNodeConfig(null, null)),
        )
    var connectRequest = breez_sdk.ConnectRequest(config, seed)
    var sdkServices = breez_sdk.connect(connectRequest, SDKListener())
    var nodeInfo = sdkServices.nodeInfo()
    assert(nodeInfo.id.equals("027e2b899f9f75b92a1ad210da21d74e7314e3499375213a71c6bf3e1b4b4394a1"))
} catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}
