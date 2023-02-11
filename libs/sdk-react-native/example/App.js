/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow strict-local
 */

import React from "react"
import { SafeAreaView, ScrollView, StatusBar, Text, View } from "react-native"
import { mnemonicToSeed } from "react-native-breez-sdk"

const TEXT_MNEMONIC =
    "reveal man culture nominee tag abuse keen behave refuse warfare crisp thunder " +
    "valve knock unique try fold energy torch news thought access hawk table"

const App = () => {
    const [result, setResult] = React.useState()

    React.useEffect(() => {
        const asyncFn = async () => {
            console.log(`Mnemonic: ${TEXT_MNEMONIC}`)
            const response = await mnemonicToSeed(TEXT_MNEMONIC)
            console.log(`Response: ${response}`)
            setResult(JSON.stringify(response))
        }
        asyncFn()
    }, [])

    return (
        <SafeAreaView>
            <StatusBar />
            <ScrollView contentInsetAdjustmentBehavior="automatic">
                <View style={{ backgroundColor: "white" }}>
                    <Text>Seed: {`${result}`}</Text>
                </View>
            </ScrollView>
        </SafeAreaView>
    )
}

export default App
