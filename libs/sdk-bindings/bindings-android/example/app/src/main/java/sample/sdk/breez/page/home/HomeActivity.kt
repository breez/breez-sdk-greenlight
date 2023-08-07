package sample.sdk.breez.page.home

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import dagger.hilt.android.AndroidEntryPoint
import sample.sdk.breez.page.BreezSdkSampleTheme
import sample.sdk.breez.page.home.lnurlauth.LnUrlAuth
import sample.sdk.breez.page.home.lnurlauth.LnUrlAuthAction
import sample.sdk.breez.page.home.lnurlauth.LnUrlAuthState
import sample.sdk.breez.page.home.lnurlpay.LnUrlPay
import sample.sdk.breez.page.home.lnurlpay.LnUrlPayAction
import sample.sdk.breez.page.home.lnurlpay.LnUrlPayState
import sample.sdk.breez.page.home.lnurlwithdraw.LnUrlWithdraw
import sample.sdk.breez.page.home.lnurlwithdraw.LnUrlWithdrawAction
import sample.sdk.breez.page.home.lnurlwithdraw.LnUrlWithdrawState
import sample.sdk.breez.page.home.lspinfo.LspInfo
import sample.sdk.breez.page.home.lspinfo.LspInfoAction
import sample.sdk.breez.page.home.lspinfo.LspInfoState
import sample.sdk.breez.page.home.receive.ReceivePayment
import sample.sdk.breez.page.home.receive.ReceivePaymentAction
import sample.sdk.breez.page.home.receive.ReceiveState
import sample.sdk.breez.page.home.receiveonchain.ReceiveOnChain
import sample.sdk.breez.page.home.receiveonchain.ReceiveOnChainAction
import sample.sdk.breez.page.home.receiveonchain.ReceiveOnChainState
import sample.sdk.breez.page.home.section.SectionError
import sample.sdk.breez.page.home.section.SectionLoading
import sample.sdk.breez.page.home.send.SendPayment
import sample.sdk.breez.page.home.send.SendPaymentAction
import sample.sdk.breez.page.home.send.SendPaymentState
import sample.sdk.breez.page.home.sendonchain.SendOnChain
import sample.sdk.breez.page.home.sendonchain.SendOnChainAction
import sample.sdk.breez.page.home.sendonchain.SendOnChainState
import sample.sdk.breez.page.home.spontaneous.SpontaneousPayment
import sample.sdk.breez.page.home.spontaneous.SpontaneousPaymentAction
import sample.sdk.breez.page.home.spontaneous.SpontaneousState

@AndroidEntryPoint
class MainActivity : ComponentActivity() {

    private val viewModel by viewModels<HomeViewModel>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            val homeState = viewModel.homeState.collectAsState()
            val invoiceState = viewModel.receiveState.collectAsState()
            val sendPaymentState = viewModel.sendPaymentState.collectAsState()
            val spontaneousState = viewModel.spontaneousState.collectAsState()
            val receiveOnChainState = viewModel.receiveOnChainState.collectAsState()
            val sendOnChainState = viewModel.sendOnChainState.collectAsState()
            val lspInfoState = viewModel.lspInfoState.collectAsState()
            val lnUrlPayState = viewModel.lnUrlPayState.collectAsState()
            val lnUrlWithdrawState = viewModel.lnUrlWithdrawState.collectAsState()
            val lnUrlAuthState = viewModel.lnUrlAuthState.collectAsState()

            BreezSdkSampleTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    when (val state = homeState.value) {

                        HomeState.Loading -> HomeLoading()

                        is HomeState.Error -> HomeError(
                            throwable = state.throwable,
                        )

                        is HomeState.Success -> Home(
                            onchainBalance = state.onchainBalance,
                            lightningBalance = state.lightningBalance,
                            inProgressSwapOuts = state.inProgressSwapOuts,
                            fiatCurrencies = state.fiatCurrencies,
                            receivePaymentComposable = {
                                when (val paymentState = invoiceState.value) {
                                    is ReceiveState.Success -> ReceivePayment(
                                        paymentState.invoice,
                                        paymentState.openingChannelFee,
                                    )
                                    is ReceiveState.Error -> SectionError(
                                        paymentState.throwable,
                                    )
                                    ReceiveState.Loading -> SectionLoading()
                                    ReceiveState.Initial -> ReceivePaymentAction(
                                        ::onReceivePaymentClick,
                                    )
                                }
                            },
                            sendPaymentComposable = {
                                when (val paymentState = sendPaymentState.value) {
                                    is SendPaymentState.Success -> SendPayment(
                                        paymentState.payment,
                                    )
                                    is SendPaymentState.Error -> SectionError(
                                        paymentState.throwable,
                                    )
                                    SendPaymentState.Loading -> SectionLoading()
                                    SendPaymentState.Initial -> SendPaymentAction(
                                        ::onSendPaymentClick,
                                    )
                                }
                            },
                            spontaneousComposable = {
                                when (val paymentState = spontaneousState.value) {
                                    is SpontaneousState.Success -> SpontaneousPayment(
                                        paymentState.payment,
                                    )
                                    is SpontaneousState.Error -> SectionError(
                                        paymentState.throwable,
                                    )
                                    SpontaneousState.Loading -> SectionLoading()
                                    SpontaneousState.Initial -> SpontaneousPaymentAction(
                                        ::onSendSpontaneousPaymentClick,
                                    )
                                }
                            },
                            receiveOnChainComposable = {
                                when (val onChainState = receiveOnChainState.value) {
                                    is ReceiveOnChainState.Success -> ReceiveOnChain(
                                        onChainState.swapInfo,
                                    )
                                    is ReceiveOnChainState.Error -> SectionError(
                                        onChainState.throwable,
                                    )
                                    ReceiveOnChainState.Loading -> SectionLoading()
                                    ReceiveOnChainState.Initial -> ReceiveOnChainAction(
                                        ::onReceiveOnChainClick,
                                    )
                                }
                            },
                            sendOnChainComposable = {
                                when (val onChainState = sendOnChainState.value) {
                                    is SendOnChainState.Success -> SendOnChain(
                                        onChainState.swapInfo,
                                    )
                                    is SendOnChainState.Error -> SectionError(
                                        onChainState.throwable,
                                    )
                                    SendOnChainState.Loading -> SectionLoading()
                                    SendOnChainState.Initial -> SendOnChainAction(
                                        ::onSendOnChainClick,
                                    )
                                }
                            },
                            lspInfoComposable = {
                                when (val lspInfo = lspInfoState.value) {
                                    is LspInfoState.Success -> LspInfo(
                                        lspInfo.lspInfo,
                                    )
                                    is LspInfoState.Error -> SectionError(
                                        lspInfo.throwable,
                                    )
                                    LspInfoState.Loading -> SectionLoading()
                                    LspInfoState.Initial -> LspInfoAction(
                                        ::onLspInfoClick,
                                    )
                                }
                            },
                            lnUrlPayComposable = {
                                when (val lnUrlPay = lnUrlPayState.value) {
                                    is LnUrlPayState.Success -> LnUrlPay(
                                        lnUrlPay.result,
                                    )
                                    is LnUrlPayState.Error -> SectionError(
                                        lnUrlPay.throwable,
                                    )
                                    LnUrlPayState.Loading -> SectionLoading()
                                    LnUrlPayState.Initial -> LnUrlPayAction(
                                        ::onLnUrlPayAction,
                                    )
                                }
                            },
                            lnUrlWithdrawComposable = {
                                when (val lnUrlWithdraw = lnUrlWithdrawState.value) {
                                    is LnUrlWithdrawState.Success -> LnUrlWithdraw()
                                    is LnUrlWithdrawState.Error -> SectionError(
                                        lnUrlWithdraw.throwable,
                                    )
                                    LnUrlWithdrawState.Loading -> SectionLoading()
                                    LnUrlWithdrawState.Initial -> LnUrlWithdrawAction(
                                        ::onLnUrlWithdrawAction,
                                    )
                                }
                            },
                            lnUrlAuthComposable = {
                                when (val lnUrlAuth = lnUrlAuthState.value) {
                                    is LnUrlAuthState.Success -> LnUrlAuth()
                                    is LnUrlAuthState.Error -> SectionError(
                                        lnUrlAuth.throwable,
                                    )
                                    LnUrlAuthState.Loading -> SectionLoading()
                                    LnUrlAuthState.Initial -> LnUrlAuthAction(
                                        ::onLnUrlAuthAction,
                                    )
                                }
                            }
                        )

                    }
                }
            }
        }
    }

    private fun onReceivePaymentClick(amount: Long, description: String) {
        viewModel.receiveLightningPayment(amount, description)
    }

    private fun onSendPaymentClick(bolt11: String, amount: Long) {
        viewModel.sendLightningPayment(bolt11, amount)
    }

    private fun onSendSpontaneousPaymentClick(nodeId: String, amount: Long) {
        viewModel.sendSpontaneousPayment(nodeId, amount)
    }

    private fun onReceiveOnChainClick() {
        viewModel.receiveOnChain()
    }

    private fun onSendOnChainClick(amount: Long, address: String) {
        viewModel.sendOnChain(amount, address)
    }

    private fun onLspInfoClick() {
        viewModel.lspInfo()
    }

    private fun onLnUrlPayAction(
        lnUrlPayUrl: String,
        amount: Long,
        comment: String,
    ) {
        viewModel.lnUrlPay(lnUrlPayUrl, amount, comment)
    }

    private fun onLnUrlWithdrawAction(
        lnUrlWithdrawUrl: String,
        amount: Long,
        comment: String,
    ) {
        viewModel.lnUrlWithdraw(lnUrlWithdrawUrl, amount, comment)
    }

    private fun onLnUrlAuthAction(
        lnUrlAuthUrl: String,
    ) {
        viewModel.lnUrlAuth(lnUrlAuthUrl)
    }

}
