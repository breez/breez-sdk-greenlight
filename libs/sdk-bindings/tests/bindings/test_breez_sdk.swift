
import breez_sdk

class SDKListener: EventListener {
    func onEvent(e: BreezEvent) {
        print("received event ", e);
    }
}

do {
    let seed = try mnemonicToSeed(phrase: "cruise clever syrup coil cute execute laundry general cover prevent law sheriff");
    let credentials = try recoverNode(network: Network.bitcoin, seed: seed, config: nil);
    let sdkServices = try start(config: nil, seed: seed, creds: credentials, listener: SDKListener())
    let nodeInfo = try sdkServices.nodeInfo();
    assert(nodeInfo?.id == "0352a918bdba7d7a69893a7d52a449f3e6df8e15a0edcc7fe59060be70d6f416f0", "nodeInfo.id");
}catch {
 fatalError("Should have not thrown!")
}