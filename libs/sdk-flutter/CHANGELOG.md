SDK release notes can be found at [breez-sdk/releases](https://github.com/breez/breez-sdk/releases/)

## 0.2.8
* Requires Dart 3.0 or later.
* Migrate to null safety.
* Introduce `prepareRefund` API
* **Breaking** `filter` field of `ListPaymentsRequest` is deprecated and must be replaced with the optional `filters` field.
* **Breaking** `PaymentTypeFilter.All` is removed. Unfiltered payment list will be retrieved if `filters` fields of `ListPaymentsRequest` is left empty(_or is a list that contains all PaymentTypeFilter types_).
* `ClosedChannel` transactions can now be filtered by adding `PaymentTypeFilter.ClosedChannels` to the `filters` list.

## 0.2.7
* **Breaking** All APIs which previously allowed multiple parameters to be
  passed now require their corresponding `Request` object.
  These API's include: `sendOnchain`, `sendPayment`, `sendSpontaneousPayment`, `refund`, `lnurlPay`, `lnurlWithdraw`
* **Breaking** All `request` params is renamed to `req`
* **Breaking** All `reqData` params that belong to a `req` object(_lnurlPay,lnurlWithdraw except lnurlAuth_) is renamed to `data`