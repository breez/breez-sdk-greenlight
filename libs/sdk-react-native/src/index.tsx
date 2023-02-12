import { NativeModules, Platform } from "react-native"
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

type LnInvoice = {
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

type RouteHint = {
    hops: RouteHintHops[]
}

type RouteHintHops = {
    srcNodeId: string
    shortChannelId: Long
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: Long
    htlcMinimumMsat?: Long
    htlcMaximumMsat: Long
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
