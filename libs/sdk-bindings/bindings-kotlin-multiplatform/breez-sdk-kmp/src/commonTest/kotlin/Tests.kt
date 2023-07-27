package com.blockstream.gdk


import breez_sdk.EnvironmentType
import breez_sdk.GreenlightNodeConfig
import breez_sdk.Network
import breez_sdk.NodeConfig
import breez_sdk.defaultConfig
import breez_sdk.mnemonicToSeed
import kotlin.test.Test
import kotlin.test.assertEquals

class Tests {

    @Test
    fun testDefaultConfig() {
        val config = defaultConfig(
            envType = EnvironmentType.PRODUCTION,
            apiKey = "",
            nodeConfig = NodeConfig.Greenlight(GreenlightNodeConfig(null, ""))
        )

        assertEquals(config.network, Network.BITCOIN)

    }

    @Test
    fun testMnemonicToSeed() {
        val seed = mnemonicToSeed("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
        assertEquals(64, seed.size)
    }
}