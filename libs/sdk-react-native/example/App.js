/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow strict-local
 */

import React, { useState } from "react"
import { SafeAreaView, ScrollView, StatusBar, Text, TouchableOpacity, View } from "react-native"
import {
    addEventListener,
    addLogListener,
    BuyBitcoinProvider,
    defaultConfig,
    EnvironmentType,
    fetchFiatRates,
    fetchReverseSwapFees,
    initServices,
    inProgressReverseSwaps,
    listFiatCurrencies,
    mnemonicToSeed,
    Network,
    nodeInfo,
    recoverNode,
    registerNode,
    buyBitcoin,
    start,
    backup,
    backupStatus
} from "@breeztech/react-native-breez-sdk"
import BuildConfig from "react-native-build-config"
import { generateMnemonic } from "@dreson4/react-native-quick-bip39"
import { obfuscateString } from "./utils/security"
import { getSecureItem, setSecureItem } from "./utils/storage"

const MNEMONIC_STORE = "MNEMONIC_SECURE_STORE"
const GREENLIGHT_DEVICE_KEY_STORE = "GREENLIGHT_DEVICE_KEY_SECURE_STORE"
const GREENLIGHT_DEVICE_CERT_STORE = "GREENLIGHT_DEVICE_CERT_SECURE_STORE"

const DebugLine = ({ title, text }) => {
    return (
        <TouchableOpacity style={{ flex: 1 }}>
            <View style={{ margin: 5 }}>
                <Text style={{ fontWeight: "bold" }}>{title}</Text>
                {text && text.length > 0 ? <Text>{text}</Text> : <></>}
            </View>
        </TouchableOpacity>
    )
}

const App = () => {
    const [lines, setLines] = useState([])

    const addLine = (title, text) => {
        setLines((lines) => [{ at: new Date().getTime(), title, text }, ...lines])
    }

    const logHandler = (l) => {
      if (l.level != "TRACE") {
        console.log(`[${l.level}]: ${l.line}`)
      }
    }

    const eventHandler = (type, data) => {
        addLine("event", `${type}${data ? " : " + JSON.stringify(data) : ""}`)
    }

    React.useEffect(() => {
        const asyncFn = async () => {
            await addLogListener(logHandler)
            addEventListener(eventHandler)

            let seed = null
            let mnemonic = await getSecureItem(MNEMONIC_STORE)

            if (mnemonic == null) {
                mnemonic = generateMnemonic(256)
                setSecureItem(MNEMONIC_STORE, mnemonic)

                seed = await mnemonicToSeed(mnemonic)
                addLine("mnemonicToSeed", obfuscateString(JSON.stringify(seed)))

                const greenlightCredentials = await registerNode(Network.BITCOIN, seed)

                addLine("registerNode", null)
                setSecureItem(GREENLIGHT_DEVICE_KEY_STORE, greenlightCredentials.deviceKey)
                setSecureItem(GREENLIGHT_DEVICE_CERT_STORE, greenlightCredentials.deviceCert)
            } else {
                seed = await mnemonicToSeed(mnemonic)
                addLine("mnemonicToSeed", obfuscateString(JSON.stringify(seed)))
            }

            let deviceKey = await getSecureItem(GREENLIGHT_DEVICE_KEY_STORE)
            let deviceCert = await getSecureItem(GREENLIGHT_DEVICE_CERT_STORE)

            if (deviceKey == null) {
                const greenlightCredentials = await recoverNode(Network.BITCOIN, seed)

                addLine("recoverNode", null)                
                setSecureItem(GREENLIGHT_DEVICE_KEY_STORE, greenlightCredentials.deviceKey)
                setSecureItem(GREENLIGHT_DEVICE_CERT_STORE, greenlightCredentials.deviceCert)
                deviceKey = greenlightCredentials.deviceKey
                deviceCert = greenlightCredentials.deviceCert
            }

            if (deviceKey && deviceCert) {
                addLine("greenlight deviceKey", obfuscateString(JSON.stringify(deviceKey)))

                const config = await defaultConfig(EnvironmentType.PRODUCTION)
                addLine("defaultConfig", JSON.stringify(config))
                config.apiKey = BuildConfig.BREEZ_API_KEY

                await initServices(config, deviceKey, deviceCert, seed)
                addLine("initServices", null)

                await start()
                addLine("start", null)

                const nodeState = await nodeInfo()
                addLine("nodeInfo", JSON.stringify(nodeState))

                const fiatCurrencies = await listFiatCurrencies()
                addLine("listFiatCurrencies", JSON.stringify(fiatCurrencies))

                const fiatRates = await fetchFiatRates()
                addLine("fetchFiatRates", JSON.stringify(fiatRates))

                const revSwapFees = await fetchReverseSwapFees()
                addLine("revSwapFees", JSON.stringify(revSwapFees))

                const inProgressRevSwaps = await inProgressReverseSwaps()
                addLine("inProgressRevSwaps", JSON.stringify(inProgressRevSwaps))

                const buyBitcoinResult = await buyBitcoin(BuyBitcoinProvider.MOONPAY)
                addLine("buyBitcoin", JSON.stringify(buyBitcoinResult))
                                                
                await backup();
                addLine("backupStatus", JSON.stringify( await backupStatus()));                
            }
        }
        asyncFn()
    }, [])

    return (
        <SafeAreaView>
            <StatusBar />
            <ScrollView style={{ padding: 5 }}>
                {lines.map((line) => (
                    <DebugLine key={line.at} title={line.title} text={line.text} />
                ))}
            </ScrollView>
        </SafeAreaView>
    )
}

export default App
