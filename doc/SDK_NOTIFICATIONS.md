# Breez SDK Notifications
Applications have their own way of delivering notifications to end users such as: mobile push, web push, emails, text messages etc...
This is usually implemented using some backend service that has a way to communicate with the end user. The SDK goal is to provide the trigger including the type and notification data to that service. A simple and standard option to implement that trigger is using webhooks.

## The use cases:

### Offline payment
User is offline and would like to be notified to accept a received payment.
- The client associate its node id with the webhook by calling a dedicated API endpoint of the LSP.
- The sender pays, htlcs land at the LSP which detects the last hop is not connected as a peer and send a POST request to the associated webhook with the payload that includes the notification type and data.
- The web hook notifies the delivery service that builds the specific notification format and payload and send it to the device. In that case the push message can be silent, waking up the app to run the signer and process the payment.

### Receive on-chain
User is offline and would like to be notified when the transaction used to send funds to an on-chain swap address is confirmed.
- The client initiate a swap and send funds to the on-chain address.
- The client registers to a push notification for the address using web hook.
- Once the transaction is confirmed the notification server send a POST request to the associated webhook with the payload that includes the notification type and data.
- The web hook notifies the delivery services that construct the specific notification format and payload and send it to the device. In that case the push message can be silent, waking up the app to run the signer and complete the swap payment.

## Suggested implementation
There are several parts for achieving the above:

### SDK
- `register_notifications(url, platform, token)` API method for the user to register all kind of notifications.
- Build the webhook URL (defined later in the document) and use it in the following cases:
 - When creating an on-chain receive swap address - Use breez server API to register for notification of any tx associated with this address
 - Use LSP API to register webhook to be notified when there is an incoming payment(htlc) for the user node.

### LSP
- `subscribe_notifications(url, signature)` API for registering a webhook by sdk clients. The signature is the url signed by the node id. The LSP should maintain a mapping between node ids to web hooks URLs. These webhooks (from the client perspective) tend to change frequently as for example in push notifications a new token is generated from time to time so the client is responsible to update the url on any change.

When an htlc is intercepted in the LSP and the destination peer is not online the LSP will notify the web hook with the right type and data, signaling incoming payment, in attempt to wake up the destination node, wait for a some time (how much?) and once the peer is online will forward the htlcs.

### Chain notification service
Breez server will implement the chain notifications for sdk clients. Connected to bitcoind breez server download every block, check for webhook registration and trigger notifications. API key is needed to subscribe for such notification.
The following endpoints are needed:
- `registration_id add_registration(current_registration_id, url)` - get the current registration_id if exists or create one if the former was deleted. Also associate the url with the new registration if is not already associated. At the first time this endpoint is called the `current_registration_id` can be null. This mechanism allows the service to delete expired registrations according to its own policy requiring the client to refresh the registration from time to time signaling it is truly active.
- `subscribe_address_transactions(registration_id, url, address)` API for subscribing to address notifications.
- `subscribe_tx_confirmation(registration_id, url, txid)` API for subscribing to specific transaction notifications.
The `registration_id` is a random unique identifier that identify the caller and allow the sdk client to both update the hook url and subscribe to notification services.

### Notification Delivery Service (NDS)
This service is the destination of the web hooks. Its job is to identify the protocol, type and data of the notification, craft the formatted notification and send it over the right channel (mobile push for example)
Such service should be operated by an app provider that would like to send push notifications to users.

## Webhook structure
A webhook is just a url that is used for a POST request when the notification is needed.
This URL should have enough information for the NDS to know where to send the notification.
We don't impose any structure or specify any required parameters for this URL.

### Example - mobile push
Let's assume the SDK is used within a mobile app and the user wants to be notified when a payment is pending. For the NDS to identify the device a push `token` is used and it is possible also to add the `platform` that generated the token (android, ios), so one option is to use the following structure:

`<base_url>?platform=<platform>&token=<token>`

Where:
- `platform` is one of: android, ios
- `token` is the token given by the client OS.
- `base_url` points to the NDS


## Webhook payload

Triggering the webhook is done by initiating a POST request with the following json payload:

{
 "type": <hook_type>,
 "data": <hook_data>
}

Every kind of hook `type` has its own optional `data` structure.
For example a payload for a hook of incoming payment may look like this:
{
 "type": "payment_received",
 "data": {
  "destination_node_id": <destination>
  "payment_hash" : <payment hash>
 }
}

Or for a tranansaction confirmation related to some address:
{
 "type": "tx_confirmed"
 "data": {
  "address": <btc address>
  "txid": <transaction_id>
  "incoming": true
 }
}