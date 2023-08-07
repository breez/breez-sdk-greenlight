package sample.sdk.breez

import breez_sdk.Config
import breez_sdk.EventListener
import javax.inject.Inject

/**
 * Breez sdk provides static methods such as connect; You can use these methods directly if you want
 * without problems; If you want to write unit tests easier, it is recommended to create a wrapper
 * class like this one; For more details, see [sample.sdk.breez.page.home.HomeViewModelTest] and
 * [sample.sdk.breez.page.home.HomeViewModel]
 */
class BreezSdkWrapper @Inject constructor() {
    fun connect(
        config: Config,
        seed: List<UByte>,
        eventListener: EventListener,
    ) = breez_sdk.connect(config, seed, eventListener)

    fun parseInput(
        input: String,
    ) = breez_sdk.parseInput(input)
}
