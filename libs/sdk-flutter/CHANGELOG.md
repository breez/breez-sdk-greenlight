SDK release notes can be found at [breez-sdk/releases](https://github.com/breez/breez-sdk/releases/)
## 0.3.2
* Fixed a signer crash

## 0.3.1
* Support notifications via a webhook after a swap transaction confirms.
* Reduced package size by not bundling pre-built binaries

## 0.3.0
* Fixes compatibility issues ith `bdk-flutter` plugin.
* Introduce `rescanSwap` API to rescan swap addresses.
* Introduce `configureNode` API to configure an address to send funds to during a mutual channel close.
* Introduce `setPaymentMetadata` API to set the external metadata of a payment as a valid JSON string.
* Add optional `chainnotifierUrl` to `Config`.
* Include `openChannelBolt11`, `lnurlPayDomain`, `reverseSwapInfo` in `LnPaymentDetails`.  
  `openChannelBolt11` for received payments which required to open a channel.  
  `lnurlPayDomain` for sent payments that are not to a Lightning Address.  
  `reverseSwapInfo` for payments that were sent in the context of a reverse swap.

## 0.2.15
* This is a hotfix release that fixes a critical issue from previous release.

## 0.2.14 (Please use >=0.2.15)
* **Breaking** Rename `sweep` to `redeemOnchainFunds`.
* * Updates `flutter_rust_bridge` to `v1.82.6`.
* Introduce `registerWebhook` API to receive payments via mobile notifications. More information [here](https://sdk-doc.breez.technology/guide/payment_notification.html).
* Allow `RegisterWebhook` command to be executed through `executeCommand` API.
* Add expiry time to pending payments.  
  Add optional `pendingExpirationBlock` to `LnPaymentDetails`.
* Add extra TLVs to send spontaneous payment.  
  Add optional `extraTlvs` to `SendSpontaneousPaymentRequest`.
* Support custom payment metadata.  
  Add optional `metadataFilters` to `ListPaymentsRequest`.

## 0.2.12
* Allow native access to SDK from flutter (Kotlin & Swift).
* Updates `flutter_rust_bridge` to `v1.82.4`.

## 0.2.10
* **Breaking** Replace parameters of `prepareSweep` with `PrepareSweepRequest`.
* Amount is now populated in failed payments.
* Introduce `reportIssue` API to report payment failures.
* Introduce `serviceHealthCheck` API to get service health status.
* Include `Payment` information on `InvoicePaid` event.

## 0.2.9
* Requires Dart 3.0 or later.
* Migrate to null safety.
* **Breaking** `filter` field of `ListPaymentsRequest` is deprecated and must be replaced with the optional `filters` field.
* **Breaking** `PaymentTypeFilter.All` is removed. Unfiltered payment list will be retrieved if `filters` fields of `ListPaymentsRequest` is left empty(_or is a list that contains all `PaymentTypeFilter` types_).
* Introduce `prepareRefund` API to estimate the refund transaction fee.
* Introduce `prepareSweep` API to estimate the sweep transaction fee.
* Introduce `maxReverseSwapAmount` API to allow draining all channels when sending on-chain.
* `ClosedChannel` transactions can now be filtered by adding `PaymentTypeFilter.ClosedChannels` to the `filters` list.
* Include `swapInfo` in `Payment`.
* Include `paymentHash` in `LnUrlPayResult`.

## 0.2.7
* **Breaking** All APIs which previously allowed multiple parameters to be
  passed now require their corresponding `Request` object.  
  These API's include: `sendOnchain`, `sendPayment`, `sendSpontaneousPayment`, `refund`, `lnurlPay`, `lnurlWithdraw`.
* **Breaking** All `request` params is renamed to `req`.
* **Breaking** All `reqData` params that belong to a `req` object(_lnurlPay, lnurlWithdraw except lnurlAuth_) is renamed to `data`.
* **Breaking** Use millisatoshi instead of satoshi for lightning amounts.  
  `ReceivePaymentRequest`, `SendPaymentRequest`, `SendSpontaneousPaymentRequest` now use `amount_msat` instead of `amount_sat`.
* Support pagination in `listPayments`.
* Add optional `claimTxid` and `lockTxid` to `ReverseSwapInfo`.
* Add `closingTxid` to closed channels received in payments list.
* Allow `GetInfo` command to be executed through `executeCommand` API.