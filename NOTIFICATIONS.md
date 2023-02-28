# Breez SDK Notifications
Applications have their own way of delivering notifications to end users such as: mobile push, web push, emails, text messages etc...
This is usually implemented using some backend service that has a way to communicate with the end user. The SDK goal is to provide the trigger including the type and notification data to that service. A simple and standard option to provide that trigger is using webhooks.

## The use cases:
1. User app registered for transaction confirmation for example to be notified when a swap on-chain payment is confirmed.
2. User is offline and would like to be notified when receiving a payment.

## Use cases

### Offline payment
User is offline and would like to be notified to accept a received a payment.
* The client creates and invoice, register the payment with the LSP passing also the notification web hook: https://breez-notifications.breez.technology?type=apn&token=push_device_token
* The sender pays, htlcs land at the LSP which detects the last hop is not connected as a peer and send a POST request to the associated webhook with the payload that includes the notification type and data.
* The web hook land on the delivery services that construct the specific notification format and payload and send it to the device. In that case the push message can be silent, waking up the app to run the signer and process the payment.

### Receive on-chain
User is offline and would like to be notified when receiving a payment.
* The client initiate a swap and send funds to the on-chain address.
* The client registered to a push notification for the transaction using web hook: https://breez-notifications.breez.technology?type=apn&token=push_device_token
* Once the transaction is confirmed the notification server send a POST request to the associated webhook with the payload that includes the notification type and data.
* The web hook land on the delivery services that construct the specific notification format and payload and send it to the device. In that case the push message can be silent, waking up the app to run the signer and complete the swap payment.

