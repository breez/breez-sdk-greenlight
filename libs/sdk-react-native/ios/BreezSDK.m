#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(BreezSDK, RCTEventEmitter)

RCT_EXTERN_METHOD(
    registerNode: (NSString*)network
    seed: (NSArray*)seed
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    recoverNode: (NSString*)network
    seed: (NSArray*)seed
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    initServices: (NSString*)apiKey
    deviceKey: (NSArray*)deviceKey
    deviceCert: (NSArray*)deviceCert
    seed: (NSArray*)seed
    resolver: (RCTPromiseResolveBlock)resolve
    rejecter: (RCTPromiseRejectBlock)reject
)

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

@end
