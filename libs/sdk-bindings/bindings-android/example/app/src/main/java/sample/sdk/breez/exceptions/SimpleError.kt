package sample.sdk.breez.exceptions

data class SimpleError(
    override val message: String,
) : Throwable(message)
