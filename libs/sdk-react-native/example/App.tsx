/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow strict-local
 */

import React from "react"
import { NavigationContainer } from "@react-navigation/native"
import { createNativeStackNavigator } from "@react-navigation/native-stack"
import { FileLogger } from "react-native-file-logger"
import HomeScreen from "./src/screens/HomeScreen"

FileLogger.configure({
    captureConsole: false,
    maximumNumberOfFiles: 5
})

const Stack = createNativeStackNavigator()

const App = () => {
    return (
        <NavigationContainer>
            <Stack.Navigator>
                <Stack.Screen name="Example App" component={HomeScreen} />
            </Stack.Navigator>
        </NavigationContainer>
    )
}

export default App
