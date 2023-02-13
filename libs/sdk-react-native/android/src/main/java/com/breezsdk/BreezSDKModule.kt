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
            promise.resolve(readableArrayOf(seed));
        } catch (e: SdkException) {
            e.printStackTrace();
            promise.reject(e);
        }
    }

    @ReactMethod
    fun parseInput(input: String, promise: Promise) {
        try {
            var inputType = parseInput(input);
            promise.resolve(readableMapOf(inputType));
        } catch (e: SdkException) {
            e.printStackTrace();
            promise.reject(e);
        }
    }

    @ReactMethod
    fun parseInvoice(invoice: String, promise: Promise) {
        try {
            var lnInvoice = parseInvoice(invoice);
            promise.resolve(readableMapOf(lnInvoice));
        } catch (e: SdkException) {
            e.printStackTrace();
            promise.reject(e);
        }
    }
}
