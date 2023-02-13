import { NativeModules, NativeEventEmitter, Platform } from "react-native"
import type Long from "long"

const LINKING_ERROR =
    `The package 'react-native-breez-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezSDK = NativeModules.BreezSDK
    ? NativeModules.BreezSDK
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

enum InputType {
    BITCOIN_ADDRESS = "bitcoinAddress",
    BOLT11 = "bolt11",
    LNURL_AUTH = "lnUrlAuth",
    LNURL_ERROR = "lnUrlError",
    LNURL_PAY = "lnUrlPay",
    LNURL_WITHDRAW = "lnUrlWithdraw",
    NODE_ID = "nodeId",
    URL = "url"
}

const BreezSDKEmitter = new NativeEventEmitter(BreezSDK)

export enum Network {
    BITCOIN = "bitcoin",
    REGTEST = "regtest",
    SIGNET = "signet",
    TESTNET = "testnet"
}

export type BitcoinAddressData = {
    address: string
    network: Network
    amountSat?: Long
    label?: string
    message?: string
}

export type LnInvoice = {
    bolt11: string
    payeePubkey: string
    paymentHash: string
    description?: string
    descriptionHash?: string
    amountMsat?: Long
    timestamp: Long
    expiry: Long
    routingHints: RouteHint[]
    paymentSecret?: Uint8Array
}

export type LnUrlAuthData = {
    k1: string
}

export type LnUrlErrorData = {
    reason: string
}

export type LnUrlPayRequestData = {
    callback: string
    minSendable: Long
    maxSendable: Long
    metadataStr: string
    commentAllowed: number
}

export type LnUrlWithdrawRequestData = {
    callback: string
    k1: string
    defaultDescription: string
    minWithdrawable: Long
    maxWithdrawable: Long
}

export type RouteHint = {
    hops: RouteHintHops[]
}

export type LogEntry = {
    line: string
    level: string
}

export type LogEntryFn = (l: LogEntry) => void

export type RouteHintHops = {
    srcNodeId: string
    shortChannelId: Long
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: Long
    htlcMinimumMsat?: Long
    htlcMaximumMsat: Long
}

export async function parseInput(
    input: string
): Promise<BitcoinAddressData | LnInvoice | LnUrlAuthData | LnUrlErrorData | LnUrlPayRequestData | LnUrlWithdrawRequestData | string> {
    const response = await BreezSDK.parseInput(input)
    console.log(JSON.stringify(response))

    switch (response.type) {
        case InputType.BITCOIN_ADDRESS:
            return response.data as BitcoinAddressData
        case InputType.BOLT11:
            return response.data as LnInvoice
        case InputType.LNURL_AUTH:
            return response.data as LnUrlAuthData
        case InputType.LNURL_ERROR:
            return response.data as LnUrlErrorData
        case InputType.LNURL_PAY:
            return response.data as LnUrlPayRequestData
        case InputType.LNURL_WITHDRAW:
            return response.data as LnUrlWithdrawRequestData
        case InputType.NODE_ID:
        case InputType.URL:
            return response.data
    }

    return response
}

export async function parseInvoice(invoice: string): Promise<LnInvoice> {
    const response = await BreezSDK.parseInvoice(invoice)
    console.log(JSON.stringify(response))

    return response
}

export async function mnemonicToSeed(phrase: string): Promise<Uint8Array> {
    const response = await BreezSDK.mnemonicToSeed(phrase)
    console.log(JSON.stringify(response))

    return response
}

export async function setLogStream(logEntryFn: LogEntryFn): Promise<void> {
    BreezSDKEmitter.addListener("breezSdkLog", logEntryFn)

    const response = await BreezSDK.startLogStream()
    console.log(JSON.stringify(response))

    return
}
