import { NativeModules, Platform, EmitterSubscription, NativeEventEmitter } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-breez-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezSDK = NativeModules.RNBreezSDK
    ? NativeModules.RNBreezSDK
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

const BreezSDKEmitter = new NativeEventEmitter(BreezSDK)

export enum EnvironmentType {
    PRODUCTION = "production",
    STAGING = "staging"
}

export enum EventType {
    INVOICE_PAID = "invoicePaid",
    NEW_BLOCK = "newBlock",
    PAYMENT_SUCCEED = "paymentSucceed",
    PAYMENT_FAILED = "paymentFailed",
    SYNCED = "synced",
    BACKUP_STARTED = "backupStarted",
    BACKUP_SUCCEEDED = "backupSucceeded",
    BACKUP_FAILED = "backupFailed"    
}

export enum InputType {
    BITCOIN_ADDRESS = "bitcoinAddress",
    BOLT11 = "bolt11",
    LNURL_AUTH = "lnUrlAuth",
    LNURL_ERROR = "lnUrlError",
    LNURL_PAY = "lnUrlPay",
    LNURL_WITHDRAW = "lnUrlWithdraw",
    NODE_ID = "nodeId",
    URL = "url"
}

export enum LnUrlPayResultType {
    ENDPOINT_SUCCESS = "endpointSuccess",
    ENDPOINT_ERROR = "endpointError"
}

export enum PaymentType {
    SEND = "send",
    RECEIVED = "received",
    CLOSED_CHANNEL = "closed_channel"
}

export enum PaymentDetailType {
    LN = "ln",
    CLOSED_CHANNEL = "closed_channel"
}

export enum PaymentTypeFilter {
    SENT = "sent",
    RECEIVED = "received",
    ALL = "all"
}

export enum Network {
    BITCOIN = "bitcoin",
    REGTEST = "regtest",
    SIGNET = "signet",
    TESTNET = "testnet"
}

export enum SuccessActionDataType {
    AES = "aes",
    MESSAGE = "message",
    URL = "url"
}

export enum SwapStatus {
    INITIAL = "initial",
    EXPIRED = "expired"
}

export enum BuyBitcoinProvider {
    MOONPAY = "moonpay"
}

export enum ReverseSwapStatus {
    INITIAL = "initial",
    IN_PROGRESS = "in_progress",
    CANCELLED = "cancelled",
    COMPLETED_SEEN = "completed_seen",
    COMPLETED_CONFIRMED = "completed_confirmed"
}

export type AesSuccessActionDataDecrypted = {
    type: string
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

export type Config = {
    breezserver: string
    mempoolspaceUrl: string
    workingDir: string
    network: Network
    paymentTimeoutSec: number
    defaultLspId?: string
    apiKey?: string
    maxfeePercent: number
}

export type ClosedChannelPaymentDetails = {
    shortChannelId: string
    state: string
    fundingTxid: string
}

export type CurrencyInfo = {
    name: string
    fractionSize: number
    spacing: number
    symbol?: Symbol
    uniqSymbol?: Symbol
    localizedName?: LocalizedName[]
    localeOverrides?: LocaleOverrides[]
}

export type EventData = InvoicePaidDetails | Payment | number | PaymentFailedData | BackupFailedData

export type EventFn = (type: EventType, data?: EventData) => void

export type GreenlightCredentials = {
    deviceKey: Uint8Array
    deviceCert: Uint8Array
}

export type FiatCurrency = {
    id: string
    info: CurrencyInfo
}

export type InputResponse = {
    type: InputType
    data: BitcoinAddressData | LnInvoice | LnUrlAuthRequestData | LnUrlErrorData | LnUrlPayRequestData | LnUrlWithdrawRequestData | NodeId | Url
}

export type InvoicePaidDetails = {
    paymentHash: string
    bolt11: string
}

export type PaymentFailedData = {
    error: string
    invoice?: LnInvoice
    nodeId: string
}

export type BackupFailedData = {
    error: string
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

export type LogEntryFn = (l: LogEntry) => void

export type LnPaymentDetails = {
    paymentHash: string
    label: string
    destinationPubkey: string
    paymentPreimage: string
    keysend: boolean
    bolt11: string
    lnurlSuccessAction?: AesSuccessActionDataDecrypted | MessageSuccessActionData | UrlSuccessActionData
    lnurlMetadata?: string
    lnAddress?: string
}

export type LnUrlAuthRequestData = {
    k1: string
    action?: string
    domain: string
    url: string
}

export type LnUrlCallbackStatus = {
    status: string
    reason?: string
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
    domain: string
    lnAddress?: string
}

export type LnUrlPayResult = {
    type: LnUrlPayResultType
    data?: AesSuccessActionDataDecrypted | LnUrlErrorData | MessageSuccessActionData | UrlSuccessActionData
}

export type LnUrlWithdrawRequestData = {
    callback: string
    k1: string
    defaultDescription: string
    minWithdrawable: number
    maxWithdrawable: number
}

export type LocaleOverrides = {
    locale: string
    spacing?: number
    symbol: Symbol
}

export type LocalizedName = {
    locale: string
    name: string
}

export type LspInformation = {
    id: string
    name: string
    widgetUrl: string
    pubkey: string
    host: string
    channelCapacity: number
    targetConf: number
    baseFeeMsat: number
    feeRate: number
    timeLockDelta: number
    minHtlcMsat: number
    channelFeePermyriad: number
    lspPubkey: Uint8Array
    maxInactiveDuration: number
    channelMinimumFeeMsat: number
}

export type MessageSuccessActionData = {
    type: string
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

export type Rate = {
    coin: string
    value: number
}

export type RecommendedFees = {
    fastestFee: number
    halfHourFee: number
    hourFee: number
    economyFee: number
    minimumFee: number
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

export type SwapInfo = {
    bitcoinAddress: string
    createdAt: number
    lockHeight: number
    paymentHash: Uint8Array
    preimage: Uint8Array
    privateKey: Uint8Array
    publicKey: Uint8Array
    swapperPublicKey: Uint8Array
    script: Uint8Array
    bolt11?: string
    paidSats: number
    unconfirmedSats: number
    confirmedSats: number
    status: SwapStatus
    refundTxIds: string[]
    unconfirmedTxIds: string[]
    confirmedTxIds: string[]
    minAllowedDeposit: number
    maxAllowedDeposit: number
    lastRedeemError?: string
}

export type ReverseSwapPairInfo = {
    min: number
    max: number
    feesHash: string
    feesPercentage: number
    feesLockup: number
    feesClaim: number
}

export type ReverseSwapInfo = {
    id: string
    claimPubkey: string
    onchainAmountSat: number
    status: ReverseSwapStatus
}

export type Symbol = {
    grapheme?: string
    template?: string
    rtl?: boolean
    position?: number
}

export type Url = string

export type UrlSuccessActionData = {
    type: string
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

export type BackupStatus = {
    backedUp: boolean
    lastBackupTime?: number   
}

const processEvent = (eventFn: EventFn) => {
    return (event: any) => {
        switch (event.type) {
            case EventType.INVOICE_PAID:
                return eventFn(EventType.INVOICE_PAID, event.data as InvoicePaidDetails)
            case EventType.NEW_BLOCK:
                return eventFn(EventType.NEW_BLOCK, event.data)
            case EventType.PAYMENT_FAILED:
                return eventFn(EventType.PAYMENT_FAILED, event.data as PaymentFailedData)
            case EventType.PAYMENT_SUCCEED:
                const payment = event.data as Payment

                switch (event.data.details.type) {
                    case PaymentDetailType.CLOSED_CHANNEL:
                        payment.details = event.data.details as ClosedChannelPaymentDetails
                        break
                    case PaymentDetailType.LN:
                        payment.details = event.data.details as LnPaymentDetails
                        payment.details.lnurlSuccessAction = processSuccessActionProcessed(event.data.details.lnurlSuccessAction)
                        break
                }

                return eventFn(EventType.PAYMENT_SUCCEED, payment)
            case EventType.SYNCED:
                return eventFn(EventType.SYNCED)
            case EventType.BACKUP_STARTED:
                return eventFn(EventType.BACKUP_STARTED)
            case EventType.BACKUP_SUCCEEDED:
                return eventFn(EventType.BACKUP_SUCCEEDED)
            case EventType.BACKUP_FAILED:
                return eventFn(EventType.BACKUP_FAILED, event.data as BackupFailedData)
        }
    }
}

const processSuccessActionProcessed = (data: any): AesSuccessActionDataDecrypted | MessageSuccessActionData | UrlSuccessActionData | undefined => {
    switch (data.type) {
        case SuccessActionDataType.AES:
            return data as AesSuccessActionDataDecrypted
        case SuccessActionDataType.MESSAGE:
            return data as MessageSuccessActionData
        case SuccessActionDataType.URL:
            return data as UrlSuccessActionData
    }

    return
}

export const addEventListener = (eventFn: EventFn): EmitterSubscription => {
    return BreezSDKEmitter.addListener("breezSdkEvent", processEvent(eventFn))
}

export const addLogListener = async (logEntryFn: LogEntryFn): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkLog", logEntryFn)

    await BreezSDK.startLogStream()

    return subscription
}

export const mnemonicToSeed = async (phrase: string): Promise<Uint8Array> => {
    return BreezSDK.mnemonicToSeed(phrase)
}

export const parseInput = async (input: string): Promise<InputResponse> => {
    const response = await BreezSDK.parseInput(input)
    return response as InputResponse
}

export const parseInvoice = async (invoice: string): Promise<LnInvoice> => {
    const response = await BreezSDK.parseInvoice(invoice)
    return response as LnInvoice
}

export const registerNode = async (
    network: Network,
    seed: Uint8Array,
    registerCreds?: GreenlightCredentials,
    inviteCode: string = ""
): Promise<GreenlightCredentials> => {
    const response = await BreezSDK.registerNode(
        network,
        seed,
        registerCreds
            ? {
                  deviceCert: Array.from(registerCreds.deviceCert),
                  deviceKey: Array.from(registerCreds.deviceKey)
              }
            : {},
        inviteCode
    )
    return response as GreenlightCredentials
}

export const recoverNode = async (network: Network, seed: Uint8Array): Promise<GreenlightCredentials> => {
    const response = await BreezSDK.recoverNode(network, seed)
    return response as GreenlightCredentials
}

export const defaultConfig = async (envType: EnvironmentType): Promise<Config> => {
    const response = await BreezSDK.defaultConfig(envType)
    return response as Config
}

export const initServices = async (config: Config, deviceKey: Uint8Array, deviceCert: Uint8Array, seed: Uint8Array): Promise<void> => {
    await BreezSDK.initServices(config, deviceKey, deviceCert, seed)
}

export const start = async (): Promise<void> => {
    await BreezSDK.start()
}

export const sync = async (): Promise<void> => {
    await BreezSDK.sync()
}

export const stop = async (): Promise<void> => {
    await BreezSDK.stop()
}

export const sendPayment = async (bolt11: string, amountSats: number = 0): Promise<Payment> => {
    const response = await BreezSDK.sendPayment(bolt11, amountSats)
    return response as Payment
}

export const sendSpontaneousPayment = async (nodeId: string, amountSats: number): Promise<Payment> => {
    const response = await BreezSDK.sendSpontaneousPayment(nodeId, amountSats)
    return response as Payment
}

export const receivePayment = async (amountSats: number, description: string): Promise<LnInvoice> => {
    const response = await BreezSDK.receivePayment(amountSats, description)
    return response as LnInvoice
}

export const lnurlAuth = async (reqData: LnUrlAuthRequestData): Promise<LnUrlCallbackStatus> => {
    const response = await BreezSDK.lnurlAuth(reqData)
    return response as LnUrlCallbackStatus
}

export const payLnurl = async (reqData: LnUrlPayRequestData, amountSats: number, comment: string = ""): Promise<LnUrlPayResult> => {
    const response = await BreezSDK.payLnurl(reqData, amountSats, comment)
    return response as LnUrlPayResult
}

export const withdrawLnurl = async (
    reqData: LnUrlWithdrawRequestData,
    amountSats: number,
    description: string = ""
): Promise<LnUrlCallbackStatus> => {
    const response = await BreezSDK.withdrawLnurl(reqData, amountSats, description)
    return response as LnUrlCallbackStatus
}

export const nodeInfo = async (): Promise<NodeState> => {
    const response = await BreezSDK.nodeInfo()
    return response as NodeState
}

export const listPayments = async (filter: PaymentTypeFilter, fromTimestamp: number = 0, toTimestamp: number = 0): Promise<Payment[]> => {
    const response = await BreezSDK.listPayments(filter, fromTimestamp, toTimestamp)
    return response as Payment[]
}

export const sweep = async (toAddress: string, feeRateSatsPerVbyte: number): Promise<void> => {
    await BreezSDK.sweep(toAddress, feeRateSatsPerVbyte)
}

export const fetchFiatRates = async (): Promise<Rate[]> => {
    const response = await BreezSDK.fetchFiatRates()
    return response as Rate[]
}

export const listFiatCurrencies = async (): Promise<FiatCurrency[]> => {
    const response = await BreezSDK.listFiatCurrencies()
    return response as FiatCurrency[]
}

export const listLsps = async (): Promise<LspInformation[]> => {
    const response = await BreezSDK.listLsps()
    return response as LspInformation[]
}

export const connectLsp = async (lspId: string): Promise<void> => {
    await BreezSDK.connectLsp(lspId)
}

export const fetchLspInfo = async (lspId: string): Promise<LspInformation> => {
    const response = await BreezSDK.fetchLspInfo(lspId)
    return response as LspInformation
}

export const lspId = async (): Promise<string> => {
    const response = await BreezSDK.lspId()
    return response
}

export const closeLspChannels = async (): Promise<void> => {
    await BreezSDK.closeLspChannels()
}

export const receiveOnchain = async (): Promise<SwapInfo> => {
    const response = await BreezSDK.receiveOnchain()
    return response as SwapInfo
}

export const inProgressSwap = async (): Promise<SwapInfo> => {
    const response = await BreezSDK.inProgressSwap()
    return response as SwapInfo
}

export const listRefundables = async (): Promise<SwapInfo[]> => {
    const response = await BreezSDK.listRefundables()
    return response as SwapInfo[]
}

export const refund = async (swapAddress: string, toAddress: string, satPerVbyte: number): Promise<string> => {
    const response = await BreezSDK.refund(swapAddress, toAddress, satPerVbyte)
    return response
}

export const fetchReverseSwapFees = async (): Promise<ReverseSwapPairInfo> => {
    const response = await BreezSDK.fetchReverseSwapFees()
    return response as ReverseSwapPairInfo
}

export const inProgressReverseSwaps = async (): Promise<ReverseSwapInfo[]> => {
    const response = await BreezSDK.inProgressReverseSwaps()
    return response as ReverseSwapInfo[]
}

export const sendOnchain = async (amountSat: number, onchainRecipientAddress: string, pairHash: string, satPerVbyte: number): Promise<ReverseSwapInfo> => {
    const response = await BreezSDK.sendOnchain(amountSat, onchainRecipientAddress, pairHash, satPerVbyte)
    return response as ReverseSwapInfo
}

export const executeDevCommand = async (command: string): Promise<string> => {
    const response = await BreezSDK.executeDevCommand(command)
    return response
}

export const recommendedFees = async (): Promise<RecommendedFees> => {
    const response = await BreezSDK.recommendedFees()
    return response as RecommendedFees
}

export const buyBitcoin = async (provider: BuyBitcoinProvider): Promise<string> => {
    const response = await BreezSDK.buyBitcoin(provider)
    return response
}

export const backup = async (): Promise<void> => {
 await BreezSDK.backup() 
}

export const backupStatus = async (): Promise<BackupStatus> => {
 const response = await BreezSDK.backupStatus()
 return response
}
