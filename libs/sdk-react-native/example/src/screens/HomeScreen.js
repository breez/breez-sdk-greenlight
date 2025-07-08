import React, { useEffect, useState } from "react"
import { Button, Platform, SafeAreaView, ScrollView } from "react-native"
import {
    backup,
    backupStatus,
    buyBitcoin,
    BuyBitcoinProvider,
    checkMessage,
    connect,
    defaultConfig,
    EnvironmentType,
    fetchFiatRates,
    fetchReverseSwapFees,
    inProgressOnchainPayments,
    lspInfo,
    listFiatCurrencies,
    mnemonicToSeed,
    NodeConfigVariant,
    nodeInfo,
    openChannelFee,
    receivePayment,
    setLogStream,
    signMessage
} from "@breeztech/react-native-breez-sdk"
import BuildConfig from "react-native-build-config"
import { FileLogger } from "react-native-file-logger"
import DebugLine from "../components/DebugLine"
import { Log } from "../utils/logging"
import { obfuscateString } from "../utils/security"
import { getSecureItem, setSecureItem } from "../utils/storage"

const log = new Log("ExampleApp")

const DEBUG = Platform.select({
    android: BuildConfig.DEBUG,
    ios: BuildConfig.DEBUG === "true"
})

const MNEMONIC_STORE = "MNEMONIC_SECURE_STORE"

const HomeScreen = ({ navigation }) => {
    const [lines, setLines] = useState([])

    // Log sharing
    const onShareLogs = async () => {
        let nodeInfo = {}

        try {
            nodeInfo = await nodeInfo()
        } catch (e) {
            nodeInfo = { error: JSON.stringify(e) }
        }

        try {
            await FileLogger.sendLogFilesByEmail({
                subject: "Logs",
                body: `OS: ${Platform.OS} - ${Platform.Version}\n\n` + JSON.stringify(nodeInfo, null, 2)
            })
        } catch (e) {
            log.error(JSON.stringify(e))
        }
    }

    useEffect(() => {
        navigation.setOptions({
            headerRight: () => <Button onPress={onShareLogs} title="Share log" />
        })
    }, [navigation])

    // Handling log entries and SDK events
    const addLine = (title, text) => {
        setLines((lines) => [{ at: new Date().getTime(), title, text }, ...lines])
        log.debug(`${title}${text && text.length > 0 ? ": " + text : ""}`, true)
    }

    const logHandler = (logEntry) => {
        switch (logEntry.level) {
            case "ERROR":
                log.error(`[ERROR]: ${logEntry.line}`, true)
                break
            case "INFO":
                log.info(`[INFO]: ${logEntry.line}`, true)
                break
            case "TRACE":
                // Only log to file when in DEBUG mode
                log.debug(`[TRACE]: ${logEntry.line}`, DEBUG)
                break
            case "WARN":
                log.warn(`[WARN]: ${logEntry.line}`, true)
                break
            default:
                log.debug(`[${logEntry.level}]: ${logEntry.line}`, true)
                break
        }
    }

    const eventHandler = (breezEvent) => {
        addLine("event", JSON.stringify(breezEvent))
    }

    // Starting the SDK
    useEffect(() => {
        let logSubscription, eventSubscription

        const asyncFn = async () => {
            try {
                logSubscription = await setLogStream(logHandler)

                let mnemonic = await getSecureItem(MNEMONIC_STORE)

                if (mnemonic == null) {
                    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
                    setSecureItem(MNEMONIC_STORE, mnemonic)
                }

                let seed = await mnemonicToSeed(mnemonic)
                addLine("mnemonicToSeed", obfuscateString(JSON.stringify(seed)))

                const nodeConfig = {
                    type: NodeConfigVariant.GREENLIGHT,
                    config: {}
                }

                const config = await defaultConfig(EnvironmentType.PRODUCTION, BuildConfig.BREEZ_API_KEY, nodeConfig)
                addLine("defaultConfig", JSON.stringify(config))

                eventSubscription = await connect({ config, seed }, eventHandler)
                addLine("connect", null)

                const nodeState = await nodeInfo()
                addLine("nodeInfo", JSON.stringify(nodeState))

                const lsp = await lspInfo()
                addLine("lspInfo", JSON.stringify(lsp))

                const fiatCurrencies = await listFiatCurrencies()
                addLine("listFiatCurrencies", JSON.stringify(fiatCurrencies))

                const fiatRates = await fetchFiatRates()
                addLine("fetchFiatRates", JSON.stringify(fiatRates))

                const revSwapFees = await fetchReverseSwapFees({ sendAmountSat: null })
                addLine("revSwapFees", JSON.stringify(revSwapFees))

                const inProgressOnchainPaymentsRes = await inProgressOnchainPayments()
                addLine("inProgressOnchainPayments", JSON.stringify(inProgressOnchainPaymentsRes))

                const buyBitcoinResult = await buyBitcoin({
                    provider: BuyBitcoinProvider.MOONPAY,
                    openingFeeParams: null
                })
                addLine("buyBitcoin", JSON.stringify(buyBitcoinResult))

                const signMessageResult = await signMessage({ message: "Hello world" })
                addLine("signMessage: Hello World", JSON.stringify(signMessageResult))

                const verifyMessageResult = await checkMessage({
                    message: "Hello world",
                    pubkey: nodeState.id,
                    signature: signMessageResult.signature
                })
                addLine("verifyMessage:", JSON.stringify(verifyMessageResult))

                const openChannelFeeResult = await openChannelFee({
                    amountMsat: 100000000,
                    expiry: 3600
                })
                addLine("openChannelFee", JSON.stringify(openChannelFeeResult))

                const receivePaymentResult = await receivePayment({
                    amountMsat: 100000000,
                    description: "Hello world",
                    expiry: 3600,
                    cltv: 144,
                    useDescriptionHash: true,
                    openingFeeParams: openChannelFeeResult.usedFeeParams
                })
                addLine("receivePayment", JSON.stringify(receivePaymentResult))

                await backup()
                addLine("backupStatus", JSON.stringify(await backupStatus()))
            } catch (e) {
                addLine("error", e.toString())
                log.error(JSON.stringify(e))
            }
        }

        asyncFn()

        return () => {
            if (logSubscription) {
                logSubscription.remove()
            }
            if (eventSubscription) {
                eventSubscription.remove()
            }
        }
    }, [])

    return (
        <SafeAreaView>
            <ScrollView style={{ padding: 5 }}>
                {lines.map((line) => (
                    <DebugLine key={line.at} title={line.title} text={line.text} />
                ))}
            </ScrollView>
        </SafeAreaView>
    )
}

export default HomeScreen
