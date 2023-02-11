package com.breezsdk;

import breez_sdk.*;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

class BreezSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {  
    override fun getName(): String {
        return "BreezSDK";
    }

    @ReactMethod
    fun mnemonicToSeed(mnemonic: String, promise: Promise) {
        try {
            var seed = mnemonicToSeed(mnemonic);
            var response = writableMapOf("type" to "seed", "data" to seed);

            promise.resolve(response);
        } catch (e: SdkException) {
            e.printStackTrace();
            promise.reject("SdkException");
        }
    }
}
