package sample.sdk.breez.page.home

import android.content.res.Resources
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import breez_sdk.BlockingBreezServices
import breez_sdk.Config
import breez_sdk.EventListener
import breez_sdk.FiatCurrency
import breez_sdk.InputType
import breez_sdk.LnInvoice
import breez_sdk.LnUrlCallbackStatus
import breez_sdk.LnUrlPayResult
import breez_sdk.LspInformation
import breez_sdk.NodeState
import breez_sdk.Payment
import breez_sdk.Rate
import breez_sdk.ReverseSwapInfo
import breez_sdk.ReverseSwapPairInfo
import breez_sdk.SwapInfo
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import sample.sdk.breez.BreezSdkWrapper
import sample.sdk.breez.R
import sample.sdk.breez.exceptions.SimpleError
import sample.sdk.breez.page.home.lnurlauth.LnUrlAuthState
import sample.sdk.breez.page.home.lnurlpay.LnUrlPayState
import sample.sdk.breez.page.home.lnurlwithdraw.LnUrlWithdrawState
import sample.sdk.breez.page.home.lspinfo.LspInfoState
import sample.sdk.breez.page.home.receive.ReceiveState
import sample.sdk.breez.page.home.receiveonchain.ReceiveOnChainState
import sample.sdk.breez.page.home.send.SendPaymentState
import sample.sdk.breez.page.home.sendonchain.SendOnChainState
import sample.sdk.breez.page.home.spontaneous.SpontaneousState
import javax.inject.Inject
import javax.inject.Named
import kotlin.math.max

private const val TAG = "HomeViewModel"

@HiltViewModel
class HomeViewModel @Inject constructor(
    private val breezSdkWrapper: BreezSdkWrapper,
    private val resources: Resources,
    private val config: Config,
    @Named("Seed") private val seed: List<UByte>,
    private val eventListener: EventListener,
    @Named("IO") private val io: CoroutineDispatcher,
) : ViewModel() {

    val homeState = MutableStateFlow<HomeState>(HomeState.Loading)
    val receiveState = MutableStateFlow<ReceiveState>(ReceiveState.Initial)
    val sendPaymentState = MutableStateFlow<SendPaymentState>(SendPaymentState.Initial)
    val spontaneousState = MutableStateFlow<SpontaneousState>(SpontaneousState.Initial)
    val receiveOnChainState = MutableStateFlow<ReceiveOnChainState>(ReceiveOnChainState.Initial)
    val sendOnChainState = MutableStateFlow<SendOnChainState>(SendOnChainState.Initial)
    val lspInfoState = MutableStateFlow<LspInfoState>(LspInfoState.Initial)
    val lnUrlPayState = MutableStateFlow<LnUrlPayState>(LnUrlPayState.Initial)
    val lnUrlWithdrawState = MutableStateFlow<LnUrlWithdrawState>(LnUrlWithdrawState.Initial)
    val lnUrlAuthState = MutableStateFlow<LnUrlAuthState>(LnUrlAuthState.Initial)

    private var breezSdk: BlockingBreezServices? = null

    init {
        viewModelScope.launch {
            breezSdk = connectBreezSdk()
            fetchSdkData()
        }
    }

    fun receiveLightningPayment(
        amount: Long,
        description: String,
    ) {
        viewModelScope.launch {
            receiveState.value = ReceiveState.Loading
            val invoice = receivePayment(amount, description)
            if (invoice == null) {
                receiveState.value = ReceiveState.Error(
                    SimpleError(resources.getString(R.string.error_receive_payment))
                )
            } else {
                val needOpenChannel = isChannelOpeningFeeNeeded(amount)
                val openChannelFee = if (needOpenChannel) {
                    calculateFeesForAmount(amount)
                } else null
                receiveState.value = ReceiveState.Success(invoice, openChannelFee)
            }
        }
    }

    fun sendLightningPayment(
        bolt11: String,
        amount: Long,
    ) {
        viewModelScope.launch {
            sendPaymentState.value = SendPaymentState.Loading
            val payment = sendPayment(bolt11, amount)
            if (payment == null) {
                sendPaymentState.value = SendPaymentState.Error(
                    SimpleError(resources.getString(R.string.error_send_payment))
                )
            } else {
                sendPaymentState.value = SendPaymentState.Success(payment)
            }
        }
    }

    fun sendSpontaneousPayment(
        bolt11: String,
        amount: Long,
    ) {
        viewModelScope.launch {
            spontaneousState.value = SpontaneousState.Loading
            val payment = sendSpontaneous(bolt11, amount)
            if (payment == null) {
                spontaneousState.value = SpontaneousState.Error(
                    SimpleError(resources.getString(R.string.error_send_spontaneous_payment))
                )
            } else {
                spontaneousState.value = SpontaneousState.Success(payment)
            }
        }
    }

    fun receiveOnChain() {
        viewModelScope.launch {
            receiveOnChainState.value = ReceiveOnChainState.Loading
            val swapInfo = receiveOnChainTransaction()
            if (swapInfo == null) {
                receiveOnChainState.value = ReceiveOnChainState.Error(
                    SimpleError(resources.getString(R.string.error_receive_on_chain))
                )
            } else {
                receiveOnChainState.value = ReceiveOnChainState.Success(swapInfo)
            }
        }
    }

    fun sendOnChain(
        amount: Long,
        address: String,
    ) {
        viewModelScope.launch {
            sendOnChainState.value = SendOnChainState.Loading
            val fees = fetchReverseSwapFees()
            if (fees == null) {
                sendOnChainState.value = SendOnChainState.Error(
                    SimpleError(resources.getString(R.string.error_send_on_chain_fee))
                )
            } else {
                val satPerVbyte = 1L;
                val swapInfo = sendOnChainTransaction(amount, address, fees.feesHash, satPerVbyte)
                if (swapInfo == null) {
                    sendOnChainState.value = SendOnChainState.Error(
                        SimpleError(resources.getString(R.string.error_send_on_chain))
                    )
                } else {
                    sendOnChainState.value = SendOnChainState.Success(swapInfo)
                }
            }
        }
    }

    fun lspInfo() {
        viewModelScope.launch {
            lspInfoState.value = LspInfoState.Loading
            val lspInfo = fetchLspInfo()
            if (lspInfo == null) {
                lspInfoState.value = LspInfoState.Error(
                    SimpleError(resources.getString(R.string.error_fetch_lsp_info))
                )
            } else {
                lspInfoState.value = LspInfoState.Success(lspInfo)
            }
        }
    }

    fun lnUrlPay(
        lnUrlPayUrl: String,
        amount: Long,
        comment: String,
    ) {
        viewModelScope.launch {
            lnUrlPayState.value = LnUrlPayState.Loading
            val lnUrlPay = payLnUrl(lnUrlPayUrl, amount, comment)
            if (lnUrlPay == null) {
                lnUrlPayState.value = LnUrlPayState.Error(
                    SimpleError(resources.getString(R.string.error_pay_ln_url))
                )
            } else {
                when (lnUrlPay) {
                    is LnUrlPayResult.EndpointSuccess -> {
                        lnUrlPayState.value = LnUrlPayState.Success(lnUrlPay.data)
                    }
                    is LnUrlPayResult.EndpointError -> {
                        lnUrlPayState.value = LnUrlPayState.Error(
                            SimpleError(resources.getString(R.string.error_pay_ln_url_end_point))
                        )
                    }
                }
            }
        }
    }

    fun lnUrlWithdraw(
        lnUrlWithdrawUrl: String,
        amount: Long,
        comment: String,
    ) {
        viewModelScope.launch {
            lnUrlWithdrawState.value = LnUrlWithdrawState.Loading
            val lnUrlWithdraw = withdrawLnUrl(lnUrlWithdrawUrl, amount, comment)
            if (lnUrlWithdraw == null) {
                lnUrlWithdrawState.value = LnUrlWithdrawState.Error(
                    SimpleError(resources.getString(R.string.error_pay_ln_withdraw))
                )
            } else {
                when (lnUrlWithdraw) {
                    is LnUrlCallbackStatus.Ok -> {
                        lnUrlWithdrawState.value = LnUrlWithdrawState.Success
                    }
                    is LnUrlCallbackStatus.ErrorStatus -> {
                        lnUrlWithdrawState.value = LnUrlWithdrawState.Error(
                            SimpleError(resources.getString(R.string.error_pay_ln_withdraw_status))
                        )
                    }
                }
            }
        }
    }

    fun lnUrlAuth(
        lnUrlAuthUrl: String,
    ) {
        viewModelScope.launch {
            lnUrlAuthState.value = LnUrlAuthState.Loading
            val lnUrlAuth = authLnUrl(lnUrlAuthUrl)
            if (lnUrlAuth == null) {
                lnUrlAuthState.value = LnUrlAuthState.Error(
                    SimpleError(resources.getString(R.string.error_pay_ln_auth))
                )
            } else {
                when (lnUrlAuth) {
                    is LnUrlCallbackStatus.Ok -> {
                        lnUrlAuthState.value = LnUrlAuthState.Success
                    }
                    is LnUrlCallbackStatus.ErrorStatus -> {
                        lnUrlAuthState.value = LnUrlAuthState.Error(
                            SimpleError(resources.getString(R.string.error_pay_ln_auth_status))
                        )
                    }
                }
            }
        }
    }

    private suspend fun connectBreezSdk(): BlockingBreezServices? = withContext(io) {
        Log.v(TAG, "connectBreezSdk")
        try {
            breezSdkWrapper.connect(config, seed, eventListener)
        } catch (e: Throwable) {
            homeState.value = HomeState.Error(e)
            null
        }.also {
            Log.v(TAG, "connectBreezSdk result: $it")
        }
    }

    private suspend fun fetchNodeInfo(): NodeState? = withContext(io) {
        Log.v(TAG, "fetchNodeInfo")
        try {
            breezSdk?.nodeInfo()
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "fetchNodeInfo result: $it")
        }
    }

    private suspend fun fetchSdkData() = withContext(io) {
        val nodeInfo = fetchNodeInfo()
        if (nodeInfo == null) {
            homeState.value = HomeState.Error(
                SimpleError(resources.getString(R.string.error_fetch_node_info))
            )
        } else {
            val inProgressSwapOuts = inProgressReverseSwaps() ?: emptyList()
            val fiatCurrencies = fetchFiatCurrencies() ?: emptyMap()
            homeState.value = HomeState.Success(
                lightningBalance = nodeInfo.channelsBalanceMsat.toLong(),
                onchainBalance = nodeInfo.onchainBalanceMsat.toLong(),
                inProgressSwapOuts = inProgressSwapOuts,
                fiatCurrencies = fiatCurrencies,
            )
        }
    }

    private suspend fun receivePayment(
        amount: Long,
        description: String,
    ): LnInvoice? = withContext(io) {
        Log.v(TAG, "receivePayment: amount: $amount, description: $description")
        try {
            breezSdk?.receivePayment(amount.toULong(), description)
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "receivePayment result: $it")
        }
    }

    private suspend fun sendPayment(
        bolt11: String,
        amount: Long,
    ): Payment? = withContext(io) {
        Log.v(TAG, "sendPayment: bolt11: $bolt11, amount: $amount")
        try {
            breezSdk?.sendPayment(bolt11, amount.toULong())
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "sendPayment result: $it")
        }
    }

    private suspend fun sendSpontaneous(
        nodeId: String,
        amount: Long,
    ): Payment? = withContext(io) {
        Log.v(TAG, "sendSpontaneous: nodeId: $nodeId, amount: $amount")
        try {
            breezSdk?.sendSpontaneousPayment(nodeId, amount.toULong())
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "sendSpontaneous result: $it")
        }
    }

    private suspend fun isChannelOpeningFeeNeeded(
        amount: Long,
    ): Boolean = withContext(io) {
        Log.v(TAG, "isChannelOpeningFeeNeeded: amount: $amount")
        try {
            breezSdk?.nodeInfo()?.inboundLiquidityMsats?.toLong()?.let { inboundLiquidityMsats ->
                inboundLiquidityMsats <= amount
            } ?: false
        } catch (e: Throwable) {
            Log.w(TAG, e)
            false
        }.also {
            Log.v(TAG, "isChannelOpeningFeeNeeded result: $it")
        }
    }

    private suspend fun calculateFeesForAmount(
        amount: Long,
    ): Long? = withContext(io) {
        Log.v(TAG, "calculateFeesForAmount: amount: $amount")
        try {
            val lspid = breezSdk?.lspId() ?: return@withContext null
            val lspInfo = breezSdk?.fetchLspInfo(lspid) ?: return@withContext null

            // We calculate the dynamic fees in millisatoshis rounded to satoshis.
            val channelDynamicFeeMsat = amount * lspInfo.channelFeePermyriad / 1000
            max(lspInfo.channelMinimumFeeMsat, channelDynamicFeeMsat)
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "calculateFeesForAmount result: $it")
        }
    }

    private suspend fun receiveOnChainTransaction(): SwapInfo? = withContext(io) {
        Log.v(TAG, "receiveOnChainTransaction")
        try {
            var swap = breezSdk?.inProgressSwap()
            if (swap == null) {
                swap = breezSdk?.receiveOnchain()
            }
            swap
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "receiveOnChainTransaction result: $it")
        }
    }

    private suspend fun fetchReverseSwapFees(): ReverseSwapPairInfo? = withContext(io) {
        Log.v(TAG, "fetchReverseSwapFees")
        try {
            breezSdk?.fetchReverseSwapFees()
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "fetchReverseSwapFees result: $it")
        }
    }

    private suspend fun sendOnChainTransaction(
        amount: Long,
        address: String,
        feeHash: String,
        satPerVbyte: Long,
    ): ReverseSwapInfo? = withContext(io) {
        Log.v(
            TAG, "sendOnChainTransaction: amount: $amount address: $address " +
                "feeHash: $feeHash satPerVbyte: $satPerVbyte"
        )
        try {
            breezSdk?.sendOnchain(amount.toULong(), address, feeHash, satPerVbyte.toULong())
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "sendOnChainTransaction result: $it")
        }
    }

    private suspend fun inProgressReverseSwaps(): List<ReverseSwapInfo>? = withContext(io) {
        Log.v(TAG, "inProgressReverseSwaps")
        try {
            breezSdk?.inProgressReverseSwaps()
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "inProgressReverseSwaps result: $it")
        }
    }

    private suspend fun fetchLspInfo(): LspInformation? = withContext(io) {
        Log.v(TAG, "fetchLspInfo")
        try {
            val lspId = breezSdk?.lspId() ?: return@withContext null
            breezSdk?.connectLsp(lspId)
            breezSdk?.fetchLspInfo(lspId)
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "fetchLspInfo result: $it")
        }
    }

    private suspend fun payLnUrl(
        lnUrlPayUrl: String,
        amount: Long,
        comment: String,
    ): LnUrlPayResult? = withContext(io) {
        Log.v(TAG, "payLnUrl: lnUrlPayUrl: $lnUrlPayUrl amount: $amount comment: $comment")
        try {
            when (val inputType = breezSdkWrapper.parseInput(lnUrlPayUrl)) {
                is InputType.LnUrlPay -> breezSdk?.payLnurl(
                    inputType.data,
                    amount.toULong(),
                    comment,
                )
                else -> null
            }
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "payLnUrl result: $it")
        }
    }

    private suspend fun withdrawLnUrl(
        lnUrlWithdrawUrl: String,
        amount: Long,
        comment: String,
    ): LnUrlCallbackStatus? = withContext(io) {
        Log.v(
            TAG, "withdrawLnUrl: lnUrlWithdrawUrl: $lnUrlWithdrawUrl amount: $amount " +
                "comment: $comment"
        )
        try {
            when (val inputType = breezSdkWrapper.parseInput(lnUrlWithdrawUrl)) {
                is InputType.LnUrlWithdraw -> breezSdk?.withdrawLnurl(
                    inputType.data,
                    amount.toULong(),
                    comment,
                )
                else -> null
            }
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "withdrawLnUrl result: $it")
        }
    }

    private suspend fun authLnUrl(
        lnUrlAuthUrl: String,
    ): LnUrlCallbackStatus? = withContext(io) {
        Log.v(TAG, "authLnUrl: lnUrlAuthUrl: $lnUrlAuthUrl")
        try {
            when (val inputType = breezSdkWrapper.parseInput(lnUrlAuthUrl)) {
                is InputType.LnUrlAuth -> breezSdk?.lnurlAuth(inputType.data)
                else -> null
            }
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "authLnUrl result: $it")
        }
    }

    private suspend fun fetchFiatCurrencies(): Map<FiatCurrency, Rate>? = withContext(io) {
        Log.v(TAG, "fetchFiatCurrencies")
        try {
            val fiatCurrencies = breezSdk?.listFiatCurrencies() ?: emptyList()
            Log.v(TAG, "fetchFiatCurrencies: fiatCurrencies: $fiatCurrencies")
            val fiatRates = breezSdk?.fetchFiatRates() ?: emptyList()
            Log.v(TAG, "fetchFiatCurrencies: fiatRates: $fiatRates")

            val ratesMap = mutableMapOf<String, Rate>()
            for (rate in fiatRates) {
                ratesMap[rate.coin.lowercase()] = rate
            }

            val sorted = fiatCurrencies.sortedBy { it.info.name }
            val result = LinkedHashMap<FiatCurrency, Rate>()
            for (currency in sorted) {
                val rate = ratesMap[currency.id.lowercase()]
                if (rate != null) {
                    result[currency] = rate
                }
            }

            result
        } catch (e: Throwable) {
            Log.w(TAG, e)
            null
        }.also {
            Log.v(TAG, "fetchFiatCurrencies result: $it")
        }
    }

}
