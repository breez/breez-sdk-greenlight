package sample.sdk.breez.page.home.lspinfo

import breez_sdk.LspInformation

sealed class LspInfoState {

    object Initial : LspInfoState()

    object Loading : LspInfoState()

    data class Error(
        val throwable: Throwable,
    ) : LspInfoState()

    data class Success(
        val lspInfo: LspInformation,
    ) : LspInfoState()

}
