
import breez_sdk

class SDKListener: EventListener {
    func onEvent(e: BreezEvent) {
        print("received event ", e);
    }
}

class LogStreamListener: LogStream {
    func log(l: LogEntry){
        if l.level != "TRACE" {
            print(l.line);
        }
    }
}

do {
    try setLogStream(logStream: LogStreamListener(), filterLevel: LevelFilter.trace);
    let seed = try mnemonicToSeed(phrase: "repeat hawk combine screen network rhythm ritual social neither casual volcano powder");
    let config = breez_sdk.defaultConfig(envType: EnvironmentType.production, apiKey: "code", nodeConfig: NodeConfig.greenlight(config: GreenlightNodeConfig(partnerCredentials: nil, inviteCode: nil)));
    let connectRequest = ConnectRequest(config: config, seed: seed)
    let sdkServices = try connect(req: connectRequest, listener: SDKListener());    
    let nodeInfo = try sdkServices.nodeInfo();
    assert(nodeInfo.id == "027e2b899f9f75b92a1ad210da21d74e7314e3499375213a71c6bf3e1b4b4394a1", "nodeInfo.id");
}catch {
 fatalError("Should have not thrown!")
}