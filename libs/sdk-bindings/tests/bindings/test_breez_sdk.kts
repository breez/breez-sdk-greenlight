
import breez_sdk.BreezEvent

class SDKListener: breez_sdk.EventListener {
    override fun onEvent(e: BreezEvent) {
        println(e.toString());
    }
}

class SDKNodeLogger: breez_sdk.Logger {
    override fun log(l: breez_sdk.LogMessage) {
        if (l.level != breez_sdk.LogLevel.TRACE) {
            println(l.message);
        }
    }
}

class LogStreamListener: breez_sdk.LogStream {
    override fun log(l: breez_sdk.LogEntry) {
        if (l.level != "TRACE") {
            println(l.line);
        }
    }
}

try {
    var seed = breez_sdk.mnemonicToSeed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff");
    var config = breez_sdk.defaultConfig(breez_sdk.EnvironmentType.STAGING, "code", breez_sdk.NodeConfig.Greenlight(breez_sdk.GreenlightNodeConfig(null, "")))
    var sdkServices = breez_sdk.connect(config, seed, SDKListener(), SDKNodeLogger(), null);
    var nodeInfo = sdkServices.nodeInfo();
    assert(nodeInfo.id.equals("0352a918bdba7d7a69893a7d52a449f3e6df8e15a0edcc7fe59060be70d6f416f0"));
}catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}