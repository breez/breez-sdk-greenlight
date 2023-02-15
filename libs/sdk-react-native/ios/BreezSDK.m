#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(BreezSDK, RCTEventEmitter)

RCT_EXTERN_METHOD(
    initServices: (NSArray*)deviceKey
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

@end
