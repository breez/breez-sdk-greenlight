import { NativeModules, NativeEventEmitter, Platform } from "react-native"

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

const BreezSDKEmitter = new NativeEventEmitter(BreezSDK)

enum EventType {
    INVOICE_PAID = "invoicePaid",
    NEW_BLOCK = "newBlock",
    PAYMENT_SUCCEED = "paymentSucceed",
    PAYMENT_FAILED = "paymentFailed",
    SYNCED = "synced"
}

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

export enum PaymentType {
    SEND = "send",
    RECEIVED = "received",
    CLOSED_CHANNEL = "closed_channel"
}

enum PaymentDetailType {
    LN = "ln",
    CLOSED_CHANNEL = "closed_channel"
}

export enum Network {
    BITCOIN = "bitcoin",
    REGTEST = "regtest",
    SIGNET = "signet",
    TESTNET = "testnet"
}

enum SuccessActionDataType {
    AES = "aes",
    MESSAGE = "message",
    URL = "url"
}

export type AesSuccessActionDataDecrypted = {
    description: string
    plaintext: string
}

export type BitcoinAddressData = {
    address: string
    network: Network
    amountSat?: number
    label?: string
    message?: string
}

export type ClosedChannelPaymentDetails = {
    shortChannelId: string
    state: string
    fundingTxid: string
}

export type GreenlightCredentials = {
    deviceKey: Uint8Array
    deviceCert: Uint8Array
}

export type InvoicePaidDetails = {
    paymentHash: string
    bolt11: string
}

export type LnInvoice = {
    bolt11: string
    payeePubkey: string
    paymentHash: string
    description?: string
    descriptionHash?: string
    amountMsat?: number
    timestamp: number
    expiry: number
    routingHints: RouteHint[]
    paymentSecret?: Uint8Array
}

export type LogEntry = {
    line: string
    level: string
}

export type EventFn = (type: EventType, data?: InvoicePaidDetails | Payment | number | string) => void

export type LogEntryFn = (l: LogEntry) => void

export type LnPaymentDetails = {
    paymentHash: string
    label: string
    destinationPubkey: string
    paymentPreimage: string
    keysend: boolean
    bolt11: string
    lnurlSuccessAction?: AesSuccessActionDataDecrypted | MessageSuccessActionData | UrlSuccessActionData
}

export type LnUrlAuthData = {
    k1: string
}

export type LnUrlErrorData = {
    reason: string
}

export type LnUrlPayRequestData = {
    callback: string
    minSendable: number
    maxSendable: number
    metadataStr: string
    commentAllowed: number
}

export type LnUrlWithdrawCallbackStatus = {
    status: string
    reason?: string
}

export type LnUrlWithdrawRequestData = {
    callback: string
    k1: string
    defaultDescription: string
    minWithdrawable: number
    maxWithdrawable: number
}

export type MessageSuccessActionData = {
    message: string
}

export type NodeId = string

export type NodeState = {
    id: string
    blockHeight: number
    channelsBalanceMsat: number
    onchainBalanceMsat: number
    utxos: UnspentTransactionOutput[]
    maxPayableMsat: number
    maxReceivableMsat: number
    maxSinglePaymentAmountMsat: number
    maxChanReserveMsats: number
    connectedPeers: string[]
    inboundLiquidityMsats: number
}

export type Payment = {
    id: string
    paymentType: PaymentType
    paymentTime: number
    amountMsat: number
    feeMsat: number
    pending: boolean
    description?: string
    details: LnPaymentDetails | ClosedChannelPaymentDetails
}

export type RouteHint = {
    hops: RouteHintHops[]
}

export type RouteHintHops = {
    srcNodeId: string
    shortChannelId: number
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: number
    htlcMinimumMsat?: number
    htlcMaximumMsat: number
}

export type Url = string

export type UrlSuccessActionData = {
    description: string
    url: string
}

export type UnspentTransactionOutput = {
    txid: Uint8Array
    outnum: number
    amountMillisatoshi: number
    address: string
    reserved: boolean
    reservedToBlock: number
}

function eventProcessor(eventFn: EventFn) {
    return (event: any) => {
        switch (event.type) {
            case EventType.INVOICE_PAID:
                return eventFn(EventType.INVOICE_PAID, event.data as InvoicePaidDetails)
            case EventType.NEW_BLOCK:
                return eventFn(EventType.NEW_BLOCK, event.data)
            case EventType.PAYMENT_FAILED:
                return eventFn(EventType.PAYMENT_FAILED, event.data)
            case EventType.PAYMENT_SUCCEED:
                const payment = event.data as Payment

                switch (event.data.details.type) {
                    case PaymentDetailType.CLOSED_CHANNEL:
                        payment.details = event.data.details as ClosedChannelPaymentDetails
                        break
                    case PaymentDetailType.LN:
                        payment.details = event.data.details as LnPaymentDetails

                        if (event.data.details.lnurlSuccessAction) {
                            switch (event.data.details.lnurlSuccessAction.type) {
                                case SuccessActionDataType.AES:
                                    payment.details.lnurlSuccessAction = event.data.details.lnurlSuccessAction as AesSuccessActionDataDecrypted
                                    break
                                case SuccessActionDataType.MESSAGE:
                                    payment.details.lnurlSuccessAction = event.data.details.lnurlSuccessAction as MessageSuccessActionData
                                    break
                                case SuccessActionDataType.URL:
                                    payment.details.lnurlSuccessAction = event.data.details.lnurlSuccessAction as UrlSuccessActionData
                                    break
                            }
                        }
                        break
                }

                return eventFn(EventType.PAYMENT_SUCCEED, payment)
            case EventType.SYNCED:
                return eventFn(EventType.SYNCED)
        }
    }
}

export async function addEventListener(eventFn: EventFn) {
    BreezSDKEmitter.addListener("breezSdkEvent", eventProcessor(eventFn))
}

export async function addLogListener(logEntryFn: LogEntryFn): Promise<void> {
    BreezSDKEmitter.addListener("breezSdkLog", logEntryFn)

    await BreezSDK.startLogStream()
}

export async function mnemonicToSeed(phrase: string): Promise<Uint8Array> {
    return BreezSDK.mnemonicToSeed(phrase)
}

export async function parseInput(
    input: string
): Promise<BitcoinAddressData | LnInvoice | LnUrlAuthData | LnUrlErrorData | LnUrlPayRequestData | LnUrlWithdrawRequestData | NodeId | Url> {
    const response = await BreezSDK.parseInput(input)

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
            return response.data as NodeId
        case InputType.URL:
            return response.data as Url
    }

    throw Error(`Unknown input type: ${response.type}`)
}

export async function parseInvoice(invoice: string): Promise<LnInvoice> {
    const response = await BreezSDK.parseInvoice(invoice)
    return response as LnInvoice
}

export async function registerNode(network: Network, seed: Uint8Array): Promise<GreenlightCredentials> {
    const response = await BreezSDK.registerNode(network, seed)
    return response as GreenlightCredentials
}

export async function recoverNode(network: Network, seed: Uint8Array): Promise<GreenlightCredentials> {
    const response = await BreezSDK.recoverNode(network, seed)
    return response as GreenlightCredentials
}

export async function initServices(apiKey: string, deviceKey: Uint8Array, deviceCert: Uint8Array, seed: Uint8Array): Promise<void> {
    await BreezSDK.initServices(apiKey, deviceKey, deviceCert, seed)
}

export async function start(): Promise<void> {
    await BreezSDK.start()
}

export async function sync(): Promise<void> {
    await BreezSDK.sync()
}

export async function stop(): Promise<void> {
    await BreezSDK.stop()
}

export async function sendPayment(bolt11: string, amountSats?: number): Promise<Payment> {
    const response = await BreezSDK.sendPayment(bolt11, amountSats || 0)
    return response as Payment
}

export async function sendSpontaneousPayment(nodeId: string, amountSats: number): Promise<Payment> {
    const response = await BreezSDK.sendSpontaneousPayment(nodeId, amountSats)
    return response as Payment
}

export async function receivePayment(amountSats: number, description: string): Promise<LnInvoice> {
    const response = await BreezSDK.receivePayment(amountSats, description)
    return response as LnInvoice
}

export async function withdrawLnurl(
    reqData: LnUrlWithdrawRequestData,
    amountSats: number,
    description?: string
): Promise<LnUrlWithdrawCallbackStatus> {
    const response = await BreezSDK.withdrawLnurl(reqData, amountSats, description)
    return response as LnUrlWithdrawCallbackStatus
}

export async function nodeInfo(): Promise<NodeState> {
    const response = await BreezSDK.nodeInfo()
    return response as NodeState
}
