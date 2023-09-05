#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(RNBreezSDK, RCTEventEmitter)

RCT_EXTERN_METHOD(
    mnemonicToSeed: (NSString*)phrase
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    parseInput: (NSString*)input
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    parseInvoice: (NSString*)invoice
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    startLogStream: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    defaultConfig: (NSString*)envType
    apiKey:(NSString*)apiKey
    nodeConfigMap: (NSDictionary*)nodeConfigMap
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    connect: (NSDictionary*)config
    seed: (NSArray*)seed
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sync: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    disconnect: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sendPayment: (NSString*)bolt11
    amountSats: (NSUInteger*)amountSats
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sendSpontaneousPayment: (NSString*)nodeId
    amountSats: (NSUInteger*)amountSats
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    openChannelFee: (NSDictionary*)reqData
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    receivePayment: (NSDictionary*)reqData
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    lnurlAuth: (NSDictionary*)reqData
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    payLnurl: (NSDictionary*)reqData
    amountSats: (NSUInteger*)amountSats
    comment: (NSString*)comment
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    withdrawLnurl: (NSDictionary*)reqData
    amountSats: (NSUInteger*)amountSats
    description: (NSString*)description
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    nodeInfo: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    paymentByHash: (NSString*)hash
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listPayments: (NSString*)filter
    fromTimestamp: (NSInteger*)fromTimestamp
    toTimestamp: (NSInteger*)toTimestamp
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sweep: (NSString*)toAddress
    feeRateSatsPerVbyte: (NSUInteger*)feeRateSatsPerVbyte
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    fetchFiatRates: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listFiatCurrencies: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listLsps: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    connectLsp: (NSString*)lspId
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    fetchLspInfo: (NSString*)lspId
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    lspId: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    lspInfo: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    closeLspChannels: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    receiveOnchain: (NSDictionary*)req
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    inProgressSwap: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listRefundables: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    refund: (NSString*)swapAddress
    toAddress: (NSString*)toAddress
    satPerVbyte: (NSUInteger*)satPerVbyte
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    fetchReverseSwapFees: (NSDictionary*)reqData
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    inProgressReverseSwaps: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sendOnchain: (NSUInteger*)amountSat
    onchainRecipientAddress: (NSString*)onchainRecipientAddress
    pairHash: (NSString*)pairHash
    satPerVbyte: (NSUInteger*)satPerVbyte
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    executeDevCommand: (NSString*)command
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    recommendedFees: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    buyBitcoin: (NSDictionary*)req
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    backup: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    backupStatus: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    signMessage: (NSDictionary*)reqData
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    checkMessage: (NSDictionary*)reqData
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

@end
