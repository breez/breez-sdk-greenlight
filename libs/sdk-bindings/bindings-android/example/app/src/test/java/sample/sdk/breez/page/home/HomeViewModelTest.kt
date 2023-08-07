package sample.sdk.breez.page.home

import android.content.res.Resources
import breez_sdk.BlockingBreezServices
import breez_sdk.Config
import breez_sdk.CurrencyInfo
import breez_sdk.EventListener
import breez_sdk.FiatCurrency
import breez_sdk.InputType
import breez_sdk.LnInvoice
import breez_sdk.LnUrlAuthRequestData
import breez_sdk.LnUrlCallbackStatus
import breez_sdk.LnUrlPayRequestData
import breez_sdk.LnUrlPayResult
import breez_sdk.LnUrlWithdrawRequestData
import breez_sdk.LspInformation
import breez_sdk.NodeState
import breez_sdk.Payment
import breez_sdk.Rate
import breez_sdk.ReverseSwapInfo
import breez_sdk.ReverseSwapPairInfo
import breez_sdk.SuccessActionProcessed
import breez_sdk.SwapInfo
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestCoroutineScheduler
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.After
import org.junit.Before
import org.junit.Test
import org.mockito.Mock
import org.mockito.MockitoAnnotations
import org.mockito.kotlin.any
import org.mockito.kotlin.mock
import org.mockito.kotlin.verify
import org.mockito.kotlin.whenever
import sample.sdk.breez.BreezSdkWrapper
import sample.sdk.breez.advance
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
import sample.sdk.breez.test

@OptIn(ExperimentalCoroutinesApi::class)
class HomeViewModelTest {

    @Mock lateinit var breezSdkWrapper: BreezSdkWrapper
    @Mock lateinit var resources: Resources
    @Mock lateinit var config: Config
    @Mock lateinit var seed: List<UByte>
    @Mock lateinit var eventListener: EventListener
    @Mock lateinit var nodeState: NodeState
    @Mock lateinit var breezSdk: BlockingBreezServices
    @Mock lateinit var lnInvoice: LnInvoice
    @Mock lateinit var inProgressSwapOut: ReverseSwapInfo
    @Mock lateinit var fiatCurrency: FiatCurrency
    @Mock lateinit var currencyInfo: CurrencyInfo
    @Mock lateinit var rate: Rate
    @Mock lateinit var lspInfo: LspInformation
    @Mock lateinit var payment: Payment
    @Mock lateinit var swapInfo: SwapInfo
    @Mock lateinit var reverseSwapInfo: ReverseSwapInfo
    @Mock lateinit var reverseSwapPairInfo: ReverseSwapPairInfo
    @Mock lateinit var endPointSuccess: LnUrlPayResult.EndpointSuccess
    @Mock lateinit var endPointError: LnUrlPayResult.EndpointError
    @Mock lateinit var successActionProcessed: SuccessActionProcessed
    @Mock lateinit var lnUrlPayRequestData: LnUrlPayRequestData
    @Mock lateinit var lnUrlWithdrawRequestData: LnUrlWithdrawRequestData
    @Mock lateinit var lnUrlAuthRequestData: LnUrlAuthRequestData

    private lateinit var inProgressSwapOuts: List<ReverseSwapInfo>
    private lateinit var fiatCurrencies: Map<FiatCurrency, Rate>

    private val testCoroutineDispatcher = StandardTestDispatcher(TestCoroutineScheduler())

    private val errorText = "An error occurred"
    private val error = SimpleError(errorText)
    private val amount = 1L
    private val amountOpenChannel = 6L
    private val description = "A description"
    private val lightningBalance = 2L
    private val onchainBalance = 3L
    private val currencyCode = "A currency code"
    private val currencyName = "A currency name"
    private val rateValue = 4.0
    private val inboundLiquidityMsats = 5L
    private val lspId = "A lsp id"
    private val channelFeePermyriad = 7L
    private val channelMinimumFeeMsat = 8L
    private val bolt11 = "A bolt 11"
    private val nodeId = "A node id"
    private val address = "An address"
    private val satPerVbyte = 1L
    private val feeHash = "A fee hash"
    private val lnUrlPayUrl = "A ln url pay url"
    private val lnUrlWithdrawUrl = "A ln url withdraw url"
    private val lnUrlAuthUrl = "A ln url auth url"
    private val comment = "A comment"

    @Before
    fun setUp() {
        Dispatchers.setMain(testCoroutineDispatcher)
        MockitoAnnotations.openMocks(this)

        inProgressSwapOuts = listOf(inProgressSwapOut)
        fiatCurrencies = mapOf(fiatCurrency to rate)

        whenever(breezSdkWrapper.connect(config, seed, eventListener)).thenReturn(breezSdk)
        whenever(
            breezSdkWrapper.parseInput(lnUrlPayUrl)
        ).thenReturn(InputType.LnUrlPay(lnUrlPayRequestData))
        whenever(
            breezSdkWrapper.parseInput(lnUrlWithdrawUrl)
        ).thenReturn(InputType.LnUrlWithdraw(lnUrlWithdrawRequestData))
        whenever(
            breezSdkWrapper.parseInput(lnUrlAuthUrl)
        ).thenReturn(InputType.LnUrlAuth(lnUrlAuthRequestData))

        whenever(resources.getString(any())).thenReturn(errorText)

        whenever(nodeState.channelsBalanceMsat).thenReturn(lightningBalance.toULong())
        whenever(nodeState.onchainBalanceMsat).thenReturn(onchainBalance.toULong())
        whenever(nodeState.inboundLiquidityMsats).thenReturn(inboundLiquidityMsats.toULong())

        whenever(fiatCurrency.info).thenReturn(currencyInfo)
        whenever(fiatCurrency.id).thenReturn(currencyCode)
        whenever(currencyInfo.name).thenReturn(currencyName)
        whenever(rate.coin).thenReturn(currencyCode)
        whenever(rate.value).thenReturn(rateValue)

        whenever(lspInfo.channelFeePermyriad).thenReturn(channelFeePermyriad)
        whenever(lspInfo.channelMinimumFeeMsat).thenReturn(channelMinimumFeeMsat)

        whenever(reverseSwapPairInfo.feesHash).thenReturn(feeHash)

        whenever(endPointSuccess.data).thenReturn(successActionProcessed)

        whenever(breezSdk.nodeInfo()).thenReturn(nodeState)
        whenever(breezSdk.receivePayment(amount.toULong(), description)).thenReturn(lnInvoice)
        whenever(
            breezSdk.receivePayment(amountOpenChannel.toULong(), description)
        ).thenReturn(lnInvoice)
        whenever(breezSdk.inProgressReverseSwaps()).thenReturn(inProgressSwapOuts)
        whenever(breezSdk.listFiatCurrencies()).thenReturn(listOf(fiatCurrency))
        whenever(breezSdk.fetchFiatRates()).thenReturn(listOf(rate))
        whenever(breezSdk.lspId()).thenReturn(lspId)
        whenever(breezSdk.fetchLspInfo(lspId)).thenReturn(lspInfo)
        whenever(breezSdk.sendPayment(bolt11, amount.toULong())).thenReturn(payment)
        whenever(breezSdk.sendSpontaneousPayment(nodeId, amount.toULong())).thenReturn(payment)
        whenever(breezSdk.inProgressSwap()).thenReturn(swapInfo)
        whenever(breezSdk.receiveOnchain()).thenReturn(swapInfo)
        whenever(breezSdk.fetchReverseSwapFees()).thenReturn(reverseSwapPairInfo)
        whenever(
            breezSdk.sendOnchain(amount.toULong(), address, feeHash, satPerVbyte.toULong())
        ).thenReturn(reverseSwapInfo)
        whenever(
            breezSdk.payLnurl(lnUrlPayRequestData, amount.toULong(), comment)
        ).thenReturn(endPointSuccess)
        whenever(
            breezSdk.withdrawLnurl(lnUrlWithdrawRequestData, amount.toULong(), comment)
        ).thenReturn(LnUrlCallbackStatus.Ok)
        whenever(breezSdk.lnurlAuth(lnUrlAuthRequestData)).thenReturn(LnUrlCallbackStatus.Ok)
    }

    @After
    fun tearDown() {
        Dispatchers.resetMain()
    }

    @Test
    fun `init should emit home state`() = runTest {
        val viewModel = make()

        val tester = viewModel.homeState.test(this)
        advance()

        tester.assertValuesAndFinish(
            HomeState.Loading,
            HomeState.Success(lightningBalance, onchainBalance, inProgressSwapOuts, fiatCurrencies),
        )
    }

    @Test
    fun `receiveLightningPayment when receivePayment returns null should emit error`() = runTest {
        whenever(breezSdk.receivePayment(amount.toULong(), description)).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.receiveState.test(this)
        advance()
        viewModel.receiveLightningPayment(amount, description)
        advance()

        tester.assertValuesAndFinish(
            ReceiveState.Initial,
            ReceiveState.Loading,
            ReceiveState.Error(error),
        )
    }

    @Test
    fun `receiveLightningPayment when amount is in capacity should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.receiveState.test(this)
        advance()
        viewModel.receiveLightningPayment(amount, description)
        advance()

        tester.assertValuesAndFinish(
            ReceiveState.Initial,
            ReceiveState.Loading,
            ReceiveState.Success(lnInvoice, null),
        )
    }

    @Test
    fun `receiveLightningPayment when amount is over capacity should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.receiveState.test(this)
        advance()
        viewModel.receiveLightningPayment(amountOpenChannel, description)
        advance()

        tester.assertValuesAndFinish(
            ReceiveState.Initial,
            ReceiveState.Loading,
            ReceiveState.Success(lnInvoice, channelMinimumFeeMsat),
        )
    }

    @Test
    fun `sendLightningPayment when it fails should emit error`() = runTest {
        whenever(breezSdk.sendPayment(bolt11, amount.toULong())).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.sendPaymentState.test(this)
        advance()
        viewModel.sendLightningPayment(bolt11, amount)
        advance()

        tester.assertValuesAndFinish(
            SendPaymentState.Initial,
            SendPaymentState.Loading,
            SendPaymentState.Error(error),
        )
    }

    @Test
    fun `sendLightningPayment should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.sendPaymentState.test(this)
        advance()
        viewModel.sendLightningPayment(bolt11, amount)
        advance()

        tester.assertValuesAndFinish(
            SendPaymentState.Initial,
            SendPaymentState.Loading,
            SendPaymentState.Success(payment),
        )
    }

    @Test
    fun `sendSpontaneousPayment when it fails should emit error`() = runTest {
        whenever(breezSdk.sendSpontaneousPayment(nodeId, amount.toULong())).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.spontaneousState.test(this)
        advance()
        viewModel.sendSpontaneousPayment(nodeId, amount)
        advance()

        tester.assertValuesAndFinish(
            SpontaneousState.Initial,
            SpontaneousState.Loading,
            SpontaneousState.Error(error),
        )
    }

    @Test
    fun `sendSpontaneousPayment should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.spontaneousState.test(this)
        advance()
        viewModel.sendSpontaneousPayment(nodeId, amount)
        advance()

        tester.assertValuesAndFinish(
            SpontaneousState.Initial,
            SpontaneousState.Loading,
            SpontaneousState.Success(payment),
        )
    }

    @Test
    fun `receiveOnChain through inProgressSwap should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.receiveOnChainState.test(this)
        advance()
        viewModel.receiveOnChain()
        advance()

        tester.assertValuesAndFinish(
            ReceiveOnChainState.Initial,
            ReceiveOnChainState.Loading,
            ReceiveOnChainState.Success(swapInfo),
        )
    }

    @Test
    fun `receiveOnChain through receiveOnchain should emit success`() = runTest {
        whenever(breezSdk.inProgressSwap()).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.receiveOnChainState.test(this)
        advance()
        viewModel.receiveOnChain()
        advance()

        tester.assertValuesAndFinish(
            ReceiveOnChainState.Initial,
            ReceiveOnChainState.Loading,
            ReceiveOnChainState.Success(swapInfo),
        )
    }

    @Test
    fun `receiveOnChain when it fails should emit error`() = runTest {
        whenever(breezSdk.inProgressSwap()).thenReturn(null)
        whenever(breezSdk.receiveOnchain()).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.receiveOnChainState.test(this)
        advance()
        viewModel.receiveOnChain()
        advance()

        tester.assertValuesAndFinish(
            ReceiveOnChainState.Initial,
            ReceiveOnChainState.Loading,
            ReceiveOnChainState.Error(error),
        )
    }

    @Test
    fun `sendOnChain should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.sendOnChainState.test(this)
        advance()
        viewModel.sendOnChain(amount, address)
        advance()

        tester.assertValuesAndFinish(
            SendOnChainState.Initial,
            SendOnChainState.Loading,
            SendOnChainState.Success(reverseSwapInfo),
        )
    }

    @Test
    fun `sendOnChain when sendOnchain fails should emit error`() = runTest {
        whenever(
            breezSdk.sendOnchain(amount.toULong(), address, feeHash, satPerVbyte.toULong())
        ).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.sendOnChainState.test(this)
        advance()
        viewModel.sendOnChain(amount, address)
        advance()

        tester.assertValuesAndFinish(
            SendOnChainState.Initial,
            SendOnChainState.Loading,
            SendOnChainState.Error(error),
        )
    }

    @Test
    fun `sendOnChain when fetchReverseSwapFees fails should emit error`() = runTest {
        whenever(breezSdk.fetchReverseSwapFees()).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.sendOnChainState.test(this)
        advance()
        viewModel.sendOnChain(amount, address)
        advance()

        tester.assertValuesAndFinish(
            SendOnChainState.Initial,
            SendOnChainState.Loading,
            SendOnChainState.Error(error),
        )
    }

    @Test
    fun `lspInfo should connect lsp`() = runTest {
        val viewModel = make()
        advance()

        viewModel.lspInfo()
        advance()

        verify(breezSdk).connectLsp(lspId)
    }

    @Test
    fun `lspInfo should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.lspInfoState.test(this)
        advance()
        viewModel.lspInfo()
        advance()

        tester.assertValuesAndFinish(
            LspInfoState.Initial,
            LspInfoState.Loading,
            LspInfoState.Success(lspInfo),
        )
    }

    @Test
    fun `lspInfo when fetchLspInfo fails should emit error`() = runTest {
        whenever(breezSdk.fetchLspInfo(lspId)).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.lspInfoState.test(this)
        advance()
        viewModel.lspInfo()
        advance()

        tester.assertValuesAndFinish(
            LspInfoState.Initial,
            LspInfoState.Loading,
            LspInfoState.Error(error),
        )
    }

    @Test
    fun `lspInfo when lspId fails should emit error`() = runTest {
        whenever(breezSdk.lspId()).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.lspInfoState.test(this)
        advance()
        viewModel.lspInfo()
        advance()

        tester.assertValuesAndFinish(
            LspInfoState.Initial,
            LspInfoState.Loading,
            LspInfoState.Error(error),
        )
    }

    @Test
    fun `lnUrlPay should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.lnUrlPayState.test(this)
        advance()
        viewModel.lnUrlPay(lnUrlPayUrl, amount, comment)
        advance()

        tester.assertValuesAndFinish(
            LnUrlPayState.Initial,
            LnUrlPayState.Loading,
            LnUrlPayState.Success(successActionProcessed),
        )
    }

    @Test
    fun `lnUrlPay when endpoint is error should emit error`() = runTest {
        whenever(
            breezSdk.payLnurl(lnUrlPayRequestData, amount.toULong(), comment)
        ).thenReturn(endPointError)
        val viewModel = make()

        val tester = viewModel.lnUrlPayState.test(this)
        advance()
        viewModel.lnUrlPay(lnUrlPayUrl, amount, comment)
        advance()

        tester.assertValuesAndFinish(
            LnUrlPayState.Initial,
            LnUrlPayState.Loading,
            LnUrlPayState.Error(error),
        )
    }

    @Test
    fun `lnUrlPay when payLnurl fails should emit error`() = runTest {
        whenever(
            breezSdk.payLnurl(lnUrlPayRequestData, amount.toULong(), comment)
        ).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.lnUrlPayState.test(this)
        advance()
        viewModel.lnUrlPay(lnUrlPayUrl, amount, comment)
        advance()

        tester.assertValuesAndFinish(
            LnUrlPayState.Initial,
            LnUrlPayState.Loading,
            LnUrlPayState.Error(error),
        )
    }

    @Test
    fun `lnUrlWithdraw should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.lnUrlWithdrawState.test(this)
        advance()
        viewModel.lnUrlWithdraw(lnUrlWithdrawUrl, amount, comment)
        advance()

        tester.assertValuesAndFinish(
            LnUrlWithdrawState.Initial,
            LnUrlWithdrawState.Loading,
            LnUrlWithdrawState.Success,
        )
    }

    @Test
    fun `lnUrlWithdraw when endpoint is error should emit error`() = runTest {
        whenever(
            breezSdk.withdrawLnurl(lnUrlWithdrawRequestData, amount.toULong(), comment)
        ).thenReturn(LnUrlCallbackStatus.ErrorStatus(mock()))
        val viewModel = make()

        val tester = viewModel.lnUrlWithdrawState.test(this)
        advance()
        viewModel.lnUrlWithdraw(lnUrlWithdrawUrl, amount, comment)
        advance()

        tester.assertValuesAndFinish(
            LnUrlWithdrawState.Initial,
            LnUrlWithdrawState.Loading,
            LnUrlWithdrawState.Error(error),
        )
    }

    @Test
    fun `lnUrlWithdraw when withdrawLnurl fails should emit error`() = runTest {
        whenever(
            breezSdk.withdrawLnurl(lnUrlWithdrawRequestData, amount.toULong(), comment)
        ).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.lnUrlWithdrawState.test(this)
        advance()
        viewModel.lnUrlWithdraw(lnUrlWithdrawUrl, amount, comment)
        advance()

        tester.assertValuesAndFinish(
            LnUrlWithdrawState.Initial,
            LnUrlWithdrawState.Loading,
            LnUrlWithdrawState.Error(error),
        )
    }

    @Test
    fun `lnUrlAuth should emit success`() = runTest {
        val viewModel = make()

        val tester = viewModel.lnUrlAuthState.test(this)
        advance()
        viewModel.lnUrlAuth(lnUrlAuthUrl)
        advance()

        tester.assertValuesAndFinish(
            LnUrlAuthState.Initial,
            LnUrlAuthState.Loading,
            LnUrlAuthState.Success,
        )
    }

    @Test
    fun `lnUrlAuth when endpoint is error should emit error`() = runTest {
        whenever(
            breezSdk.lnurlAuth(lnUrlAuthRequestData)
        ).thenReturn(LnUrlCallbackStatus.ErrorStatus(mock()))
        val viewModel = make()

        val tester = viewModel.lnUrlAuthState.test(this)
        advance()
        viewModel.lnUrlAuth(lnUrlAuthUrl)
        advance()

        tester.assertValuesAndFinish(
            LnUrlAuthState.Initial,
            LnUrlAuthState.Loading,
            LnUrlAuthState.Error(error),
        )
    }

    @Test
    fun `lnUrlAuth when withdrawLnurl fails should emit error`() = runTest {
        whenever(
            breezSdk.lnurlAuth(lnUrlAuthRequestData)
        ).thenReturn(null)
        val viewModel = make()

        val tester = viewModel.lnUrlAuthState.test(this)
        advance()
        viewModel.lnUrlAuth(lnUrlAuthUrl)
        advance()

        tester.assertValuesAndFinish(
            LnUrlAuthState.Initial,
            LnUrlAuthState.Loading,
            LnUrlAuthState.Error(error),
        )
    }

    private fun make() = HomeViewModel(
        breezSdkWrapper,
        resources,
        config,
        seed,
        eventListener,
        testCoroutineDispatcher,
    )

}
