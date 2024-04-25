import XCTest
@testable import BreezSDK

class SDKListener: EventListener {
    func onEvent(e: BreezEvent) {
        print("received event ", e);
    }
}

final class BreezSDKTests: XCTestCase {
    func testExample() throws {
        let seedPhraseForTesting = "repeat hawk combine screen network rhythm ritual social neither casual volcano powder"
        let seedForTesting = try mnemonicToSeed(phrase: seedPhraseForTesting)
        let credentials = try recoverNode(network: Network.bitcoin, seed: seedForTesting)
        
        let sdkServices = try initServices(
            config: BreezSDK.defaultConfig(envType: EnvironmentType.production),
            seed: seedForTesting,
            creds: credentials,
            listener: SDKListener()
        )
        
        try sdkServices.start()
        let nodeInfo = try sdkServices.nodeInfo()
        
        XCTAssertEqual(nodeInfo.id, "0352a918bdba7d7a69893a7d52a449f3e6df8e15a0edcc7fe59060be70d6f416f0")
    }
}
